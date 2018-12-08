extern crate piston_window;
extern crate ws;

use piston_window::{
    PistonWindow,
    WindowSettings,
    clear,
};

use ws::{
    connect,
    Sender,
    Handler,
    Result,
    Handshake,
};

use std::io::stdin;
use std::thread;

struct Client {
    output: Sender,
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
}

fn main() {

    println!("Player name: ");
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Input error.");

    /* the socket handling is performed into a dedicated thread,
     * otherwise the program would just block here waiting for messages */
    thread::spawn(move || {

        const SERVER_ADDRESS: &str = "ws://127.0.0.1:3000";
        let _ = connect(SERVER_ADDRESS, |output| {
            Client { output }
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
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |_context, window| {

                clear(
                    [0.2, 0.5, 0.3, 1.0], /* green */
                    window,
                );
            }
        );
    }

    /* the socket thread is terminated here as well,
       after the window has been closed by the user,
       we voluntarily dont wait for it to be terminated (no join) */
}
