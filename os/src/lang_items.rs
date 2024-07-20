
use core::panic::PanicInfo;
use crate::{println, sbi::shutdown};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // info.localtion() 返回的Option 是 Some类型，将Some类型中的值绑定给location变量
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        )
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown(true);
}