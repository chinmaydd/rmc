include!{"../../rmc-prelude.rs"}

fn main() {
    fn truncate_zero_test() {
        let mut vec = rmc_vec![1, 2, 3];
        vec.truncate(0);
        assert_eq!(vec, []);
    }

    truncate_zero_test();
}
