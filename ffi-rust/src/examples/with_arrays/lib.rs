#![allow(non_snake_case)]

mod count;

use count::Counter;
use std::mem::transmute;

#[no_mangle]
pub extern fn createCounter(val: u32) -> *mut Counter {
    let _counter = unsafe { transmute(Box::new(Counter::new(val))) };
    _counter
}

#[no_mangle]
pub extern fn getCounterValue(ptr: *mut Counter) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    _counter.get()
}

#[no_mangle]
pub extern fn incrementCounterBy(ptr: *mut Counter, bys_ptr: *const u32, bys_len: usize) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    let bys = unsafe { std::slice::from_raw_parts(bys_ptr, bys_len) };
    _counter.incr(bys)
}

#[no_mangle]
pub extern fn decrementCounterBy(ptr: *mut Counter, bys_ptr: *const u32, bys_len: usize) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    let bys = unsafe { std::slice::from_raw_parts(bys_ptr, bys_len) };
    _counter.decr(bys)
}

#[no_mangle]
pub extern fn destroyCounter(ptr: *mut Counter) {
    let _counter: Box<Counter> = unsafe{ transmute(ptr) };
    // Drop
}
