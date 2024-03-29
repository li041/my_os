
const SBI_SET_TIMER: usize = 0;

// use interface provided by RustSBI to set mtimecmp
pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as u64);
}

pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    }
    else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
