//! Socket message action enumeration.

#[derive(Serialize, Deserialize, PartialEq)]
pub enum MessageAction {
    NewPlayer,
    SendPlayerCard,
    SendBankCard,
    Hit,
}

