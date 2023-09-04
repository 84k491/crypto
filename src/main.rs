use crate::signal::Side;

mod binance_downloader;
mod visualizer;
mod candle;
mod signal;
mod strategy;

static SYMBOL: &str = "BTCUSDT";

fn trade_and_get_currency_delta(pos: &mut f32, vol_delta: f32, price: f32) -> f32 {
        *pos += vol_delta;
        let currency_delta = vol_delta * price as f32 * -1.0f32;
        return currency_delta;
}

fn main() {
    let from = chrono::NaiveDate::from_ymd_opt(2023, 3, 1).expect("Bad hardcode");
    let to = chrono::NaiveDate::from_ymd_opt(2023, 4, 1).expect("Bad hardcode");

    let mut strategy = strategy::Strategy::new();
    binance_downloader::process_prices(
        SYMBOL.to_owned(), from, to,
        |c| { strategy.push_candle(&c); });

    let mut vis = visualizer::Visualizer::new(SYMBOL, strategy.get_candles());
    vis.set_signals(strategy.get_signals());
    vis.set_additional_1(strategy.get_slow_sma_results().clone());
    vis.set_additional_2(strategy.get_quick_sma_results().clone());

    let init_currency = 1000.0f32;
    let mut currency_amount = init_currency;
    let mut position = 0.0f32;
    let vol_limit = 0.1f32;

    let mut last_price = 0.0f32;
    strategy.get_signals().iter().for_each(|s| {
        let sign = match s.side {
            Side::Buy => 1,
            Side::Sell => -1,
        };
        let desired_pos = vol_limit * sign as f32;
        let vol_delta = desired_pos - position;
        currency_amount += trade_and_get_currency_delta(&mut position, vol_delta, s.price);
        last_price = s.price;
    });
    let delta_to_zero = position.clone() * -1.0f32;
    currency_amount += trade_and_get_currency_delta(&mut position, delta_to_zero, last_price);

    let procent_delta = 100.0f32 * (currency_amount - init_currency) / init_currency;
    println!("Depo changed for {}%. Init: {}, final: {}", procent_delta, init_currency, currency_amount);

    vis.draw();

    println!("Exit");
}
