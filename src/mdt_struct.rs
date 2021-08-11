#[repr(align(1))]
#[derive(Copy, Clone, Debug)]
pub struct SzL2Order {
    rec_time: u64,  // 8
    channel_no: u32, // 4
    order_id: u64,  // 8
    symbol_code: [char; 40],  // 40
    symbol_source: [char; 5],  // 5
    time: i64,  // 8
    price: f64,  // 8
    qty: f64,  // 8
    code: char,  // 1
    r#type: char  // 1
}