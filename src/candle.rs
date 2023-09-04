use serde::Deserialize;

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct Candle {
    pub open_ts: u64,
    pub open_price: f32,
    pub high_price: f32,
    pub low_price: f32,
    pub close_price: f32,
    volume: f32,
    pub close_ts: u64,
    qa_vol: f32,
    trades_num: u64,
    tbba_volume: f32,
    tbqa_volume: f32,
    ignore: u64,
}

