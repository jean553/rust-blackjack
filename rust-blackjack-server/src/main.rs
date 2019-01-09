#![deny(warnings)]
#![feature(uniform_paths)]

extern crate ws;
extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod message_action;
mod socket_message;
mod server;

use ws::listen;

use server::Server;

fn main() {

    const LISTENING_ADDRESS: &str = "127.0.0.1:3000";
    listen(LISTENING_ADDRESS, |output| {
        Server::new(output)
    }).unwrap();
}
