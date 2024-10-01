# Courier-main-wrapper
This library is used in `courier` project.

# Usage
```rust
#![no_main]

#[no_mangle]
pub fn courier_main(args: courier_main_wrapper::args::Arg)->isize{
    for arg in args.into_iter() {
        println!("{}",arg);
    }
    0
}
```