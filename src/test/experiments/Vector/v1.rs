include!{"../rmc-prelude.rs"}

fn main() {
    let mut v = RmcVec::new();
    v.push(10);
    assert!(v.len() == 1);
}
