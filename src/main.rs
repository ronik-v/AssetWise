#![allow(warnings)]
mod data;
mod utils;
mod models;
mod signals;
mod trade_report;
pub mod oracle;

use std::io::{self, Write};
use std::string::String;
use std::thread;
use std::sync::Arc;
use std::time::Duration;

use chrono::Local;
use figlet_rs::FIGfont;
use crate::data::moex_parser::Ticker;

use crate::models::arima::Arima;
use crate::models::sma::Sma;
use crate::signals::arima_signals::trade_signal_arima;
use crate::signals::sma_signals::trade_signal_sma;

fn trade_robot(ticker: Arc<String>, data: Ticker) {
    let price_data = data.close;

    // Use models for getting trade states
    let arima = Arima { price_data: price_data.clone() };
    let prediction = arima.model_prediction_time_series();

    let sma5 = Sma::new(price_data.clone(), 5);
    let sma5_time_series = sma5.values();

    let sma12 = Sma::new(price_data, 12);
    let sma12_time_series = sma12.values();

    // State predictions
    let arima_state = trade_signal_arima(prediction);
    let sma_state = trade_signal_sma(sma5_time_series, sma12_time_series);

    // Logging states
    trade_report::log_state(&ticker, arima_state, "ARIMA strategy".to_string(), (20, 0));
    trade_report::log_state(&ticker, sma_state, "SMA strategy".to_string(), (21, 0));
}

fn main() {
    // Console title text
    let standard_font = FIGfont::standard().unwrap();
    let trade_robot_text = standard_font.convert("TRADE ROBOT (ARIMA/SMA)");
    let author_text = standard_font.convert("Author: ronik-v");
    let license_text = standard_font.convert("License: MIT");
    println!("{}", trade_robot_text.unwrap());
    println!("{}", author_text.unwrap());
    println!("{}", license_text.unwrap());

    let mut ticker = String::new();
    print!("Enter your ticker >> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ticker).unwrap();
    let ticker = ticker.trim().to_string();
    let ticker = Arc::new(ticker);

    // Today
    let today = Local::now().date_naive();

    // String formatting to template - "yyyy-mm-dd"
    let date_start = today.format("%Y-%m-%d").to_string();
    let date_end = today.format("%Y-%m-%d").to_string();
    let interval = 1;

    loop {
        let ticker_data = data::moex_parser::get_ticker_data(ticker.clone(), date_start.clone(), date_end.clone(), interval);
        match ticker_data {
            Ok(data) => {
                let ticker_clone = ticker.clone();
                let handle = thread::spawn(move || {
                    trade_robot(ticker_clone, data);
                });
                if let Err(e) = handle.join() {
                    eprintln!("Error in trade_robot thread: {:?}", e);
                }
            },
            Err(e) => eprintln!("Error fetching data: {}", e),
        }
        // Sleep
        thread::sleep(Duration::from_secs(60));
    }
}