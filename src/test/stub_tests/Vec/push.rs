include!{"../../rmc-prelude.rs"}

fn main() {
    fn push_test() {
        let mut vec = rmc_vec![1, 2];
        vec.push(3);
        assert!(vec == [1, 2, 3]);
    }

    push_test()
}
