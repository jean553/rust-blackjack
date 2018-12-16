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
    PistonWindow,
    WindowSettings,
    Transformed,
};

use ws::{
    connect,
    Handler,
    Result,
    Handshake,
    Message,
    Sender,
};

use std::io::stdin;
use std::thread;
use std::sync::{
    Mutex,
    Arc,
};
use std::sync::mpsc;

/// TODO: add the Disconnect event to correctly close the ws thread and the rendering loop together
enum Event {
    Connect(Sender),
}

#[derive(Deserialize, PartialEq)]
enum CardAction {
    SendCard,
}

#[derive(Deserialize)]
struct CardMessage {
    action: CardAction,
    card_index: u8,
}

struct Client {

    /* FIXME: add the Sender field,
       required to communicate with the server */

    card_mutex_arc: Arc<Mutex<Vec<u8>>>,
    sender: Sender,
    thread_sender: mpsc::Sender<Event>,
}

impl Handler for Client {

    /// Called when a successful connexion has been established with the server.
    fn on_open(
        &mut self,
        _: Handshake
    ) -> Result<()> {

        println!("Connected.");

        self.thread_sender.send(Event::Connect(self.sender.clone())).unwrap();

        Ok(())
    }

    /// Called when a message is received from the server.
    fn on_message(
        &mut self,
        message: Message,
    ) -> Result<()> {

        let data: CardMessage = serde_json::from_str(
            &message.into_text()
                .unwrap()
        ).unwrap();

        if data.action == CardAction::SendCard {

            let mut displayed_card = self.card_mutex_arc.lock()
                .unwrap();

            displayed_card.push(data.card_index);
        }

        Ok(())
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
                sender: sender,
                thread_sender: channel_sender.clone(),
            }
        });
    });

    /* TODO: the window should be displayed only if the Connect event
       is received from the channel receiver, meaning the socket
       is correctly initialized, otherwise, the program should stop */
    if let Ok(Event::Connect(_sender)) = channel_receiver.recv() {
    }

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

    let cards = cards::load_all_cards_textures(&mut window);

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, window| {

                clear(
                    [0.2, 0.5, 0.3, 1.0], /* green */
                    window,
                );

                let displayed_card = card_mutex_arc.lock().unwrap();
                if !displayed_card.is_empty() {

                    const CARD_HORIZONTAL_POSITION: f64 = 300.0;
                    const CARD_VERTICAL_POSITION: f64 = 400.0;
                    const CARD_DIMENSIONS_SCALE: f64 = 0.5;

                    let mut card_horizontal_position: f64 = CARD_HORIZONTAL_POSITION;
                    let mut card_vertical_position: f64 = CARD_VERTICAL_POSITION;

                    for card_index in 0..displayed_card.len() {

                        image(
                            &cards[*displayed_card.get(card_index).unwrap() as usize],
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
            }
        );
    }

    /* the socket thread is terminated here as well,
       after the window has been closed by the user,
       we voluntarily dont wait for it to be terminated (no join) */
}
