use regex::Regex;
use std::env;

/// A regex that matches numbers that start with "1".
static mut REGEX: Option<Regex> = None;

#[export_name = "wizer.initialize"]
pub fn init() {
    unsafe {
        REGEX = Some(Regex::new(r"^1\d*$").unwrap());
    }
}

#[no_mangle]
pub fn _start() {
    let s = env::var("NUM").unwrap_or("0".to_string());

    let regex = unsafe { REGEX.as_ref().unwrap() };
    if regex.is_match(&s) {
        println!("Matched!")
    } else {
        println!("Not matched!")
    }
}
