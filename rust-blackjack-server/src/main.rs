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
enum MessageAction {
    NewPlayer,
    SendCard,
    Hit,
}

/// Contains the web socket output sender and the cards array.
///
/// NOTE: there are many more optimized ways to store the cards (memory and time complexity), but
/// we voluntarily keep a raw array to store them all in order to create a genuine black-jack game situation
struct Server {
    output: Sender,
    cards: Vec<u16>,
}

#[derive(Serialize, Deserialize)]
struct SocketMessage {
    action: MessageAction,
    card_index: u16,
    cards_amount: u16,
    text: String,
}

impl Server {

    /// Creates a new server.
    ///
    /// # Args:
    ///
    /// `output` - the server ws sender in order to send back information
    fn new(output: ws::Sender) -> Server {

        const MIN_CARD_ID: u16 = 0;
        const MAX_CARD_ID: u16 = 416;

        let mut all_cards: Vec<u16> = (MIN_CARD_ID..MAX_CARD_ID).collect();
        let cards: &mut [u16] = &mut all_cards;
        thread_rng().shuffle(cards);

        Server {
            output: output,
            cards: all_cards,
        }
    }

    /// Sends one random card to the client through the socket.
    fn send_card(&mut self) {

        let card_message = SocketMessage {
            action: MessageAction::SendCard,
            card_index: self.cards.pop().unwrap(),
            cards_amount: self.cards.len() as u16,
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

        if data.action == MessageAction::Hit {

            self.send_card();

            return Ok(());
        }

        if data.action == MessageAction::NewPlayer {

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
        Server::new(output)
    }).unwrap();
}
