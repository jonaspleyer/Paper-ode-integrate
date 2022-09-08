// gritty details follow
#[allow(non_camel_case_types, non_upper_case_globals)]
pub mod my_func_impl {
    #[derive(Clone, Copy)]
    pub struct my_func_struct;

    fn my_func_body_1(v: usize) -> u64 {
        v as u64
    }
    impl FnOnce<(usize,)> for my_func_struct {
        type Output = u64;
        #[inline(always)]
        extern "rust-call" fn call_once(self, args: (usize,)) -> Self::Output {
            my_func_body_1.call_once(args)
        }
    }
    impl FnMut<(usize,)> for my_func_struct {
        #[inline(always)]
        extern "rust-call" fn call_mut(&mut self, args: (usize,)) -> Self::Output {
            my_func_body_1.call_once(args)
        }
    }
    impl Fn<(usize,)> for my_func_struct {
        #[inline(always)]
        extern "rust-call" fn call(&self, args: (usize,)) -> Self::Output {
            my_func_body_1.call_once(args)
        }
    }
    impl From<my_func_struct> for fn(usize) -> u64 {
        fn from(_: my_func_struct) -> fn(usize) -> u64 {
            my_func_body_1
        }
    }

    fn my_func_body_2(x: u8, y: u8) -> u16 {
        (x as u16) + (y as u16)
    }
    impl FnOnce<(u8, u8)> for my_func_struct {
        type Output = u16;
        #[inline(always)]
        extern "rust-call" fn call_once(self, args: (u8, u8)) -> Self::Output {
            my_func_body_2.call_once(args)
        }
    }
    impl FnMut<(u8, u8)> for my_func_struct {
        #[inline(always)]
        extern "rust-call" fn call_mut(&mut self, args: (u8, u8)) -> Self::Output {
            my_func_body_2.call_once(args)
        }
    }
    impl Fn<(u8, u8)> for my_func_struct {
        #[inline(always)]
        extern "rust-call" fn call(&self, args: (u8, u8)) -> Self::Output {
            my_func_body_2.call_once(args)
        }
    }
    impl From<my_func_struct> for fn(u8, u8) -> u16 {
        fn from(_: my_func_struct) -> fn(u8, u8) -> u16 {
            my_func_body_2
        }
    }

    pub const my_func: my_func_struct = my_func_struct;
}