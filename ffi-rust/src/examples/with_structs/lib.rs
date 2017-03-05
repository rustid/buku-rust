#![allow(non_snake_case)]

mod count;

use count::{Counter, Args};
use std::mem::transmute;

#[no_mangle]
pub extern fn createCounter(args: Args) -> *mut Counter {
    let _counter = unsafe { transmute(Box::new(Counter::new(args))) };
    _counter
}

#[no_mangle]
pub extern fn getCounterValue(ptr: *mut Counter) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    _counter.get()
}

#[no_mangle]
pub extern fn incrementCounterBy(ptr: *mut Counter) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    _counter.incr()
}

#[no_mangle]
pub extern fn decrementCounterBy(ptr: *mut Counter) -> u32 {
    let mut _counter = unsafe { &mut *ptr };
    _counter.decr()
}

#[no_mangle]
pub extern fn destroyCounter(ptr: *mut Counter) {
    let _counter: Box<Counter> = unsafe{ transmute(ptr) };
    // Drop
}
