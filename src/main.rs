mod binance_downloader;
mod visualizer;
mod candle;
mod signal;
mod strategy;

static SYMBOL: &str = "BTCUSDT";

fn main() {
    let from = chrono::NaiveDate::from_ymd_opt(2023, 4, 1).expect("Bad hardcode");
    let to = chrono::NaiveDate::from_ymd_opt(2023, 5, 1).expect("Bad hardcode");

    let mut strategy = strategy::Strategy::new();
    binance_downloader::process_prices(
        SYMBOL.to_owned(), from, to, 
        |c| { strategy.push_candle(&c); });

    let mut vis = visualizer::Visualizer::new(SYMBOL, strategy.get_candles());
    vis.set_signals(strategy.get_signals());
    vis.set_additional_1(strategy.get_slow_sma_results().clone());
    vis.set_additional_2(strategy.get_quick_sma_results().clone());
    vis.draw();

    println!("Exit");
}
