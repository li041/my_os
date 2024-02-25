mod context;

use core::arch::global_asm;
use crate::task::{self, suspend_current_and_run_next};
use crate::syscall::syscall;
use crate::timer;
use riscv::register::{
    mtvec::TrapMode, 
    scause::{self, Exception, Interrupt, Trap}, 
    stval, stvec
};

pub use self::context::TrapContext;


global_asm!(include_str!("trap.S"));

/// initialize CSR `stvec` as the entry of `__alltraps`
/// __alltraps在trap.S中, 通过global_asm! macro引入
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

/* trap.S中__alltraps call trap_handler调用trap_handler */
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            task::exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            task::exit_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            timer::set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}