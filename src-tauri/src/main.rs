mod logger;
mod utils;
use client_lib::login::request::request_connection_server;
use utils::set_console_title;

#[tokio::main]
async fn main() {
    set_console_title().await;

    if request_connection_server().await {
        client_lib::run();
    }
}
