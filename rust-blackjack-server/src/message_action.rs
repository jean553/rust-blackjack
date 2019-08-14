//! The message action enumeration, containing all possible actions to send.

#[derive(Serialize, Deserialize, PartialEq)]
pub enum MessageAction {
    SendPlayerCard,
    SendBankCard,
    Hit,
    Stand,
    Continue,
    SendBankCards,
    Restart,
}
