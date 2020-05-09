#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

mod routes;
mod server;
mod template_engine;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    server::start().await;
}

