use std::error::Error;
use std::sync::Arc;
use egui::TextBuffer;
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

    if !data.is_empty() {
        Ok(prepare_data_structure(&data))
    } else {
        Err("По данной компании нет данных".into())
    }
}

pub fn get_ticker_by_company_name(company_name: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://iss.moex.com/iss/securities.json?q={}", company_name);
    let client = Client::new();
    let response = client.get(&url).send()?;
    let response_body = response.text()?;

    let json: SerdeValue = serde_json::from_str(&response_body)?;
    let data = &json["securities"]["data"];

    for item in data.as_array().ok_or("Ошибка формата данных")? {
        if let Some(list) = item.as_array() {
            if list.contains(&SerdeValue::String("common_share".to_string())) {
                if let Some(ticker) = list.get(1) {
                    return Ok(ticker.as_str().unwrap_or("").to_string());
                }
            }
        }
    }

    Err("Тикер не найден".into())
}