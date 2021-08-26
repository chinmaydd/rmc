include!{"../../rmc-prelude.rs"}

fn main() {
    fn clear_test() {
        let mut v = rmc_vec![1, 2, 3];

        v.clear();

        assert!(v.is_empty());
    }

    clear_test();
}
