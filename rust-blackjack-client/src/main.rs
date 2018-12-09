extern crate piston_window;
extern crate ws;

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
    Sender,
    Handler,
    Result,
    Handshake,
    Message,
};

use std::io::stdin;
use std::thread;
use std::sync::{
    Mutex,
    Arc,
};

struct Client {
    output: Sender,
    card_mutex_arc: Arc<Mutex<Option<u8>>>,
}

impl Handler for Client {

    /// Called when a successful connexion has been established with the server.
    fn on_open(
        &mut self,
        _: Handshake
    ) -> Result<()> {

        println!("Connected.");
        self.output.send("OK")
    }

    /// Called when a message is received from the server.
    fn on_message(
        &mut self,
        message: Message,
    ) -> Result<()> {

        let data = message.into_data();
        let action = data.get(0).unwrap();

        const RECEIVE_CARD: u8 = 0;

        if *action == RECEIVE_CARD {

            let card = data.get(1).unwrap();

            let mut displayed_card = self.card_mutex_arc.lock().unwrap();
            *displayed_card = Some(*card);
        }

        self.output.send("OK")
    }
}

fn main() {

    /* FIXME: for now, a player only has one unique card
       (in order to develop the feature step by step);
       the player should have a full deck */
    let displayed_card: Option<u8> = None;

    let card_mutex = Mutex::new(displayed_card);
    let card_mutex_arc = Arc::new(card_mutex);
    let card_mutex_arc_clone = card_mutex_arc.clone();

    println!("Player name: ");
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Input error.");

    /* the socket handling is performed into a dedicated thread,
     * otherwise the program would just block here waiting for messages */
    thread::spawn(move || {

        const SERVER_ADDRESS: &str = "ws://127.0.0.1:3000";
        let _ = connect(SERVER_ADDRESS, |output| {
            Client {
                output: output,
                card_mutex_arc: card_mutex_arc_clone.clone(),
            }
        });
    });

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
                if displayed_card.is_some() {

                    const CARD_HORIZONTAL_POSITION: f64 = 300.0;
                    const CARD_VERTICAL_POSITION: f64 = 400.0;
                    const CARD_DIMENSIONS_SCALE: f64 = 0.5;

                    image(
                        &cards[displayed_card.unwrap() as usize],
                        context.transform.trans(
                            CARD_HORIZONTAL_POSITION,
                            CARD_VERTICAL_POSITION,
                        ).scale(
                            CARD_DIMENSIONS_SCALE,
                            CARD_DIMENSIONS_SCALE
                        ),
                        window,
                    );
                }
            }
        );
    }

    /* the socket thread is terminated here as well,
       after the window has been closed by the user,
       we voluntarily dont wait for it to be terminated (no join) */
}
