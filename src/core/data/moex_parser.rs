use std::sync::Arc;
use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Debug)]
pub struct Ticker {
    // Ticker data from MOEX API
    pub open: Vec<f64>,
    pub close: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub value: Vec<f64>,
    pub volume: Vec<i64>,
    pub begin: Vec<String>,
    pub end: Vec<String>,
}

pub fn api_url(ticker: Arc<String>, date_start: String, date_end: String, interval: u32) -> String {
    // Prepare url for api request
    let api_prefix = "https://iss.moex.com/iss/engines/stock/markets/shares/securities/";
    let json_format_data_piece = "/candles.json";
    let date_from = "?from=";
    let date_till = "&till=";
    let data_interval = "&interval=";

    format!(
        "{}{}{}{}{}{}{}{}{}",
        api_prefix, ticker, json_format_data_piece, date_from, date_start, date_till, date_end, data_interval, interval
    )
}

pub fn prepare_data_structure(data: &[Vec<Value>]) -> Ticker {
    Ticker {
        open: data.iter().map(|v| v[0].as_f64().unwrap_or(0.0)).collect(),
        close: data.iter().map(|v| v[1].as_f64().unwrap_or(0.0)).collect(),
        high: data.iter().map(|v| v[2].as_f64().unwrap_or(0.0)).collect(),
        low: data.iter().map(|v| v[3].as_f64().unwrap_or(0.0)).collect(),
        value: data.iter().map(|v| v[4].as_f64().unwrap_or(0.0)).collect(),
        volume: data.iter().map(|v| v[5].as_i64().unwrap_or(0)).collect(),
        begin: data.iter().map(|v| v[6].as_str().unwrap_or("").to_string()).collect(),
        end: data.iter().map(|v| v[7].as_str().unwrap_or("").to_string()).collect(),
    }
}

pub fn get_ticker_data(ticker: Arc<String>, date_start: String, date_end: String, interval: u32) -> Result<Ticker, Box<dyn std::error::Error>> {
    // Getting data structure with api
    let api_data_url = api_url(ticker, date_start, date_end, interval);
    let client = Client::new();
    let response = client.get(&api_data_url).send()?;
    let response_body = response.text()?;

    let json: Value = serde_json::from_str(&response_body)?;
    let data: Vec<Vec<Value>> = serde_json::from_value(json["candles"]["data"].clone())?;

    Ok(prepare_data_structure(&data))
}
