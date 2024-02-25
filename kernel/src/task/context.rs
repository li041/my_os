/* 
 * ra记录switch ret后到哪里执行
 */

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    // return address
    ra: usize,
    // kernel stack pointer of app
    sp: usize,
    // callee saved registers
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12]
        }
    }
    /// set task context {__restore ASM funciton in trap/trap.S, kernel stack, s_0..12 }
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr ,
            s: [0; 12],
        }
    }
}