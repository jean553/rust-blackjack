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
    Arc,
    mpsc,
};

use crate::socket_message::SocketMessage;
use crate::message_action::MessageAction;
use crate::event::Event;

pub struct Client {
    pub cards_mutex_arc: Arc<Mutex<Vec<u16>>>,
    pub bank_cards_mutex_arc: Arc<Mutex<Vec<u16>>>,
    pub hand_points_arc: Arc<Mutex<u8>>,
    pub cards_amount_arc: Arc<Mutex<u16>>,
    pub socket_sender: Sender,
    pub channel_sender: mpsc::Sender<Event>,
}

impl Handler for Client {

    /// Called when a successful connexion has been established with the server,
    /// sends a successful connection event to the main thread through the channel.
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

        if data.action == MessageAction::SendPlayerCard {

            let mut displayed_cards = self.cards_mutex_arc.lock()
                .unwrap();
            displayed_cards.push(data.card_index);

            let mut remaining_cards_amount = self.cards_amount_arc.lock()
                .unwrap();
            *remaining_cards_amount = data.cards_amount;

            let mut hand_points = self.hand_points_arc.lock().
                unwrap();
            *hand_points = data.player_handpoints;

            return Ok(());
        }

        if data.action == MessageAction::SendBankCard {

            let mut bank_cards = self.bank_cards_mutex_arc.lock()
                .unwrap();
            bank_cards.push(data.card_index);
        }

        Ok(())
    }

    /// Called when the server closes the connection. Sends a message to the main thread in order to stop the program.
    fn on_close(&mut self, _: CloseCode, _: &str) {

        self.channel_sender.send(Event::Disconnect).unwrap();
    }
}

