use std::collections::HashMap;
use std::convert::Infallible;
use std::path::PathBuf;
use std::sync::RwLock;

use handlebars::{Handlebars, TemplateFileError};
use log::error;
use serde_json::value::Value;
use warp::Reply;
use warp::http::StatusCode;

use crate::utils::{get_base_dir, read_template_config};

pub struct Registry<'reg> {
    pub hbs: Handlebars<'reg>,
}


lazy_static! {
    pub static ref REGISTRY: RwLock<Registry<'static>> = {
        RwLock::new(Registry::new())
    };
}

pub fn register_templates() {
    REGISTRY.write().unwrap().register_templates();
}

impl Registry<'_> {
    pub fn new() -> Self {
        Registry {
            hbs: Handlebars::new()
        }
    }

    pub fn register_templates(&mut self) {
        let base_dir = get_base_dir();
        let mut path = PathBuf::from(base_dir);
        path.push("templates");

        let res = self.hbs.register_templates_directory(".hbs", path.as_path());
        if res.is_err() {
            let err = res.err().unwrap();

            match err {
                TemplateFileError::TemplateError(e) => {
                    error!("{:?}", e);
                }
                TemplateFileError::IOError(e, n) => {
                    error!("Error {:?}, name: {}", e, n);
                }
            }
        }
    }
}


#[derive(Clone, Debug)]
pub struct TemplateParameters {
    pub name: &'static str,
    pub value: Value,
}

#[derive(Clone)]
pub struct State {
    pub template: TemplateParameters,
}


lazy_static! {
    pub static ref PAGE_MAP: HashMap<String, Value> = {
        read_template_config()
    };
}



pub async fn render(param: String) -> Result<impl Reply, Infallible>
{
    let page_info = PAGE_MAP.get(&param);

    if page_info.is_none() {
        const ERROR404: &'static str = "error_404";
        let page_info: Value = PAGE_MAP.get(ERROR404).unwrap().clone();
        let render = REGISTRY.read().unwrap().hbs
            .render(ERROR404, &page_info)
            .unwrap_or_else(|err| err.to_string());
        return Ok(warp::reply::with_status(warp::reply::html(render), StatusCode::NOT_FOUND));
    }

    let page_info = page_info.unwrap().clone();


    let render = REGISTRY.read().unwrap().hbs
        .render(&param, &page_info)
        .unwrap_or_else(|err| err.to_string());
    Ok(warp::reply::with_status(warp::reply::html(render), StatusCode::OK))
}


