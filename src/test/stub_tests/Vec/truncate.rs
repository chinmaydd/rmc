include!{"../../rmc-prelude.rs"}

fn main() {
    fn truncate_test() {
        let mut vec = rmc_vec![1, 2, 3];
        vec.truncate(8);
        assert_eq!(vec, [1, 2, 3]);
    }

    truncate_test();
}
