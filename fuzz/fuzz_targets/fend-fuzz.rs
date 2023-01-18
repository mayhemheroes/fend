#![no_main]
extern crate fend_core;
use libfuzzer_sys::fuzz_target;
use std::str;
use fend_core::*;

fuzz_target!(|data: &[u8]| {
    match str::from_utf8(data) {
        Ok(in_string)=>{
            let mut context = Context::new();
            let result = evaluate(in_string, &mut context);
        },
        Err(..)=>()
    }
});