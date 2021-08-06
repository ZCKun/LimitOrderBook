#[derive(Debug, Copy, Clone)]
pub struct Price {
    var: i64
}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.var.cmp(&other.var) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.var == other.var
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Price {
    
}
    

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Side {
    BID,
    ASK,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TradeType {
    TRADED,
    CACNEL,
}

#[derive(Debug, Copy, Clone)]
pub struct Order {
    pub time: i64,
    pub id: i64,
    pub price: f64,
    pub qty: i64,
    pub side: Side,
}

impl Order {
    fn new(time: i64, id: i64, price: f64, qty: i64, side: Side) -> Order {
        Self {
            time: time,
            id: id,
            price: price,
            qty: qty,
            side: side,
        }
    }

    pub fn is_sell(&self) -> bool {
        self.side == Side::ASK
    }

    pub fn is_buy(&self) -> bool {
        self.side == Side::BID
    }
}

pub struct Trade {
    pub time: i64,
    pub id: i64,
    pub price: f64,
    pub qty: i64,
    pub ask_id: i64,
    pub bid_id: i64,
    pub trade_type: TradeType
}

impl Trade {
    pub fn buy_trade(&self) -> bool {
        self.ask_id > self.bid_id
    }

    pub fn sell_trade(&self) -> bool {
        self.bid_id > self.ask_id
    }
}
