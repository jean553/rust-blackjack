//! The message structure to send through the socket.

use crate::message_action::MessageAction;

#[derive(Serialize, Deserialize)]
pub struct SocketMessage {
    pub action: MessageAction,
    pub text: String,
    pub card_index: u16,
    pub cards_amount: u16,
    pub player_handpoints: u8,
    pub bank_cards: Vec<u16>,
}
