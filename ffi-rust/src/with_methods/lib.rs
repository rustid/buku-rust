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
pub extern fn incrementCounterBy(ptr: *mut Counter, by: u32) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    _counter.incr(by)
}

#[no_mangle]
pub extern fn decrementCounterBy(ptr: *mut Counter, by: u32) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    _counter.decr(by)
}

#[no_mangle]
pub extern fn destroyCounter(ptr: *mut Counter) {
    let _counter: Box<Counter> = unsafe{ transmute(ptr) };
    // Drop
}
