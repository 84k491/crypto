#[derive(Clone, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug)]
pub struct Signal {
    pub ts: u64,
    pub price: f32,
    pub side: Side,
}
