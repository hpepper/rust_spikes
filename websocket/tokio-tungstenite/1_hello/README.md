# Hello

This shows how to use tokio-tungstenite to create a websocket connection.

The code is based on the [tokio-tungstenite on github](https://github.com/snapview/tokio-tungstenite)

* [tokio_tungstenite documentation](https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/all.html)

tokio-tungstenite seems to be a higher level access to [Tungstenite](https://crates.io/crates/tungstenite)

## Running the app

* in tab 1 run: `cargo run --bin server`
* in tab 2 run : `cargo run --bin client`

The server will send a message to the client every second, the message will include the clients IP address and port.

You can run multiple client against the server at the same time.
