use std::collections::LinkedList;
use crate::candle::Candle;

mod binance_downloader;
mod visualizer;
mod candle;

static OUT_FILE_NAME: &str = "plot.bmp";
static SYMBOL: &str = "BTCUSDT";

fn main() {
    let from = chrono::NaiveDate::from_ymd_opt(2023, 4, 1).expect("Bad hardcode");
    let to = chrono::NaiveDate::from_ymd_opt(2023, 5, 1).expect("Bad hardcode");

    let mut candles = LinkedList::<Candle>::new();
    binance_downloader::process_prices(SYMBOL.to_owned(), from, to, |c| { candles.push_back(c); });

    let vis = visualizer::Visualizer::new(SYMBOL, &candles);
    vis.draw();

    println!("Exit");
}
