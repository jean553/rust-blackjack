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
    pub basic_strategy_action_mutex_arc: Arc<Mutex<MessageAction>>,
}

/// Returns the a card points amount according to its index.
fn get_card_points(card_index: u16) -> u8 {

    const ONE_SET_CARDS_AMOUNT: u16 = 52;
    let card_index = ( card_index % ONE_SET_CARDS_AMOUNT ) as u8;

    const TEN_POINTS_CARDS_START_INDEX: u8 = 32;
    const ACE_CARDS_START_INDEX: u8 = 48;

    if card_index >= TEN_POINTS_CARDS_START_INDEX &&
        card_index < ACE_CARDS_START_INDEX {

        const TEN_VALUE_CARDS_POINTS_AMOUNT: u8 = 10;
        return TEN_VALUE_CARDS_POINTS_AMOUNT;
    }
    else if card_index >= ACE_CARDS_START_INDEX {

        const ACE_CARDS_POINTS_AMOUNT: u8 = 11;
        return ACE_CARDS_POINTS_AMOUNT;

    }

    const CARDS_WITH_SAME_VALUE_BY_COLOR: u8 = 4;
    const MINIMUM_CARD_VALUE: u8 = 2;
    return card_index / CARDS_WITH_SAME_VALUE_BY_COLOR
        + MINIMUM_CARD_VALUE;
}

/// Indicates the action to follow according to basic strategy rules
///
/// # Args:
///
/// `player_cards` - the current player cards list
/// `bank_cards` - the current bank cards list
fn get_strategic_action(
    player_cards: &Vec<u16>,
    bank_cards: &Vec<u16>,
) -> MessageAction {

    let first_player_card = get_card_points(*player_cards.get(0).unwrap());
    let second_player_card = get_card_points(*player_cards.get(1).unwrap());
    let player_points = first_player_card + second_player_card;
    let bank_card = get_card_points(*bank_cards.get(0).unwrap());

    /* the player got a pair */
    if first_player_card == second_player_card {

        if
            first_player_card == 11 ||
            first_player_card == 8 ||
            first_player_card == 9 && (
                bank_card != 7 ||
                bank_card != 10 ||
                bank_card != 11
            ) ||
            first_player_card == 7 && bank_card <= 7 ||
            first_player_card == 6 && bank_card <= 6 && bank_card != 2 ||
            (
                first_player_card == 3 ||
                first_player_card == 2
            ) && (
                bank_card <= 7 &&
                bank_card >= 4
            )
        {
            return MessageAction::Split;
        }

        if
            first_player_card == 6 && bank_card == 2 ||
            first_player_card == 4 && (
                bank_card == 5 ||
                bank_card == 6
            ) ||
            (
                first_player_card == 3 ||
                first_player_card == 2
            ) && bank_card < 4
        {
            return MessageAction::DoubleDown;
        }
    }

    /* the player does not have a pair */
    if
        player_points >= 17 ||
        (
            player_points <= 16 &&
            player_points >= 13 &&
            bank_card <= 6
        ) ||
        (
            player_points == 12 &&
            bank_card >= 4 &&
            bank_card <= 6
        )
    {
        return MessageAction::Stand;
    }
    else if
        (
            player_points <= 16 &&
            player_points >= 13 &&
            bank_card >= 7
        ) || (
            player_points == 12 &&
            (
                bank_card <= 3 ||
                bank_card >= 7
            )
        ) || (
            player_points == 10 &&
            bank_card >= 10
        ) || (
            player_points == 9 && (
                bank_card == 2 ||
                bank_card >= 7
            )
        )
    {
        return MessageAction::Hit;
    }
    else if
        player_points == 11 ||
        player_points == 10 &&
        bank_card <= 9 ||
        player_points == 9 && (
            bank_card >= 3 ||
            bank_card <= 6
        )
    {
        return MessageAction::DoubleDown;
    }

    return MessageAction::Stand;
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

            let mut basic_strategy_action: MutexGuard<MessageAction> =
                self.basic_strategy_action_mutex_arc.lock().unwrap();
            let player_cards: MutexGuard<Vec<u16>> =
                self.player_cards_mutex_arc.lock().unwrap();

            *basic_strategy_action = get_strategic_action(
                &player_cards,
                &bank_cards,
            );
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

