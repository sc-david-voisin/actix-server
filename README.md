# Test actix-server

Tiny HTTP server that serves web page on port 8080.
Listens on port defined by `PORT` environment variable if present.

Based on [Actix Web](https://actix.rs/) <3

# Routes

* `/`        serves [Scalingo](https://scalingo.com/) template page.
* `/status`  returns text message, to ensure the server is up and listening.