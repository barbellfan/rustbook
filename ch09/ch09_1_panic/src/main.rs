fn main() {
    //_call_panic();
    //_panic_in_lib();
}

fn _call_panic() {
    panic!("crash and burn");
}

fn _panic_in_lib() {
    let v = vec![1, 2, 3];

    v[99]; // this line will panic
    /*
    run with BACKTRACE=1 to see full error stack:
    thread 'main' panicked at ch09/ch09_1_panic/src/main.rs:13:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library/std/src/panicking.rs:597:5
   1: core::panicking::panic_fmt
             at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library/core/src/panicking.rs:72:14
   2: core::panicking::panic_bounds_check
             at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library/core/src/panicking.rs:180:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library/core/src/slice/index.rs:261:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library/alloc/src/vec/mod.rs:2728:9
   6: panic::panic_in_lib
             at ./ch09/ch09_1_panic/src/main.rs:13:6
   7: panic::main
             at ./ch09/ch09_1_panic/src/main.rs:3:5
   8: core::ops::function::FnOnce::call_once
             at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
     */
}
