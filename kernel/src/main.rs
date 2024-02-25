#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;

mod config;
mod lang_items;
mod loader;
mod sbi;
#[path ="boards/qemu.rs"]
mod board;
pub mod trap;
pub mod timer;
pub mod syscall;
pub mod sync;
mod task;


use core::arch::global_asm;

use sbi::console_putchar;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    console_putchar('O' as usize);
    console_putchar('K' as usize);
    console_putchar('\n' as usize);
    trap::init();
    loader::load_apps();
    task::run_first_task();
    unreachable!();
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
