

pub fn console_putchar(c : usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

// 这里的failure表示系统是否正常退出，这会影响Qemu模拟器进程退出之后的返回值，
// 我们则以此判断系统的执行是否正常
pub fn shutdown(failure : bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}

pub fn sleep(time : i32) {
    #[allow(deprecated)]
    sbi_rt::legacy::set_timer(1);
}