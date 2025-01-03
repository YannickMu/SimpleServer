use std::sync::Mutex;

pub static SHUTDOWN: Mutex<Vec<i32>> = Mutex::new(Vec::new());
pub static ARGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub static NOT_FOUND_ERROR: &str = "404 File not found...";

pub static BASE_PATH: &str = "/var/rust/www/";
pub static API_PATH: &str = "rust";
pub static HTML_PATH: &str = "html";
