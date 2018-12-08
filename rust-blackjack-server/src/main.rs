extern crate ws;

use ws::{
    listen,
    Sender,
    Handler,
    Message,
    Result,
    CloseCode,
};

struct Server {
    output: Sender,
}

impl Handler for Server {

    ///
    ///
    ///
    fn on_message(&mut self, message: Message) -> Result<()> {
        println!("Received message: {}", message);
        self.output.send("OK")
    }

    ///
    ///
    ///
    fn on_close(&mut self, _: CloseCode, _: &str) {
        println!("Terminate socket.");
        self.output.shutdown().unwrap();
    }
}

fn main() {

    listen("127.0.0.1:3000", |output| {
        Server { output }
    }).unwrap();
}
