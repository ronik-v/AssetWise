#![allow(warnings)]
mod data;
mod utils;
mod models;
mod signals;
mod trade_report;

use std::io::{self, Write};
use std::string::String;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::Local;
use figlet_rs::FIGfont;
use crate::data::moex_parser::Ticker;

use crate::trade_report::Logger;
use crate::models::arima::Arima;
use crate::models::sma::Sma;
use crate::models::adx::Adx;
use crate::signals::arima_signals::trade_signal_arima;
use crate::signals::sma_signals::trade_signal_sma;
use crate::signals::adx_signals::trade_signal_adx;


fn trade_robot(ticker: Arc<String>, data: Ticker, logger: Arc<Mutex<Logger>>) {
    let ticker_data = data;
    let price_data = ticker_data.close.clone();

    // Use models for getting trade states
    let arima = Arima { price_data: price_data.clone() };
    let prediction = arima.model_prediction_time_series();

    let sma5 = Sma::new(price_data.clone(), 5);
    let sma5_time_series = sma5.values();

    let sma12 = Sma::new(price_data, 12);
    let sma12_time_series = sma12.values();

    let adx = Adx::new(ticker_data, 5);
    let adx_values = adx.adx();
    let di_plus = adx.directional_indicators(true);
    let di_minus = adx.directional_indicators(false);


    // State predictions
    let arima_state = trade_signal_arima(prediction);
    let sma_state = trade_signal_sma(sma5_time_series, sma12_time_series);
    let adx_fast_state = trade_signal_adx(di_plus.clone(), di_minus.clone(), adx_values.clone(), true);
    let adx_long_state = trade_signal_adx(di_plus.clone(), di_minus.clone(), adx_values.clone(), false);

    // Logging states
    println!("====================================================================");
    logger.lock().unwrap().log_state(&ticker, arima_state, "ARIMA strategy".to_string());
    logger.lock().unwrap().log_state(&ticker, sma_state, "SMA strategy".to_string());
    logger.lock().unwrap().log_state(&ticker, adx_fast_state, "ADX fast strategy".to_string());
    logger.lock().unwrap().log_state(&ticker, adx_long_state, "ADX long strategy".to_string());
    println!("====================================================================");
}

fn main() {
    // Console title text
    let standard_font = FIGfont::standard().unwrap();
    let trade_robot_text = standard_font.convert("TRADE ROBOT (ARIMA/SMA/ADX)");
    let author_text = standard_font.convert("Author: ronik-v");
    let license_text = standard_font.convert("License: MIT");
    println!("{}", trade_robot_text.unwrap());
    println!("{}", author_text.unwrap());
    println!("{}", license_text.unwrap());

    let logger = Arc::new(Mutex::new(Logger::new()));

    let tickers = Arc::new(Mutex::new(Vec::new()));
    let tickers_clone = Arc::clone(&tickers);
    let logger_clone = Arc::clone(&logger);

    thread::spawn(move || {
        loop {
            // Ticker input
            let mut ticker = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ticker).unwrap();
            let ticker = ticker.trim().to_string();
            let ticker = Arc::new(ticker);

            tickers_clone.lock().unwrap().push(ticker.clone());

            let logger = Arc::clone(&logger_clone);

            thread::spawn(move || {
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
                            trade_robot(ticker.clone(), data, Arc::clone(&logger));
                        },
                        Err(_) => {},
                    }
                    // Sleep
                    thread::sleep(Duration::from_secs(60));
                }
            });
        }
    });
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
