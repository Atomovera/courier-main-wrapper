use core::ffi::c_char;
use libc;

pub unsafe fn c_string_to_rust(ptr:*mut u8)->Result<&'static str,core::str::Utf8Error>{
    let length = libc::strlen(ptr as *const c_char);
    let slice = core::slice::from_raw_parts(ptr, length);
    core::str::from_utf8(slice)
}

pub unsafe fn ptr_to_slice(ptr:*mut *mut u8,length:usize)->&'static mut [*mut u8]{
    core::slice::from_raw_parts_mut(ptr, length)
}