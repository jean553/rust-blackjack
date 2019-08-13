//! Socket message structure.

use crate::message_action::MessageAction;

#[derive(Serialize, Deserialize)]
pub struct SocketMessage {
    pub action: MessageAction,
    pub card_index: u16,
    pub cards_amount: u16,
    pub text: String,
    pub player_handpoints: u8,
    pub bank_cards: Vec<u16>,
}
