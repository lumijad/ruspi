use std::env;

use futures::{future};
use warp::{self, Filter};

use crate::routes;
use crate::template_engine::register_templates;
use std::net::SocketAddr;

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

fn get_bind_addresses() -> (Option<SocketAddr>, Option<SocketAddr>) {
    let http = env::var("BIND_ADDRESS_HTTP");
    let https = env::var("BIND_ADDRESS_HTTPS");

    let mut http_address: Option<SocketAddr> = None;
    let mut https_address: Option<SocketAddr> = None;

    if http.is_ok() {
        http_address = Some(http.unwrap().parse().expect("BIND_ADDRESS_HTTP is invalid"));
    }

    if https.is_ok() {
        https_address = Some(https.unwrap().parse().expect("BIND_ADDRESS_HTTPS is invalid"));
    }

    (http_address, https_address)

}


pub async fn start() {
    register_templates();

    let addresses = get_bind_addresses();

    if addresses.0.is_none() && addresses.1.is_none() {
        panic!("At least one bind address must be set!")
    }

    let routes = routes::get_routes().with(warp::log(APPLICATION_NAME));

    if addresses.0.is_some() && addresses.1.is_some() {
        let http_warp = warp::serve(routes.clone()).bind(addresses.0.unwrap());
        let https_warp = warp::serve(routes).tls().cert_path("tls/cert.pem")
            .key_path("tls/key.pem").bind(addresses.1.unwrap());
        future::join(http_warp, https_warp).await;

    } else if addresses.0.is_some() {
        warp::serve(routes).run(addresses.0.unwrap()).await;

    } else if addresses.1.is_some() {
        warp::serve(routes).tls().cert_path("tls/cert.pem")
            .key_path("tls/key.pem").run(addresses.1.unwrap()).await;
    }

}


