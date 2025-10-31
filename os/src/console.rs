//! # 控制台输出模块
//!
//! 本模块实现了格式化输出功能，提供了类似标准库的 print! 和 println! 宏。
//! 通过 SBI 接口实现字符输出，用于内核的调试和信息输出。

use crate::sbi::console_putchar;
use core::fmt::{self, Write};

/// 标准输出结构体
/// 
/// 这是一个类单元结构体（unit-like struct），用于实现 core::fmt::Write trait。
/// 它作为格式化输出的目标，将字符通过 SBI 接口输出到控制台。
struct Stdout;

/// 为 Stdout 实现 Write trait
/// 
/// 实现 write_str 方法，将字符串中的每个字符逐一输出到控制台。
/// 这使得 Stdout 可以作为格式化输出的目标使用。
impl Write for Stdout {
    /// 将字符串写入标准输出
    /// 
    /// # 参数
    /// 
    /// * `s` - 要输出的字符串切片
    /// 
    /// # 返回值
    /// 
    /// 返回 fmt::Result，表示写入操作是否成功
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 遍历字符串中的每个字符，通过 SBI 接口输出
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

/// 格式化输出函数
/// 
/// 这是一个内部函数，被 print! 和 println! 宏调用。
/// 它创建一个 Stdout 实例并使用 write_fmt 方法进行格式化输出。
/// 
/// # 参数
/// 
/// * `args` - 格式化参数，由 format_args! 宏生成
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// print! 宏
/// 
/// 用于格式化输出到控制台，类似于标准库的 print! 宏。
/// 不会在输出末尾添加换行符。
/// 
/// # 示例
/// 
/// ```
/// print!("Hello, world!");
/// print!("The answer is {}", 42);
/// ```
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

/// println! 宏
/// 
/// 用于格式化输出到控制台，并在末尾自动添加换行符。
/// 类似于标准库的 println! 宏。
/// 
/// # 示例
/// 
/// ```
/// println!("Hello, world!");
/// println!("The answer is {}", 42);
/// ```
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    };
}