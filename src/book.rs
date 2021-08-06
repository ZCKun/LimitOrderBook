use std::{any::Any, collections::BTreeMap, fmt::{Display, format}};

use crate::types::{Order, Trade};
#[derive(Debug)]
pub struct Book {
    bids_id: BTreeMap<i64, Order>,
    asks_id: BTreeMap<i64, Order>,
    // Key: price, Value: all ids with the same price
    bids_price: BTreeMap<i64, Vec<i64>>,
    asks_price: BTreeMap<i64, Vec<i64>>,
}

impl Display for Book {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut asks = BTreeMap::<i64, i64>::new();
        for (_, order) in &self.asks_id {
            let price: i64 = (order.price * 100.0) as i64;
            match asks.get_mut(&price) {
                Some(qty) => { 
                    *qty += order.qty;
                },
                None => {
                    asks.insert(price, order.qty);
                }
            }
        }

        let mut bids = BTreeMap::<i64, i64>::new();
        for (_, order) in &self.bids_id {
            let price: i64 = (order.price * 100.0) as i64;
            match bids.get_mut(&price) {
                Some(qty) => { 
                    *qty += order.qty;
                },
                None => {
                    bids.insert(price, order.qty);
                }
            }
        }

        let mut msg = String::new();
        for (price, qty) in asks.iter().rev() {
            let p = *price as f64;
            msg = format!("{}\nask | {:.2} | {}", msg, p / 100.0, qty);
        }
        msg = format!("{}\n-----------------", msg);
        for (price, qty) in bids.iter().rev() {
            let p = *price as f64;
            msg = format!("{}\nbid | {:.2} | {}", msg, p / 100.0, qty);
        }

        write!(f, "{}", msg)
    }
}

impl Book {
    pub fn new() -> Book {
        Self {
            bids_id: BTreeMap::<i64, Order>::new(),
            asks_id: BTreeMap::<i64, Order>::new(),
            bids_price: BTreeMap::<i64, Vec<i64>>::new(),
            asks_price: BTreeMap::<i64, Vec<i64>>::new(),
        }
    }

    pub fn add_order(&mut self, order: &Order) {
        // 不处理非限价单
        if order.price == 0.0 { return }

        let price = (order.price * 100.0) as i64;

        match order.side {
            crate::types::Side::ASK => {
                self.asks_id.insert(order.id, *order);
                match self.asks_price.get_mut(&price) {
                    Some(v) => {
                        v.push(order.id);
                    }
                    None => {
                        self.asks_price.insert(price, vec![order.id]);
                    }
                }
            }
            crate::types::Side::BID => {
                self.bids_id.insert(order.id, *order);
                match self.bids_price.get_mut(&price) {
                    Some(v) => {
                        v.push(order.id);
                    }
                    None => {
                        self.bids_price.insert(price, vec![order.id]);
                    }
                }
            }
        }
    }

    pub fn on_trade(&mut self, trade: &Trade) {
        match trade.trade_type {
            crate::types::TradeType::TRADED => self.on_traded(trade),
            crate::types::TradeType::CACNEL => self.on_cancel(trade),
        }
    }

    #[inline]
    fn on_traded(&mut self, trade: &Trade) {
        self.bids_id.retain(|order_id, order| *order_id >= trade.bid_id && order.price <= trade.price);
        self.asks_id.retain(|order_id, order| *order_id >= trade.ask_id && order.price >= trade.price);

        /*
        for (order_id, order) in self.bids_id.iter_mut() {
            let trade_id = &trade.bid_id;
            let mut rm_order_id  = 0i64;

            if order_id == trade_id {
                if trade.qty >= order.qty {
                    rm_order_id = *order_id;
                } else {
                    order.qty -= trade.qty;
                }
            } else if order.price >= order.price {
                rm_order_id = *order_id;
            }

            self.bids_id.remove(&rm_order_id);
        }*/

        match self.bids_id.get_mut(&trade.bid_id) {
            Some(order) => {
                if order.qty <= trade.qty {
                    self.bids_id.remove(&trade.bid_id);
                } else {
                    order.qty -= trade.qty;
                }
            }
            None => {
                // println!("not found order id:{}", trade.bid_id);
            }
        }

        match self.asks_id.get_mut(&trade.ask_id) {
            Some(order) => {

                if order.qty <= trade.qty {
                    self.asks_id.remove(&trade.ask_id);
                } else {
                    order.qty -= trade.qty;
                }
            }
            None => {
                // println!("not found order id:{}", trade.ask_id);
            }
        }
    }

    #[inline]
    fn on_cancel(&mut self, trade: &Trade) {
        if trade.bid_id != 0 {
            self.bids_id.remove(&trade.bid_id);
        } else if trade.ask_id != 0 {
            self.asks_id.remove(&trade.ask_id);
        }
    }
}

#[cfg(test)]
mod BookTest {
    use crate::book::*;
    use crate::types::{Order, Side, Trade};

    #[test]
    fn test_add() {
        let mut book = Book::new();

        let bid_order = Order {
            time: 0,
            id: 1,
            price: 3.14,
            qty: 100,
            side: Side::BID,
        };

        let ask_order = Order {
            time: 0,
            id: 2,
            price: 3.14,
            qty: 100,
            side: Side::ASK,
        };

        book.add_order(&bid_order);
        book.add_order(&ask_order);

        assert_eq!(book.bids_id.len(), 1);
        assert_eq!(book.asks_id.len(), 1);
    }

    #[test]
    fn test_traded() {
        let mut book = Book::new();

        let bid_order = Order {
            time: 0,
            id: 1,
            price: 3.14,
            qty: 100,
            side: Side::BID,
        };

        let ask_order = Order {
            time: 0,
            id: 2,
            price: 3.13,
            qty: 50,
            side: Side::ASK,
        };
                
        let ask_order2 = Order {
            time: 0,
            id: 3,
            price: 3.12,
            qty: 100,
            side: Side::ASK,
        };

        book.add_order(&bid_order);
        book.add_order(&ask_order);
        book.add_order(&ask_order2);

        let bid_trade = Trade {
            time: 0,
            id: 3,
            ask_id: 3,
            bid_id: 1,
            price: 3.13,
            qty: 100,
            trade_type: crate::types::TradeType::TRADED,
        };
        book.on_trade(&bid_trade);

        assert_eq!(book.bids_id.len(), 0);
        assert_eq!(book.asks_id.len(), 0);
    }

    #[test]
    fn test_cancel() {
        let mut book = Book::new();

        let bid_order = Order {
            time: 0,
            id: 1,
            price: 3.14,
            qty: 100,
            side: Side::BID,
        };

        let ask_order = Order {
            time:0,
            id: 2,
            price: 3.13,
            qty: 100,
            side: Side::ASK,
        };

        book.add_order(&bid_order);
        book.add_order(&ask_order);

        let bid_trade = Trade {
            time: 0,
            id: 3,
            ask_id: 0,
            bid_id: 1,
            price: 3.13,
            qty: 100,
            trade_type: crate::types::TradeType::CACNEL,
        };
        book.on_cancel(&bid_trade);

        assert_eq!(book.bids_id.len(), 0);
        assert_eq!(book.asks_id.len(), 1);
    }
}
