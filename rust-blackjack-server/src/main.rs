extern crate ws;

use ws::{
    listen,
    Sender,
    Handler,
    Result,
    CloseCode,
    Handshake,
};

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

        /* FIXME: for now, we simply always send
           the same card to a new connected client;
           we should send random cards from a queue */
        const GIVE_CARD: u8 = 0;
        const CLOVER_TWO: u8 = 0;
        self.output.send(vec![GIVE_CARD, CLOVER_TWO])
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
