mod binance_downloader;

fn main() {
    let from = chrono::NaiveDate::from_ymd_opt(2022, 12, 1).expect("Bad hardcode");
    let to = chrono::NaiveDate::from_ymd_opt(2023, 5, 1).expect("Bad hardcode");

    binance_downloader::process_prices(from, to, |_| { });
    println!("Exit");
}
