use std::process::exit;


#[no_mangle]
pub fn panic() {
    backtrace::trace(|frame| {
        let ip = frame.ip();
        println!("ip: {:?}", ip);
        true
    });
    exit(1);
}