use chrono::Local;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::core::utils::states::States;

pub struct Logger {
    logs: Arc<Mutex<HashMap<Arc<String>, Vec<String>>>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            logs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn log_state(&self, ticker: &Arc<String>, state: States, strategy: String) {
        let date_time = Local::now();
        let log_file = "app_log.txt";
        let mut app_log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
            .expect("Unable to open file");
        // prepare log message
        let mut logs = self.logs.lock().unwrap();
        let log_entry = format!("[{}] Ticker: {}, State: {:?}, Strategy: {}", date_time.format("%Y-%m-%d %H:%M:%S"), ticker, state, strategy);
        let ticker_logs = logs.entry(ticker.clone()).or_insert_with(Vec::new);
        ticker_logs.push(log_entry.clone());

        // Logging in console
        println!("\n{}\n", log_entry);
        // Logging in log file
        let prepared_log = log_entry.clone() + "\n";
        app_log_file.write_all(prepared_log.as_bytes()).expect("Unable to open file");
    }
}