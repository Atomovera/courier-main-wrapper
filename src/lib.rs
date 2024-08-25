#![no_std]

use core::ffi::c_int;

pub mod args;
mod utils;

extern "Rust" {
    pub fn courier_main(args:args::Arg)->isize;
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn main(argc:c_int,argv:*mut *mut u8)->c_int{
    match args::Arg::new(argc as isize,argv as *mut *mut u8) {
        Ok(args) => return courier_main(args) as _,
        Err(_) => {
            libc::printf("ERROR: INVALID COMMAND ARGUMENTS\n\0".as_ptr() as _);
            return 255;
        }
    }
}