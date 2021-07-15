// Abstraction which tracks the length and the last element of the vector
struct RmcLenVec<T> {
    len: usize,
    last: Option<T>,
}

impl<T: Copy> RmcLenVec<T> {
    fn drain(&mut self) {
        self.len = 0;
        self.last = None;
    }

    fn update_last(&mut self, elem: Option<T>) {
        self.last = elem;
    }

    pub fn new() -> RmcLenVec<T> {
        RmcLenVec {
            len: 0,
            last: None
        }
    }

    pub fn with_capacity(_: usize) -> Self {
        RmcLenVec {
            len: 0,
            last: None
        }
    }

    pub fn push(&mut self, elem: T) {
        self.len += 1;
        self.last = Some(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            self.last
        }
    }

    pub fn append(&mut self, other: &mut RmcLenVec<T>) {
        if other.len() != 0 {
            self.update_last(other.pop());
            self.len += other.len();
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len);

        if index == self.len {
            self.push(elem);
        } else {
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);

        if index == self.len - 1 {
            panic!("This operation is unsound for the current abstraction!");
        } else {
            self.pop().unwrap()
        }
    }

    pub fn extend<I: Iterator>(&mut self, iter: I) where I: Iterator<Item = T> {
        let mut last = None;
        for value in iter {
            self.len += 1;
            last = Some(value);
        }
        self.update_last(last);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn reserve(&mut self, _: usize) {
    }

    pub fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }
}

impl<T> Drop for RmcLenVec<T> {
    fn drop(&mut self) {
    }
}

// NOTE: To implement:
// Iterators
// PartialEq

fn main() {}
