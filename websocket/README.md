# Web Socket
Play with Rust web sockets

### How to run
- Set your root directory to rust_play/websocket
- Open up 2 terminals
- In the first terminal, run:

    `cargo run --bin server`
- In the second terminal, run:

    `cargo run --bin client`

You should see the server and client exchange a message.

### Mini-Project Objectives:
- Server
    1. Start client event loop (single-threaded, synchronous)
    2. Create a web socket, bind it to port 8081
    3. Wait for a connection request
    4. Make the connection
    5. Receive a message, print it out in the terminal
    6. Close the connection

- Client
    1. Start - ephemeral use so no event loop needed
    2. Create a socket, bind to port 8080
    3. Initiate connection
    4. Send a message
