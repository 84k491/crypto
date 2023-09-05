use crate::signal::Side;
use crate::mock_trading_gateway::MockTradingGateway;

mod binance_downloader;
mod visualizer;
mod candle;
mod signal;
mod strategy;
mod mock_trading_gateway;
mod order;

static SYMBOL: &str = "BTCUSDT";

fn main() {
    let from = chrono::NaiveDate::from_ymd_opt(2023, 3, 1).expect("Bad hardcode");
    let to = chrono::NaiveDate::from_ymd_opt(2023, 4, 1).expect("Bad hardcode");

    let mut trgw = MockTradingGateway::new();

    let init_depo = trgw.get_depo();
    {
        let mut strategy = strategy::Strategy::new(&mut trgw);

        binance_downloader::process_prices(
            SYMBOL.to_owned(), from, to,
            |c| { strategy.push_candle(&c); });

        let mut vis = visualizer::Visualizer::new(SYMBOL, strategy.get_candles());
        vis.set_signals(strategy.get_signals());
        vis.set_additional_1(strategy.get_slow_sma_results().clone());
        vis.set_additional_2(strategy.get_quick_sma_results().clone());
        vis.draw();
    }
    trgw.close_position();
    let final_depo = trgw.get_depo();

    let procent_delta = 100.0f32 * (final_depo - init_depo) / init_depo;
    println!("Depo changed for {}%. Init: {}, final: {}", procent_delta, init_depo, final_depo);
    println!("Won: {}({}%), lost: {}",
        trgw.get_won_positions(),
        (100 * trgw.get_won_positions()) / (trgw.get_won_positions() + trgw.get_lost_positions()),
        trgw.get_lost_positions());

    println!("Done");
}
