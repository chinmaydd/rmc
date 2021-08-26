include!{"../../rmc-prelude.rs"}

fn main() {
    fn insert_test() {
        let mut vec = rmc_vec![1, 2, 3];
        vec.insert(1, 4);
        assert_eq!(vec, [1, 4, 2, 3]);
        vec.insert(4, 5);
        assert_eq!(vec, [1, 4, 2, 3, 5]);
    }

    insert_test();
}
