extern crate piston_window;
extern crate ws;

use piston_window::{
    PistonWindow,
    WindowSettings,
    clear,
};

use ws::{
    connect,
    CloseCode,
    Sender,
    Handler,
    Message,
    Result,
};

use std::io::stdin;
use std::thread;

struct Client {
    output: Sender,
}

impl Handler for Client {

    ///
    ///
    ///
    fn on_message(&mut self, message: Message) -> Result<()> {
        println!("Message received.");
        self.output.send("OK")
    }
}

fn main() {

    println!("Player name: ");
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Input error.");

    let socket_thread = thread::spawn(move || {

        const SERVER_ADDRESS: &str = "ws://127.0.0.1:3000";

        connect(SERVER_ADDRESS, |output| {

            let player_name = input.trim().to_string();
            output.send(player_name);

            move |message| {
                output.close(CloseCode::Normal)
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
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, window| {

                clear(
                    [0.2, 0.5, 0.3, 1.0], /* green */
                    window,
                );
            }
        );
    }

    socket_thread.join();
}
