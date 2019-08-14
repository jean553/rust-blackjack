//! Socket message action enumeration.

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
