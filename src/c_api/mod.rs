use std::ffi::CStr;

use libc::{c_char, c_double, size_t};

use crate::BloomFilter;

#[no_mangle]
pub extern fn bb_bloom_filter_from_fpp(
    p: c_double,
    n: size_t,
) -> *mut BloomFilter {
    Box::into_raw(Box::new(BloomFilter::from_fpp(p, n)))
}

#[no_mangle]
pub extern fn bb_bloom_filter_insert(
    ctx: *mut BloomFilter,
    key: *const c_char,
) -> bool {
    let filter = unsafe { &mut *ctx };
    let key = unsafe { CStr::from_ptr(key).to_str().unwrap() };
    filter.insert(key)
}

#[no_mangle]
pub extern fn bb_bloom_filter_contains(
    ctx: *mut BloomFilter,
    key: *const c_char,
) -> bool {
    let filter = unsafe { &mut *ctx };
    let key = unsafe { CStr::from_ptr(key).to_str().unwrap() };
    filter.contains(key)
}

#[no_mangle]
pub extern fn bb_bloom_filter_free(ctx: *mut BloomFilter) {
    unsafe { Box::from_raw(ctx); }
}
