// TODO: Реалировать временного тоггового робота для ARIMA и SMA и теогетико игровой стартегии
// TODO: Демон процесс который проверяет набор акций
mod data;
mod utils;
mod models;
pub mod oracle;
mod signals;

fn main() {
    let ticker = "SBER";
    let date_start = "2024-05-01";
    let date_end = "2024-06-07";
    let interval = 1;
    match data::moex_parser::get_ticker_data(ticker, date_start, date_end, interval) {
        Ok(ticker_data) => println!("{:#?}", ticker_data),
        Err(e) => eprintln!("Error fetching data: {}", e),
    }

}
