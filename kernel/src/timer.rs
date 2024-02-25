use riscv::register::time;
use crate::config::CLOCK_FREQ;    
use crate::sbi;

const TICKS_PER_SEC: usize = 100; // 每秒产生的时间中断
const MSEC_PER_SEC: usize = 1000; // 每秒计时器的增量

pub fn get_time() -> usize {
    time::read()
}

/* 设置mtimecmp为当前计时器的值+下一次时间中断计时器增量 */
pub fn set_next_trigger() {
    sbi::set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

/* 统计一个应用运行时长(毫秒秒) */
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}
