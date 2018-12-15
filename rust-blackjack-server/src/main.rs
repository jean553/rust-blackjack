extern crate ws;
extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use ws::{
    listen,
    Sender,
    Handler,
    Result,
    CloseCode,
    Handshake,
};

use rand::{thread_rng, Rng};

#[derive(Serialize)]
enum CardAction {
    SendCard,
}

struct Server {
    output: Sender,
}

#[derive(Serialize)]
struct CardMessage {
    action: CardAction,
    card_index: u8,
}

/// Draws a card for a player.
fn draw_card() -> u8 {

    /* FIXME: for now, we simply select a random card for a client;
       we should take our cards from a queue */
    const MIN_CARD_ID: u8 = 0;
    const MAX_CARD_ID: u8 = 51;
    thread_rng().gen_range(
        MIN_CARD_ID,
        MAX_CARD_ID + 1
    )
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

        let card_message = CardMessage {
            action: CardAction::SendCard,
            card_index: draw_card(),
        };

        let message = serde_json::to_string(&card_message).unwrap();

        self.output.send(message).unwrap();

        let card_message = CardMessage {
            action: CardAction::SendCard,
            card_index: draw_card(),
        };

        let message = serde_json::to_string(&card_message).unwrap();

        self.output.send(message)
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
