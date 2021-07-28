#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_1068_0() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, i32> = ["Alice", "Bob", "Carol", "Cheryl"]
            .iter()
            .map(|&s| (s, 0))
            .collect();
        for (_, balance) in map.range_mut("B".."Cheryl") {
            *balance += 100;
        }
        for (name, balance) in &map {
            println!("{} => {}", name, balance);
        }
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_1068_0()
}
