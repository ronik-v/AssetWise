use std::sync::Arc;
use reqwest::blocking::Client;
use serde_json::Value as SerdeValue;
use crate::core::data::moex_parser::{api_url, prepare_data_structure, Ticker};


pub fn get_ticker_data(ticker: Arc<String>, date_start: String, date_end: String, interval: u32) -> Result<Ticker, Box<dyn std::error::Error>> {
    let api_data_url = api_url(ticker, date_start, date_end, interval);
    let client = Client::new();
    let response = client.get(&api_data_url).send()?;
    let response_body = response.text()?;

    let json: SerdeValue = serde_json::from_str(&response_body)?;
    let data: Vec<Vec<SerdeValue>> = serde_json::from_value(json["candles"]["data"].clone())?;

    Ok(prepare_data_structure(&data))
}