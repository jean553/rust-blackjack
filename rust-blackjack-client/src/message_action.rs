//! Socket message action enumeration.

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum MessageAction {
    SendPlayerCard,
    SendBankCard,
    Hit,
    Stand,
    DoubleDown,
    Continue,
    SendBankCards,
    Restart,
    Split,
    NoSplit,
}
