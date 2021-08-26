include!{"../../rmc-prelude.rs"}

fn main() {
    fn truncate_reduce_test() {
        let mut vec = rmc_vec![1, 2, 3, 4, 5];
        vec.truncate(2);
        assert_eq!(vec, [1, 2]);
    }

    truncate_reduce_test();
}
