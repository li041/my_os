use core::ptr;
use crate::config::*;
use core::arch::asm;
use crate::trap::TrapContext;
use crate::println;

/* 每一个应用程序都有自己的user stack */
#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, trap_cx: TrapContext) -> usize {
        let trap_cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *trap_cx_ptr = trap_cx;
        }
        trap_cx_ptr as usize
    }
}

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

pub fn get_num_app() -> usize {
    extern "C" { fn _num_app(); }
    unsafe {(_num_app as usize as *const usize).read_volatile() }
}

pub fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

// get app info with entry and sp 
// and save 'TrapContext' in corresponding kernel stack
pub fn init_app_cx(app_id: usize) -> usize {
    KERNEL_STACK[app_id].push_context(
        TrapContext::app_init_context(
            get_base_i(app_id),
            USER_STACK[app_id].get_sp(),
        )
    )
}

pub fn load_apps() {
    extern "C" { fn _num_app(); }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    /* app_start存放每个app的初始地址 */
    let app_start = unsafe {
        core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1)
    };
    // clear i-cache first
    unsafe {
        asm!("fence.i");
    }
    // load apps
    for i in 0..num_app {
        let base_i = get_base_i(i);
        // clear region
        unsafe { 
            ptr::write_bytes(base_i as usize as *mut u8, 0, APP_SIZE_LIMIT);
        }
        // load app from data section to memory
        let len = app_start[i+1] - app_start[i];
        unsafe {
            ptr::copy_nonoverlapping(app_start[i] as *const u8, base_i as *mut u8, len);
        }
        println!("load app_{} at 0x{:x} with {} length", i, app_start[i], len);
        println!("app_{} located at 0x{:x}", i, base_i);
    }
    unsafe{
        asm!("fence.i");
    }
}
