//! Handles the web socket client.

use ws::{
    Handler,
    Result,
    Handshake,
    Message,
    Sender,
    CloseCode,
};

use std::sync::{
    Mutex,
    MutexGuard,
    Arc,
    mpsc,
};

use crate::socket_message::SocketMessage;
use crate::message_action::MessageAction;
use crate::event::Event;

pub struct Client {
    pub player_cards_mutex_arc: Arc<Mutex<Vec<u16>>>,
    pub bank_cards_mutex_arc: Arc<Mutex<Vec<u16>>>,
    pub player_points_mutex_arc: Arc<Mutex<u8>>,
    pub bank_points_mutex_arc: Arc<Mutex<u8>>,
    pub cards_amount_arc: Arc<Mutex<u16>>,
    pub displayed_bank_cards_amount_mutex_arc: Arc<Mutex<usize>>,
    pub socket_sender: Sender,
    pub channel_sender: mpsc::Sender<Event>,
}

impl Handler for Client {

    /// Called when a successful connexion has been established with the server,
    /// sends a successful connection event to the main thread through the channel;
    /// sending that first message to the main thread unlocks it and starts rendering the window.
    fn on_open(
        &mut self,
        _: Handshake
    ) -> Result<()> {

        println!("Connected.");

        self.channel_sender.send(
            Event::Connect(self.socket_sender.clone())
        ).unwrap();

        Ok(())
    }

    /// Called when a message is received from the server.
    /// Stores the message into a socket message object
    /// and modifies the client according to the message action.
    ///
    /// # Args:
    ///
    /// `message` - the received message
    fn on_message(
        &mut self,
        message: Message,
    ) -> Result<()> {

        let text_message: &str = &message.into_text().unwrap();
        let data: SocketMessage = serde_json::from_str(text_message).unwrap();

        if data.action == MessageAction::SendPlayerCard {

            let mut displayed_cards: MutexGuard<Vec<u16>> =
                self.player_cards_mutex_arc.lock().unwrap();
            displayed_cards.push(data.card_index);

            let mut remaining_cards_amount: MutexGuard<u16> =
                self.cards_amount_arc.lock().unwrap();
            *remaining_cards_amount = data.cards_amount;

            let mut player_points: MutexGuard<u8> =
                self.player_points_mutex_arc.lock().unwrap();
            *player_points = data.player_handpoints;

            return Ok(());
        }

        if data.action == MessageAction::SendBankCard {

            let mut bank_cards: MutexGuard<Vec<u16>> =
                self.bank_cards_mutex_arc.lock().unwrap();
            bank_cards.push(data.card_index);

            let mut bank_points: MutexGuard<u8> =
                self.bank_points_mutex_arc.lock().unwrap();
            *bank_points = data.player_handpoints;
        }

        if data.action == MessageAction::SendBankCards {

            let mut bank_cards: MutexGuard<Vec<u16>> =
                self.bank_cards_mutex_arc.lock().unwrap();
            *bank_cards = data.bank_cards;

            let mut bank_points: MutexGuard<u8> =
                self.bank_points_mutex_arc.lock().unwrap();
            *bank_points = data.player_handpoints;

            let mut displayed_bank_cards_amount: MutexGuard<usize> =
                self.displayed_bank_cards_amount_mutex_arc.lock().unwrap();
            const DISPLAYED_BANK_CARDS_AMOUNT_AFTER_DRAWING: usize = 2;
            *displayed_bank_cards_amount = DISPLAYED_BANK_CARDS_AMOUNT_AFTER_DRAWING;
        }

        Ok(())
    }

    /// Called when the server closes the connection.
    /// Sends a message to the main thread in order to stop the program.
    fn on_close(
        &mut self,
        _: CloseCode,
        _: &str
    ) {
        self.channel_sender.send(Event::Disconnect).unwrap();
    }
}

