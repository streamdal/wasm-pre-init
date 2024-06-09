use std::{thread, time::Duration};

static mut NAME :&str = "N/A";

#[export_name = "wizer.initialize"]
pub fn init() {
    setup();
}


#[no_mangle]
pub fn _start() {
    println!("Name: {}", unsafe { NAME });
}

pub fn setup() {
    thread::sleep(Duration::from_secs(5));

    unsafe {
        NAME = "John Doe";
    }
}
