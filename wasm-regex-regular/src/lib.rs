use regex::Regex;
use std::env;

#[no_mangle]
pub fn _start() {
    let s = env::var("NUM").unwrap_or("0".to_string());

    let regex = Regex::new(r"^1\d*$").unwrap();

    if regex.is_match(&s) {
        println!("Matched!")
    } else {
        println!("Not matched!");
    }
}
