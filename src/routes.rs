use std::path::PathBuf;

use warp::{self, Filter};

use crate::sse::get_sse_filters;
use crate::template_engine::render;
use crate::utils::get_base_dir;
use crate::websocket::get_websocket_filters;

pub fn static_resources() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let base_dir = get_base_dir();
    let path = PathBuf::from(base_dir);

    let mut assets = path.clone();
    assets.push("static/assets");

    let mut css = path.clone();
    css.push("static/css");

    let mut webfonts = path.clone();
    webfonts.push("static/webfonts");

    let mut js = path.clone();
    js.push("static/js");

    let acme = PathBuf::from("/www/vhosts/ruspi.dev/httpdocs/.well-known/acme-challenge");

    let acme = warp::path(".well-known/acme-challenge").and(warp::fs::dir(acme.into_os_string()));

    warp::path("assets").and(warp::fs::dir(assets.into_os_string()))
        .or(warp::path("css").and(warp::fs::dir(css.into_os_string())))
        .or(warp::path("webfonts").and(warp::fs::dir(webfonts.into_os_string())))
        .or(warp::path("js").and(warp::fs::dir(js.into_os_string())))
        .or(acme)
}

pub fn get_page_filters() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {

    // /*  all get requests with a parameter
    warp::get()
        .and(warp::any())
        .and(warp::path::param())
        .and_then(render)
        // / all get request without a parameter will be mapped to index
        .or(warp::path::end()
            .and(warp::get())
            .map(|| "index".to_string())
            .and_then(render))
}

pub fn get_routes() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    static_resources().or(get_sse_filters()).or(get_websocket_filters()).or(get_page_filters())
}