#![allow(unused_variables)]
#![deny(warnings)]
fn main() {
    #[allow(non_snake_case)]
    fn _doctest_main_library_alloc_src_collections_btree_set_rs_35_0() {
        use std::collections::BTreeSet;

        // Type inference lets us omit an explicit type signature (which
        // would be `BTreeSet<&str>` in this example).
        let mut books = BTreeSet::new();

        // Add some books.
        books.insert("A Dance With Dragons");
        books.insert("To Kill a Mockingbird");
        books.insert("The Odyssey");
        books.insert("The Great Gatsby");

        // Check for a specific one.
        if !books.contains("The Winds of Winter") {
            println!(
                "We have {} books, but The Winds of Winter ain't one.",
                books.len()
            );
        }

        // Remove a book.
        books.remove("The Odyssey");

        // Iterate over everything.
        for book in &books {
            println!("{}", book);
        }
    }
    _doctest_main_library_alloc_src_collections_btree_set_rs_35_0()
}
