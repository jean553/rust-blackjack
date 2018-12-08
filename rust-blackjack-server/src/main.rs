extern crate ws;

use ws::{
    listen,
    Sender,
    Handler,
    Message,
    Result,
    CloseCode,
    Handshake,
};

struct Server {
    output: Sender,
}

impl Handler for Server {

    ///
    ///
    ///
    fn on_open(
        &mut self,
        handshake: Handshake
    ) -> Result<()> {

        println!(
            "New connexion from {}.",
            handshake.remote_addr().unwrap().unwrap()
        );
        self.output.send("OK")
    }

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

    const LISTENING_ADDRESS: &str = "172.0.0.1:3000";
    listen(LISTENING_ADDRESS, |output| {
        Server { output }
    }).unwrap();
}
