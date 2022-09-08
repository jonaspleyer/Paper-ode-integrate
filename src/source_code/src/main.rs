#![feature(fn_traits, unboxed_closures)]

mod overload;
use overload::my_func_impl::my_func;

// Overloaded functions in Rust!
fn main() {
    assert_eq!(my_func(12usize), 12u64);
    assert_eq!(my_func(3u8, 4u8), 7u16);
}