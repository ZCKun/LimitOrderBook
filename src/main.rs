use std::{fmt::Display, fs, io::BufRead};

use book::Book;
use types::Side;

use crate::types::{Order, Trade, TradeType};

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

fn main() {

    let mut book = book::Book::new();

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

                book.add_order(&order);
            }
        }
    }

    if let Ok(lines) = read_lines("files/trade.csv") {
        for line in lines {
            if let Ok(row) = line {
                let items: Vec<&str> = row.split(',').collect();
                let time = items[1].parse::<i64>().unwrap();
                let id = items[2].parse::<i64>().unwrap();
                let bid_id = items[3].parse::<i64>().unwrap();
                let ask_id = items[4].parse::<i64>().unwrap();
                let price = items[6].parse::<f64>().unwrap();
                let qty = items[7].parse::<i64>().unwrap();

                let trade_type = if ask_id != 0 && bid_id != 0 { TradeType::TRADED } else { TradeType::CACNEL };

                let trade = Trade{
                    time: time,
                    id: id,
                    price: price,
                    qty: qty,
                    ask_id : ask_id,
                    bid_id: bid_id,
                    trade_type: trade_type
                };

                book.on_trade(&trade);

                println!("{}", book)
            }
        }
    }

    println!("process done!")
}

fn read_lines<P>(filepath: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where P: AsRef<std::path::Path>, {
    let file = std::fs::File::open(filepath).expect("open file failed.");
    Ok(std::io::BufReader::new(file).lines())
}
