#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_vec_mod_rs_155_0() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);

        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);

        vec[0] = 7;
        assert_eq!(vec[0], 7);

        vec.extend([1, 2, 3].iter().copied());

        for x in &vec {
            println!("{}", x);
        }
        assert_eq!(vec, [7, 1, 2, 3]);
    }
    _doctest_main_library_alloc_src_vec_mod_rs_155_0()
}
