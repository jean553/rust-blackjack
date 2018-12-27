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
    players_handpoints: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct SocketMessage {
    action: MessageAction,
    card_index: u16,
    cards_amount: u16,
    text: String,
    player_handpoints: u8,
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
            players_handpoints: vec![],
        }
    }

    /// Sends one random card to the client through the socket.
    fn send_card(&mut self) {

        let card_index = self.cards.pop().unwrap();

        const ONE_SET_CARDS_AMOUNT: u16 = 52;
        let one_set_card_index = (card_index % ONE_SET_CARDS_AMOUNT) as u8;

        const TEN_POINTS_CARDS_START_INDEX: u8 = 32;
        const ACE_CARDS_START_INDEX: u8 = 47;

        /* FIXME: we handle only handpoints of the first player for now,
           of course we should be able to handle all the playing players */
        let player_handpoints = self.players_handpoints.get_mut(0).unwrap();

        if one_set_card_index >= TEN_POINTS_CARDS_START_INDEX &&
            one_set_card_index < ACE_CARDS_START_INDEX {

            const TEN_VALUE_CARDS_POINTS_AMOUNT: u8 = 10;
            *player_handpoints += TEN_VALUE_CARDS_POINTS_AMOUNT;
        }
        else if one_set_card_index >= ACE_CARDS_START_INDEX {

            const ACE_CARDS_FIRST_POINTS_AMOUNT: u8 = 1;
            const ACE_CARDS_SECOND_POINTS_AMOUNT: u8 = 11;
            const MAX_HAND_POINTS_FOR_ACE_CARDS_SECOND_POINTS_AMOUNT: u8 = 11;

            /* FIXME: the player should also be able to
               select an ace value in some situations */

            if *player_handpoints >= MAX_HAND_POINTS_FOR_ACE_CARDS_SECOND_POINTS_AMOUNT {
                *player_handpoints += ACE_CARDS_FIRST_POINTS_AMOUNT;
            }

            *player_handpoints += ACE_CARDS_SECOND_POINTS_AMOUNT;
        } else {

            const CARDS_WITH_SAME_VALUE_BY_COLOR: u8 = 4;
            const MINIMUM_CARD_VALUE: u8 = 2;
            *player_handpoints += one_set_card_index / CARDS_WITH_SAME_VALUE_BY_COLOR
                + MINIMUM_CARD_VALUE;
        }

        let card_message = SocketMessage {
            action: MessageAction::SendCard,
            card_index: card_index,
            cards_amount: self.cards.len() as u16,
            text: "".to_string(),
            player_handpoints: *player_handpoints,
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

        self.players_handpoints.push(0);

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

            println!(
                "Player hand points: {}",
                *self.players_handpoints.get(0).unwrap()
            );

            return Ok(());
        }

        if data.action == MessageAction::NewPlayer {

            println!("Player name: {}", data.text);

            println!(
                "Player hand points: {}",
                *self.players_handpoints.get(0).unwrap()
            );
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
