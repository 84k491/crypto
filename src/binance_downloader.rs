use chrono::Datelike;
use crate::candle::Candle;
use downloader::download::Download as dld;
use downloader::downloader::Downloader as dlr;
use zip_extensions::zip_extract;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::str::FromStr;

static BASE_URL: &str = "https://data.binance.vision";
static DEFAULT_INTERVAL: &str = "1m";

static TARGET_DIR: &str = "./downloads";

fn compose_archive_string(uppercase_symbol: &str, from: &chrono::NaiveDate) -> PathBuf {
    let mut link = String::new();
    link.push_str(uppercase_symbol); link.push_str("-");
    link.push_str(DEFAULT_INTERVAL); link.push_str("-");
    link.push_str(from.year().to_string().as_str()); link.push_str("-");
    let month: String = {
        if from.month() < 10 {
            format!("0{}", from.month())
        }
        else {
            from.month().to_string()
        }
    };
    link.push_str(month.as_str());
    link.push_str(".zip");

    return PathBuf::from_str(&link).expect("Can't compose path");
}

pub fn download_csv(uppercase_symbol: &str, from: chrono::NaiveDate) -> Option<PathBuf> {
    let archive_name = compose_archive_string(uppercase_symbol, &from);
    let mut archive_buf = PathBuf::from_str(TARGET_DIR).unwrap();
    archive_buf.push(archive_name.clone());
    let mut output = archive_buf.clone();
    output.set_extension("csv");

    if output.exists() {
        println!("Using existing csv: {}", output.to_string_lossy());
        return Some(output);
    }

    let mut link = BASE_URL.to_owned();
    link.push_str("/data/spot/monthly/klines/");
    link.push_str(uppercase_symbol); link.push_str("/");
    link.push_str(DEFAULT_INTERVAL); link.push_str("/");
    link.push_str(archive_name.to_str().unwrap());

    println!("Downloading from '{}'...", link);
    let download = [dld::new(link.as_str())];
    let mut builder = dlr::builder();
    let mut downloader = builder.download_folder(Path::new(TARGET_DIR)).build().expect("Can't build a downloader");
    downloader.download(&download).expect("Can't download");

    if !archive_buf.exists() {
        println!("Archive {} doesn't exist.", archive_buf.to_string_lossy());
        return None;
    }

    let target = PathBuf::from_str(TARGET_DIR).expect("Can't inflate buf for target");
    zip_extract(&archive_buf, &target).expect("Can't extract");
    println!("Unpacked");

    if !output.exists() {
        println!("No extracted file");
        return None;
    }

    fs::remove_file(archive_buf).expect("Can't remove archive");
    return Some(output);
}

pub fn process_prices<F>(symbol: String, from: chrono::NaiveDate, to: chrono::NaiveDate, mut f: F) -> bool where
   F: FnMut(Candle) -> ()  {
    let mut year = from.year();
    let mut month = from.month();

    loop {
        if year == to.year() && month == to.month() {
            return true;
        }

        let csv = download_csv(
            symbol.as_str(),
            chrono::NaiveDate::from_ymd_opt(year.clone(), month.clone(), 1).unwrap());
        if csv.is_none() {
            return false;
        }
        let csv = csv.unwrap();
        let file = File::open(csv).expect("Can't open csv");

        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
        for res in rdr.deserialize() {
            let candle: Candle = res.expect("Can't deserialize a record");
            // println!("{:?}", candle);
            f(candle);
        }

        month = month + 1;
        if 13 == month {
            month = 1;
            year = year + 1;
        }
    }
}
