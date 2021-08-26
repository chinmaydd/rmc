include!{"../../rmc-prelude.rs"}

fn main() {
    fn set_len_test() {
        let mut vec = rmc_vec![rmc_vec![1, 0, 0], rmc_vec![0, 1, 0], rmc_vec![0, 0, 1]];
        // SAFETY:
        // 1. `old_len..0` is empty so no elements need to be initialized.
        // 2. `0 <= capacity` always holds whatever `capacity` is.
        //
        // There is a memory leak here.
        unsafe {
            vec.set_len(0);
        }
    }
    
    set_len_test();
}
