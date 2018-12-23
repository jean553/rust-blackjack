#![deny(warnings)]

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
    Message,
};

use rand::{thread_rng, Rng};

#[derive(Serialize, Deserialize, PartialEq)]
enum CardAction {
    NewPlayer,
    SendCard,
    Hit,
}

struct Server {
    output: Sender,
}

#[derive(Serialize, Deserialize)]
struct SocketMessage {
    action: CardAction,
    card_index: u8,
    text: String,
}

impl Server {

    /// Sends one random card to the client through the socket.
    ///
    /// FIXME: we randomly select a card for now, should be popped from a queue
    fn send_card(&self) {

        const MIN_CARD_ID: u8 = 0;
        const MAX_CARD_ID: u8 = 51;

        let card_message = SocketMessage {
            action: CardAction::SendCard,
            card_index: thread_rng().gen_range(
                MIN_CARD_ID,
                MAX_CARD_ID + 1
            ),
            text: "".to_string(),
        };

        let message = serde_json::to_string(&card_message).unwrap();

        self.output.send(message).unwrap();
    }
}

impl Handler for Server {

    /// Called when a new connexion is established from a client. Sends two cards to the new connected client.
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

        self.send_card();
        self.send_card();

        Ok(())
    }

    /// Called when a message is received from the client.
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

        if data.action == CardAction::Hit {

            self.send_card();

            return Ok(());
        }

        if data.action == CardAction::NewPlayer {

            println!("Player name: {}", data.text);
        }

        Ok(())
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
