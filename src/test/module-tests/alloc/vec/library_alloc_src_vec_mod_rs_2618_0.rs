#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_2618_0() {
        let some_predicate = |x: &mut i32| *x == 2 || *x == 3 || *x == 6;
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let mut i = 0;
        while i < vec.len() {
            if some_predicate(&mut vec[i]) {
                let val = vec.remove(i);
                // your code here
            } else {
                i += 1;
            }
        }

        assert_eq!(vec, vec![1, 4, 5]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_2618_0()
}
