#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

mod routes;
mod server;
mod sse;
mod template_engine;
mod utils;
mod websocket;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    server::start().await;
}

