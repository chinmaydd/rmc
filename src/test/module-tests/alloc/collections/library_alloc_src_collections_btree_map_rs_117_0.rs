#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_map_rs_117_0() {
        use std::collections::BTreeMap;

        // type inference lets us omit an explicit type signature (which
        // would be `BTreeMap<&str, u8>` in this example).
        let mut player_stats = BTreeMap::new();

        fn random_stat_buff() -> u8 {
            // could actually return some random value here - let's just return
            // some fixed value for now
            42
        }

        // insert a key only if it doesn't already exist
        player_stats.entry("health").or_insert(100);

        // insert a key using a function that provides a new value only if it
        // doesn't already exist
        player_stats
            .entry("defence")
            .or_insert_with(random_stat_buff);

        // update a key, guarding against the key possibly not being set
        let stat = player_stats.entry("attack").or_insert(100);
        *stat += random_stat_buff();
    }
    _doctest_main_library_alloc_src_collections_btree_map_rs_117_0()
}
