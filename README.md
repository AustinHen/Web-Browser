This is an HTML-only web browser written in Rust. It fetches files over HTTP/TCP/TLS and generates/renders DOM trees. I made this to mess around with network programming. 

This browser uses HTTP 1.1. Implementing HTTP 1.1 on the client side has some interesting challenges. One of the main issues with 1.1 is that you need to do a TCP handshake for every file you want to download. Essentially, you have to ping the server before you can request a file for every file. Websites typically have three files as a bare minimum (HTML, JS, and CSS files) but can easily have way more (e.g. images, and fonts). If the client and server are on opposite sides of the US, a single TCP handshake can add over 100ms of delay. These messages don't contain any data and can add seconds of delay. Browsers using HTTP 1.1, in turn, have to use techniques such as multi-threading (where they send out multiple requests at once) to be usable. 

## Running:
 Just call
```console
cargo run
```

## Demo: 
Soon to be added
