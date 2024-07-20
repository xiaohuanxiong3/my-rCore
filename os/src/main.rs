#![no_std]
#![no_main]
#![feature(panic_info_message)]
mod lang_items;
mod sbi;
mod console;
mod logging;

// fn main() {
//     // println!("Hello, world!");
// }
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

use log::{trace,debug,info,warn,error,LevelFilter};
use logging::SIMPLE_LOGGER;

// 告诉编译器保留函数的原始名称，不做修改
#[no_mangle]
pub fn rust_main() -> !{
    clear_bss();
    // 初始化日志组件
    log::set_logger(&SIMPLE_LOGGER).unwrap();
    log::set_max_level(match option_env!("log") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });

    // 自定义println!宏
    println!("Hello world!");

    // 测试日志宏
    trace!("this is trace info!");
    debug!("this is debug info!");
    info!("this is info info!");
    warn!("this is warn info!");
    error!("this is error info!");

    print_os_space();

    // 程序退出
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    })
}

fn print_os_space() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
    }
    // 这段代码输出了 os 内存空间布局，这到这些信息对于编写 os 十分重要
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    error!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
}
