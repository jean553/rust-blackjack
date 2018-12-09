extern crate ws;
extern crate rand;

use ws::{
    listen,
    Sender,
    Handler,
    Result,
    CloseCode,
    Handshake,
};

use rand::{thread_rng, Rng};

struct Server {
    output: Sender,
}

impl Handler for Server {

    /// Called when a new connexion is established from a client.
    ///
    /// # Args:
    ///
    /// `handshake` - client-server handshake properties
    fn on_open(
        &mut self,
        handshake: Handshake
    ) -> Result<()> {

        println!(
            "New connexion from {}.",
            handshake.remote_addr().unwrap().unwrap()
        );

        const GIVE_CARD: u8 = 0;

        /* FIXME: for now, we simply select a random card for a client;
           we should take our cards from a queue */
        const MIN_CARD_ID: u8 = 0;
        const MAX_CARD_ID: u8 = 51;
        let random_card: u8 = thread_rng().gen_range(
            MIN_CARD_ID,
            MAX_CARD_ID + 1
        );

        self.output.send(
            vec![
                GIVE_CARD,
                random_card,
            ]
        )
    }

    /// Called when a connexion is terminated from the client side.
    fn on_close(&mut self, _: CloseCode, _: &str) {
        println!("Terminate socket.");
    }
}

fn main() {

    const LISTENING_ADDRESS: &str = "127.0.0.1:3000";
    listen(LISTENING_ADDRESS, |output| {
        Server { output }
    }).unwrap();
}
