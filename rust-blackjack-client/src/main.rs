#![deny(warnings)]

extern crate piston_window;
extern crate ws;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod cards;

use piston_window::{
    clear,
    image,
    text,
    PistonWindow,
    WindowSettings,
    Transformed,
    PressEvent,
    Button,
    Key,
    Glyphs,
    TextureSettings,
};

use ws::{
    connect,
    Handler,
    Result,
    Handshake,
    Message,
    Sender,
    CloseCode,
};

use std::io::stdin;
use std::thread;
use std::sync::{
    Mutex,
    Arc,
};
use std::sync::mpsc;

#[derive(PartialEq)]
enum Event {
    Connect(Sender),
    Disconnect,
}

#[derive(Serialize, Deserialize, PartialEq)]
enum CardAction {
    SendCard,
    Hit,
}

#[derive(Serialize, Deserialize)]
struct CardMessage {
    action: CardAction,
    card_index: u8,
}

struct Client {
    card_mutex_arc: Arc<Mutex<Vec<u8>>>,
    socket_sender: Sender,
    channel_sender: mpsc::Sender<Event>,
}

impl Handler for Client {

    /// Called when a successful connexion has been established with the server,
    /// sends a successful connection event to the main thread through the channel.
    fn on_open(
        &mut self,
        _: Handshake
    ) -> Result<()> {

        println!("Connected.");

        self.channel_sender.send(
            Event::Connect(self.socket_sender.clone())
        ).unwrap();

        Ok(())
    }

    /// Called when a message is received from the server.
    ///
    /// # Args:
    ///
    /// `message` - the received message
    fn on_message(
        &mut self,
        message: Message,
    ) -> Result<()> {

        let data: CardMessage = serde_json::from_str(
            &message.into_text()
                .unwrap()
        ).unwrap();

        if data.action == CardAction::SendCard {

            let mut displayed_cards = self.card_mutex_arc.lock()
                .unwrap();

            displayed_cards.push(data.card_index);
        }

        Ok(())
    }

    /// Called when the server closes the connection. Sends a message to the main thread in order to stop the program.
    fn on_close(&mut self, _: CloseCode, _: &str) {

        self.channel_sender.send(Event::Disconnect).unwrap();
    }
}

fn main() {

    let card_mutex_arc = Arc::new(Mutex::new(vec![]));
    let card_mutex_arc_clone = card_mutex_arc.clone();

    println!("Player name: ");
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Input error.");

    let (
        channel_sender,
        channel_receiver,
    ) = mpsc::channel::<Event>();

    /* the socket handling is performed into a dedicated thread,
     * otherwise the program would just block here waiting for messages */
    thread::spawn(move || {

        const SERVER_ADDRESS: &str = "ws://127.0.0.1:3000";
        let _ = connect(SERVER_ADDRESS, |sender| {
            Client {
                card_mutex_arc: card_mutex_arc_clone.clone(),
                socket_sender: sender,
                channel_sender: channel_sender.clone(),
            }
        });
    });

    /* the program halts here until a concrete
       connection attempt status is established */
    let channel_message = channel_receiver.recv().unwrap();

    let sender = match channel_message {
        Event::Connect(s) => s,
        _ => {
            panic!("Unexpected channel message.");
        }
    };

    const WINDOW_WIDTH: f64 = 800.0;
    const WINDOW_HEIGHT: f64 = 600.0;

    let mut window: PistonWindow = WindowSettings::new(
        "rust-blackjack",
        [
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ]
    )
    .fullscreen(false)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let cards_images = cards::load_all_cards_textures(&mut window);

    const TITLE_FONT_PATH: &str = "res/title_font.ttf";

    let mut title_glyphs = Glyphs::new(
        TITLE_FONT_PATH,
        window.factory.clone(),
        TextureSettings::new()
    ).unwrap();

    while let Some(event) = window.next() {

        if Ok(Event::Disconnect) == channel_receiver.try_recv() {
            break;
        }

        if let Some(Button::Keyboard(Key::Return)) = event.press_args() {

            let hit_message = CardMessage {
                action: CardAction::Hit,
                card_index: 0,
            };

            let message = serde_json::to_string(&hit_message).unwrap();

            sender.send(message).unwrap();
        }

        window.draw_2d(
            &event,
            |context, window| {

                clear(
                    [0.2, 0.5, 0.3, 1.0], /* green */
                    window,
                );

                const TITLE_FONT_SIZE: u32 = 64;
                const TITLE_HORIZONTAL_POSITION: f64 = 275.0;
                const TITLE_VERTICAL_POSITION: f64 = 80.0;

                text::Text::new_color(
                    [1.0, 1.0, 1.0, 1.0], /* white */
                    TITLE_FONT_SIZE
                ).draw(
                    "Blackjack",
                    &mut title_glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        TITLE_HORIZONTAL_POSITION,
                        TITLE_VERTICAL_POSITION,
                    ),
                    window,
                ).unwrap();

                let displayed_cards = card_mutex_arc.lock().unwrap();
                if displayed_cards.is_empty() {
                    return;
                }

                const CARD_HORIZONTAL_POSITION: f64 = 300.0;
                const CARD_VERTICAL_POSITION: f64 = 400.0;
                const CARD_DIMENSIONS_SCALE: f64 = 0.5;

                let mut card_horizontal_position: f64 = CARD_HORIZONTAL_POSITION;
                let mut card_vertical_position: f64 = CARD_VERTICAL_POSITION;

                for card_index in 0..displayed_cards.len() {

                    image(
                        &cards_images[
                            *displayed_cards.get(card_index)
                                .unwrap() as usize
                        ],
                        context.transform.trans(
                            card_horizontal_position,
                            card_vertical_position,
                        ).scale(
                            CARD_DIMENSIONS_SCALE,
                            CARD_DIMENSIONS_SCALE
                        ),
                        window,
                    );

                    const CARDS_DISTANCE: f64 = 40.0;
                    card_horizontal_position += CARDS_DISTANCE;
                    card_vertical_position += CARDS_DISTANCE;
                }
            }
        );
    }

    /* the socket thread is terminated here as well,
       after the window has been closed by the user,
       we voluntarily dont wait for it to be terminated (no join) */
}
