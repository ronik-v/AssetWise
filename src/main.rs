mod data;
mod utils;
mod models;
mod signals;
mod trade_report;
pub mod oracle;

use chrono::Local;
use std::io::{self, Write};
use std::string::String;
use std::thread;
use std::time::Duration;
use crate::models::arima::Arima;
use crate::models::sma::Sma;
use crate::signals::arima_signals::trade_signal_arima;
use crate::signals::sma_signals::trade_signal_sma;

fn main() {
    let mut ticker = String::new();
    print!("Enter your ticker >> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ticker).unwrap();
    let ticker = ticker.trim();

    // Today
    let today = Local::now().date_naive();

    // String formatting to template - "yyyy-mm-dd"
    let date_start = "2024-06-07".to_string(); // today.format("%Y-%m-%d").to_string();
    let date_end = "2024-06-07".to_string(); // today.format("%Y-%m-%d").to_string();
    let interval = 1;

    loop {
        let ticker_data = data::moex_parser::get_ticker_data(ticker, date_start.clone(), date_end.clone(), interval);
        match ticker_data {
            Ok(data) => {
                let price_data = data.close;
                // Use models for getting trade states
                let arima = Arima { price_data: price_data.clone() };
                let prediction = arima.model_prediction_time_series();
                // Arima predictions
                //println!("ARIMA model prediction time series: {:?}", prediction);

                let sma5 = Sma { data: price_data.clone(), split: 5 };
                let sma5_time_series = sma5.values();

                let sma12 = Sma { data: price_data, split: 12 };
                let sma12_time_series = sma12.values();
                //print!("SMA5: {:?}", sma5_time_series);
                //print!("SMA12: {:?}", sma12_time_series);

                // State predictions
                let arima_state = trade_signal_arima(prediction);
                let sma_state = trade_signal_sma(sma5_time_series, sma12_time_series);
                // Logging states
                trade_report::log_state(arima_state, "ARIMA strategy".to_string());
                trade_report::log_state(sma_state, "SMA strategy".to_string());
            },
            Err(e) => eprintln!("Error fetching data: {}", e),
        }
        // Sleep
        thread::sleep(Duration::from_secs(60));
    }
}
