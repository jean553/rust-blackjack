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
enum MessageAction {
    NewPlayer,
    SendCard,
    Hit,
}

#[derive(Serialize, Deserialize)]
struct SocketMessage {
    action: MessageAction,
    card_index: u8,
    text: String,
}

struct Client {
    cards_mutex_arc: Arc<Mutex<Vec<u8>>>,
    hand_points_arc: Arc<Mutex<u8>>,
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

        let data: SocketMessage = serde_json::from_str(
            &message.into_text()
                .unwrap()
        ).unwrap();

        if data.action == MessageAction::SendCard {

            let mut displayed_cards = self.cards_mutex_arc.lock()
                .unwrap();

            displayed_cards.push(data.card_index);

            let mut hand_points = self.hand_points_arc.lock().
                unwrap();

            let card_index = data.card_index;

            const TEN_POINTS_CARDS_START_INDEX: u8 = 32;
            const ACE_CARDS_START_INDEX: u8 = 47;
            if card_index >= TEN_POINTS_CARDS_START_INDEX &&
                card_index < ACE_CARDS_START_INDEX {

                const TEN_VALUE_CARDS_POINTS_AMOUNT: u8 = 10;
                *hand_points += TEN_VALUE_CARDS_POINTS_AMOUNT;

                return Ok(());
            }

            if card_index >= ACE_CARDS_START_INDEX {

                /* FIXME: ace value should be 1 or 11 */
                const ACE_CARDS_POINTS_AMOUNT: u8 = 11;
                *hand_points += ACE_CARDS_POINTS_AMOUNT;

                return Ok(());
            }

            const CARDS_WITH_SAME_VALUE_BY_COLOR: u8 = 4;
            const MINIMUM_CARD_VALUE: u8 = 2;
            *hand_points += card_index / CARDS_WITH_SAME_VALUE_BY_COLOR
                + MINIMUM_CARD_VALUE;
        }

        Ok(())
    }

    /// Called when the server closes the connection. Sends a message to the main thread in order to stop the program.
    fn on_close(&mut self, _: CloseCode, _: &str) {

        self.channel_sender.send(Event::Disconnect).unwrap();
    }
}

fn main() {

    let cards_mutex_arc = Arc::new(Mutex::new(vec![]));
    let hand_points_arc: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));

    println!("Player name: ");
    let mut player_name: String = String::new();
    stdin().read_line(&mut player_name).expect("Input error.");
    let player_name = player_name.trim().to_string();

    let (
        channel_sender,
        channel_receiver,
    ) = mpsc::channel::<Event>();

    /* the asynchronous reference counters on mutexes
       have to be copied once here as they are moved
       when passed to the client thread below and
       still used by the main thread */
    let cards_mutex_arc_clone = cards_mutex_arc.clone();
    let hand_points_arc_clone = hand_points_arc.clone();

    /* the socket handling is performed into a dedicated thread,
     * otherwise the program would just block here waiting for messages */
    thread::spawn(move || {

        const SERVER_ADDRESS: &str = "ws://127.0.0.1:3000";
        let _ = connect(SERVER_ADDRESS, |sender| {
            Client {
                cards_mutex_arc: cards_mutex_arc_clone.clone(),
                hand_points_arc: hand_points_arc_clone.clone(),
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

    let new_player_message = SocketMessage {
        action: MessageAction::NewPlayer,
        card_index: 0,
        text: player_name.clone(),
    };

    let message = serde_json::to_string(&new_player_message).unwrap();
    sender.send(message).unwrap();

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

    let mut glyphs = Glyphs::new(
        TITLE_FONT_PATH,
        window.factory.clone(),
        TextureSettings::new()
    ).unwrap();

    while let Some(event) = window.next() {

        if Ok(Event::Disconnect) == channel_receiver.try_recv() {
            break;
        }

        if let Some(Button::Keyboard(Key::Return)) = event.press_args() {

            let hit_message = SocketMessage {
                action: MessageAction::Hit,
                card_index: 0,
                text: "".to_string(),
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
                    TITLE_FONT_SIZE,
                ).draw(
                    "Blackjack",
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        TITLE_HORIZONTAL_POSITION,
                        TITLE_VERTICAL_POSITION,
                    ),
                    window,
                ).unwrap();

                const POINTS_FONT_SIZE: u32 = 32;
                const POINTS_HORIZONTAL_POSITION: f64 = 400.0;
                const POINTS_VERTICAL_POSITION: f64 = 400.0;

                let hand_points = hand_points_arc.lock().unwrap();

                text::Text::new_color(
                    [1.0, 1.0, 1.0, 1.0], /* white */
                    POINTS_FONT_SIZE,
                ).draw(
                    &*hand_points.to_string(),
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        POINTS_HORIZONTAL_POSITION,
                        POINTS_VERTICAL_POSITION,
                    ),
                    window,
                ).unwrap();

                const INFO_FONT_SIZE: u32 = 24;
                const HIT_INFO_HORIZONTAL_POSITION: f64 = 600.0;
                const HIT_INFO_VERTICAL_POSITION: f64 = 400.0;

                text::Text::new_color(
                    [1.0, 1.0, 1.0, 1.0], /* white */
                    INFO_FONT_SIZE,
                ).draw(
                    "HIT - press Enter",
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        HIT_INFO_HORIZONTAL_POSITION,
                        HIT_INFO_VERTICAL_POSITION,
                    ),
                    window,
                ).unwrap();

                const PLAYER_NAME_FONT_SIZE: u32 = 16;
                const PLAYER_NAME_HORIZONTAL_POSITION: f64 = 300.0;
                const PLAYER_NAME_VERTICAL_POSITION: f64 = 380.0;

                text::Text::new_color(
                    [1.0, 1.0, 1.0, 1.0], /* white */
                    PLAYER_NAME_FONT_SIZE,
                ).draw(
                    &player_name,
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        PLAYER_NAME_HORIZONTAL_POSITION,
                        PLAYER_NAME_VERTICAL_POSITION,
                    ),
                    window,
                ).unwrap();

                let displayed_cards = cards_mutex_arc.lock().unwrap();
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
