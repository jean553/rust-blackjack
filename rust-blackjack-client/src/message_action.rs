#[derive(Serialize, Deserialize, PartialEq)]
pub enum MessageAction {
    NewPlayer,
    SendPlayerCard,
    SendBankCard,
    Hit,
}

