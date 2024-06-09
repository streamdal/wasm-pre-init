use std::{thread, time::Duration};

static mut NAME :&str = "N/A";

#[no_mangle]
pub fn _start() {
    setup();

    println!("Name: {}", unsafe { NAME });
}

pub fn setup() {
    thread::sleep(Duration::from_secs(5));

    unsafe {
        NAME = "John Doe";
    }
}
