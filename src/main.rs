use std::{fmt::Display, fs, io::BufRead};

use book::Book;
use types::Side;

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
*/

#[derive(Debug)]
struct Info {
    name: String,
    age: i32
}

impl Display for Info {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        write!(f, "{} - {}", self.name, self.age)
    }
}

fn main()
{
    let i = Info {name: "zs".to_string(), age: 12};
    println!("{}", i);
}

fn main1() {
    if let Ok(lines) = read_lines("files/order.csv") {
        for line in lines {
            if let Ok(row) = line {
                let items: Vec<&str> = row.split(',').collect();
                let time = items[1].parse::<i64>().unwrap();
                let id = items[2].parse::<i64>().unwrap();
                let side = if items[3] == "BUY" {Side::BID} else {Side::ASK};
                let price = items[4].parse::<f64>().unwrap();
                let qty = items[5].parse::<i64>().unwrap();

                let order = Order{
                    time: time,
                    id: id,
                    side: side,
                    price: price,
                    qty: qty
                };

                println!("{:?}", order)
            }
        }
    }
    
}

fn read_lines<P>(filepath: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where P: AsRef<std::path::Path>, {
    let file = std::fs::File::open(filepath).expect("open file failed.");
    Ok(std::io::BufReader::new(file).lines())
}
