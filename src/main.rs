use book::Book;

use crate::types::{Order, Trade};

mod book;
mod types;

/*
struct A {
    n: i32
}

impl A {
    pub fn new(number: i32) -> A {
        Self {
            n: number
        }
    }
}

// 重写比较
impl Ord for A {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.n.cmp(&other.n) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        self.n != other.n
    }
}

impl PartialOrd for A {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for A {
}

#[derive(Debug)]
struct Info {
    name: String,
    age: i32
}
*/

fn main() {

}
