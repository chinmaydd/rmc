#![allow(unused_variables)]
#![deny(warnings)]
#![allow(unused_must_use)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_fmt_rs_484_0() {
        use std::fmt;
        use std::io::{self, Write};

        let mut some_writer = io::stdout();
        write!(
            &mut some_writer,
            "{}",
            format_args!("print with a {}", "macro")
        );

        fn my_fmt_fn(args: fmt::Arguments) {
            write!(&mut io::stdout(), "{}", args);
        }
        my_fmt_fn(format_args!(", or a {} too", "function"));
    }
    _doctest_main_library_alloc_src_fmt_rs_484_0()
}
