#[macro_use]
extern crate lazy_static;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::sync::Mutex;

lazy_static! {
    static ref BUFFER1: Mutex<RefCell<Vec<u32>>> = Mutex::new(RefCell::new(vec![]));
    static ref BUFFER2: Mutex<RefCell<Vec<u32>>> = Mutex::new(RefCell::new(vec![]));
}

#[wasm_bindgen]
pub fn alloc1(n: usize) -> js_sys::Uint32Array {
    let locked = BUFFER1.lock().unwrap();
    let b = &mut locked.borrow_mut();
    b.resize(n, 0);
    for i in 0..n {
        b[i] = i as u32;
    }
    unsafe { js_sys::Uint32Array::view_mut_raw(b.as_mut_ptr(), b.len()) }
}

#[wasm_bindgen]
pub fn get1() -> js_sys::Uint32Array {
    let locked = BUFFER1.lock().unwrap();
    let b = &mut locked.borrow_mut();
    unsafe { js_sys::Uint32Array::view_mut_raw(b.as_mut_ptr(), b.len()) }
}

#[wasm_bindgen]
pub fn alloc2(n: usize) -> js_sys::Uint32Array {
    let locked = BUFFER2.lock().unwrap();
    let b = &mut locked.borrow_mut();
    b.resize(n, 0);
    for i in 0..n {
        b[i] = i as u32;
    }
    unsafe { js_sys::Uint32Array::view_mut_raw(b.as_mut_ptr(), b.len()) }
}

