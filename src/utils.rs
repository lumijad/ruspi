use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::ffi::OsString;
use std::ops::Deref;
use std::fs::read_to_string;
use serde_json::{Value};

pub fn read_template_config() -> HashMap<String, Value> {

    let base_dir = get_base_dir();
    let mut path = PathBuf::from(base_dir);
    path.push("config");
    path.push("template_config.json");

    let res = read_to_string(path.as_path());

    if res.is_err() {
        panic!("Error reading config file {}. Error: {}", path.into_os_string().to_str().unwrap(), res.err().unwrap().to_string());
    }
    let config = res.unwrap();

    let config: Value = serde_json::from_str(&config).unwrap();

    let pages = config["pages"].clone();
    let pages_array = pages.as_array().unwrap();

    let mut hashmap: HashMap<String, Value> = HashMap::new();

    for page in pages_array {
        hashmap.insert(String::from(page["name"].as_str().unwrap()), page["params"].clone() as Value);
    }

    hashmap

}

pub fn get_base_dir() -> OsString {

    const BASE_DIR: &'static str = "BASE_DIR";

    // Search in HASHMAP
    let base_dir = config_get(BASE_DIR);
    if base_dir.is_some() {
        return OsString::from(base_dir.unwrap().deref());
    }

    // Search in env
    let path: PathBuf;
    let base_dir = env::var("BASE_DIR");

    if base_dir.is_ok() {
        let base_dir = base_dir.unwrap();

        if base_dir == "." {
            path = env::current_dir().unwrap();
        } else {
            path = PathBuf::from(base_dir);
        }
    } else {
        path = env::current_dir().unwrap();
    }

    let os_string = path.into_os_string();


    config_set(String::from(BASE_DIR), String::from(os_string.to_str().unwrap()));

    os_string
}

lazy_static! {
    static ref CONFIG: RwLock<HashMap<String, Arc<String>>> = RwLock::new(HashMap::new());
}
pub fn config_set(name:String, value:String) {
    CONFIG.write().unwrap().insert(name, Arc::new(value));
}
pub fn config_get(name: &str) -> Option<Arc<String>> {
    CONFIG.read().unwrap().get(name).map(|v| Arc::clone(v))
}


