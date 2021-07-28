#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_string_rs_202_0() {
        use std::mem;

        let story = String::from("Once upon a time...");

        // Prevent automatically dropping the String's data
        let mut story = mem::ManuallyDrop::new(story);

        let ptr = story.as_mut_ptr();
        let len = story.len();
        let capacity = story.capacity();

        // story has nineteen bytes
        assert_eq!(19, len);

        // We can re-build a String out of ptr, len, and capacity. This is all
        // unsafe because we are responsible for making sure the components are
        // valid:
        let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

        assert_eq!(String::from("Once upon a time..."), s);
    }
    _doctest_main_library_alloc_src_string_rs_202_0()
}
