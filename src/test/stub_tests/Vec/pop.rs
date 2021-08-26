include!{"../../rmc-prelude.rs"}

fn main() {
    fn pop_test() {
        let mut vec = rmc_vec![1, 2, 3];
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec, [1, 2]);
    }

    pop_test();
}
