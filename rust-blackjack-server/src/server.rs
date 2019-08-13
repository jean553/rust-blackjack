//! The main server structure and implementation.

use ws::{
    Sender,
    Handler,
    Result,
    CloseCode,
    Handshake,
    Message,
};

use rand::{thread_rng, Rng};

use crate::socket_message::SocketMessage;
use crate::message_action::MessageAction;

/// Contains the web socket output sender and the cards array.
///
/// NOTE: there are many more optimized ways to store the cards (memory and time complexity), but
/// we voluntarily keep a raw array to store them all in order to create a genuine black-jack game situation
pub struct Server {
    output: Sender,
    cards: Vec<u16>,
    players_handpoints: Vec<u8>,
    bank_handpoints: u8,
    bank_cards: Vec<u16>,
}

/// Return card points according to a given index.
///
/// Args:
///
/// `card_index` - the current card index,
/// `player_handpoints` - the current player handpoints amount,
fn get_card_points(
    card_index: u8,
    player_handpoints: u8,
) -> u8 {

    const TEN_POINTS_CARDS_START_INDEX: u8 = 32;
    const ACE_CARDS_START_INDEX: u8 = 47;

    if card_index >= TEN_POINTS_CARDS_START_INDEX &&
        card_index < ACE_CARDS_START_INDEX {

        const TEN_VALUE_CARDS_POINTS_AMOUNT: u8 = 10;
        return TEN_VALUE_CARDS_POINTS_AMOUNT;
    }
    else if card_index >= ACE_CARDS_START_INDEX {

        const ACE_CARDS_FIRST_POINTS_AMOUNT: u8 = 1;
        const ACE_CARDS_SECOND_POINTS_AMOUNT: u8 = 11;
        const MAX_HAND_POINTS_FOR_ACE_CARDS_SECOND_POINTS_AMOUNT: u8 = 11;

        /* FIXME: the player should also be able to
           select an ace value in some situations */

        if player_handpoints >= MAX_HAND_POINTS_FOR_ACE_CARDS_SECOND_POINTS_AMOUNT {
            return ACE_CARDS_FIRST_POINTS_AMOUNT;
        }

        return ACE_CARDS_SECOND_POINTS_AMOUNT;

    } else {

        const CARDS_WITH_SAME_VALUE_BY_COLOR: u8 = 4;
        const MINIMUM_CARD_VALUE: u8 = 2;
        return card_index / CARDS_WITH_SAME_VALUE_BY_COLOR
            + MINIMUM_CARD_VALUE;
    }
}

impl Server {

    /// Creates a new server.
    ///
    /// # Args:
    ///
    /// `output` - the server ws sender in order to send back information
    pub fn new(output: ws::Sender) -> Server {

        const MIN_CARD_ID: u16 = 0;
        const MAX_CARD_ID: u16 = 416;

        let mut all_cards: Vec<u16> = (MIN_CARD_ID..MAX_CARD_ID).collect();
        let cards: &mut [u16] = &mut all_cards;
        thread_rng().shuffle(cards);

        Server {
            output: output,
            cards: all_cards,
            players_handpoints: vec![],
            bank_handpoints: 0,
            bank_cards: vec![],
        }
    }

    /// Sends one random card to the client through the socket.
    fn send_card(&mut self) {

        let card_index = self.cards.pop().unwrap();

        const ONE_SET_CARDS_AMOUNT: u16 = 52;
        let one_set_card_index = (card_index % ONE_SET_CARDS_AMOUNT) as u8;

        /* FIXME: we handle only handpoints of the first player for now,
           of course we should be able to handle all the playing players */
        let player_handpoints = self.players_handpoints.get_mut(0).unwrap();

        let drawn_card_points = get_card_points(
            one_set_card_index,
            *player_handpoints,
        );

        *player_handpoints += drawn_card_points;

        let card_message = SocketMessage {
            action: MessageAction::SendPlayerCard,
            card_index: card_index,
            cards_amount: self.cards.len() as u16,
            text: "".to_string(),
            player_handpoints: *player_handpoints,
            bank_cards: vec![],
        };

        let message = serde_json::to_string(&card_message).unwrap();

        self.output.send(message).unwrap();
    }

    /// Send a card to the bank, and render the cards on the client side.
    fn send_bank_card(&mut self) {

        let card_index = self.cards.pop().unwrap();

        self.bank_cards.push(card_index);

        const ONE_SET_CARDS_AMOUNT: u16 = 52;
        let one_set_card_index = (card_index % ONE_SET_CARDS_AMOUNT) as u8;

        self.bank_handpoints = get_card_points(
            one_set_card_index,
            0
        );

        let card_message = SocketMessage {
            action: MessageAction::SendBankCard,
            card_index: card_index,
            cards_amount: self.cards.len() as u16,
            text: "".to_string(),
            player_handpoints: self.bank_handpoints,
            bank_cards: vec![],
        };

        let message = serde_json::to_string(&card_message).unwrap();

        self.output.send(message).unwrap();
    }

    /// TODO
    fn draw_bank_cards(&mut self) {

        const MAX_HAND_POINTS: u8 = 17;
        while self.bank_handpoints < MAX_HAND_POINTS {

            let card_index = self.cards.pop().unwrap();

            self.bank_cards.push(card_index);

            const ONE_SET_CARDS_AMOUNT: u16 = 52;
            let one_set_card_index = (card_index % ONE_SET_CARDS_AMOUNT) as u8;

            let card_points = get_card_points(
                one_set_card_index,
                self.bank_handpoints,
            );

            self.bank_handpoints += card_points;
        }
    }

    /// TODO
    fn send_bank_cards(&self) {

        let cards_message = SocketMessage {
            action: MessageAction::SendBankCards,
            card_index: 0,
            cards_amount: 0,
            text: "".to_string(),
            player_handpoints: self.bank_handpoints,
            bank_cards: self.bank_cards.clone(),
        };

        let message = serde_json::to_string(&cards_message).unwrap();

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
        self.send_bank_card();

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

            let player_handpoints = *self.players_handpoints.get(0).unwrap();

            const MAX_HAND_POINTS: u8 = 21;
            if player_handpoints > MAX_HAND_POINTS {
                let player_handpoints = self.players_handpoints.get_mut(0).unwrap();
                *player_handpoints = 0;
                self.send_card();
            }

            self.send_card();

            if player_handpoints > MAX_HAND_POINTS {
                self.send_bank_card();
            }

            println!(
                "Player hand points: {}",
                *self.players_handpoints.get(0).unwrap()
            );

            return Ok(());
        }

        if data.action == MessageAction::Stand ||
            data.action == MessageAction::Continue {

            self.draw_bank_cards();
            self.send_bank_cards();

            return Ok(());
        }

        if data.action == MessageAction::Restart {

            let player_handpoints = self.players_handpoints.get_mut(0).unwrap();
            *player_handpoints = 0;
            self.bank_handpoints = 0;

            self.bank_cards.clear();
            self.send_card();
            self.send_card();
            self.send_bank_card();

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
