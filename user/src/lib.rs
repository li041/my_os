#![no_std]
/* 用于支持与链接相关的宏 */
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console; /* print and println macro for user app, implemented through syscall */
mod lang_items; /* panic_handler */
mod syscall; 

#[no_mangle]
#[link_section = ".text.entry"]
/* 利用rust的宏将_start编译后的汇编代码放在一个名为.text.entry的代码段中 */
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
/*  该main标记为弱链接，这样在最后链接的时候，虽然在 lib.rs 和 bin 目录下的某个应用程序
    都有 main 符号，但由于 lib.rs 中的 main 符号是弱链接，
    链接器会使用 bin 目录下的应用主逻辑作为 main 。这里我们主要是进行某种程度上的保护，
    如果在 bin 目录下找不到任何 main ，那么编译也能够通过，但会在运行时报错。  */
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn yield_() -> isize {
    sys_yield()
}

pub fn get_time() -> isize {
    sys_get_time()
}
