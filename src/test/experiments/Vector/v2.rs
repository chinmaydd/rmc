#![feature(rustc_private)]
include!{"../../rmc-prelude.rs"}

fn main() {
    let mut v1 = RmcVec::new();
    v1.push(1);
    v1.push(0);
    v1.push(2);
    v1.sort_unstable();
    assert!(v1[2] == 2);
    assert!(v1.len() == 3);
}
