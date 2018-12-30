use ws::Sender;
#[derive(PartialEq)]
pub enum Event {
    Connect(Sender),
    Disconnect,
}
