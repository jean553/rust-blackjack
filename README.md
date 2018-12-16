# rust-blackjack

Very simple blackjack game with server and client programs,
for WebSocket/multi-threading self-learning purposes.

**NOTE: This is a work in progress...**

## Develop and build

```sh
vagrant up
```

```sh
vagrant ssh
```

Build the server:

```sh
cd rust-blackjack/rust-blackjack-server/
cargo build --release
```

Build the client:

```sh
cd rust-blackjack/rust-blackjack-client/
cargo build --release
```

## Projects

 * `rust-blackjack-client` - graphical client with Piston library,
 * `rust-blackjack-server` - server

## Credits

### Images

Playing cards images from https://code.google.com/archive/p/vector-playing-cards/ (public domain).

### Fonts

* Cambay - https://fontlibrary.org/en/font/cambay - SIL OpenFont License - by Pooja Saxena (http://www.poojasaxena.in/)
