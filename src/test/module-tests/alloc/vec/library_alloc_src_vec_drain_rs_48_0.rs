#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_drain_rs_48_0() {
        let mut vec = vec!['a', 'b', 'c'];
        let mut drain = vec.drain(..);
        assert_eq!(drain.as_slice(), &['a', 'b', 'c']);
        let _ = drain.next().unwrap();
        assert_eq!(drain.as_slice(), &['b', 'c']);
    }
    _doctest_main_library_alloc_src_vec_drain_rs_48_0()
}
