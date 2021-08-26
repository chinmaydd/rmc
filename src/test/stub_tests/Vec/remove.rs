include!{"../../rmc-prelude.rs"}

fn main() {
    fn remove_test() {
        let mut v = rmc_vec![1, 2, 3];
        assert_eq!(v.remove(1), 2);
        assert_eq!(v, [1, 3]);
    }

    remove_test();
}
