use std::error::Error;
use reqwest::blocking::get;
use scraper::{Html, Selector};

use crate::core::data::moex_parser::Ticker;
use crate::core::utils::states::Utility;
use crate::core::utils::stat_functions::std;
use crate::core::utils::stat_functions::beta;

// TODO: Need to refactor

pub struct UtilityFunction {
    pub ticker: Ticker,
    pub risk_value: f64
}

impl UtilityFunction {
    pub fn new(ticker: Ticker, risk_value: f64) -> Self {
        Self { ticker, risk_value }
    }

    fn bound_rate(&self) -> Result<f64, Box<dyn Error>> {
        let url = "https://www.cbr.ru/hd_base/zcyc_params/zcyc/";
        let response = get(url)?.text()?;
        let document = Html::parse_document(&response);
        let td_selector = Selector::parse("td").unwrap();

        let mut sum = 0.0;
        let mut count = 0;

        for element in document.select(&td_selector) {
            if element.value().attr("class").is_none() {
                let text = element.text().collect::<Vec<_>>().concat();
                if let Ok(num) = text.parse::<f64>() {
                    sum += num;
                    count += 1;
                }
            }
        }

        if count > 0 {
            Ok(sum / count as f64)
        } else {
            Err("Numbers for calculate mean not found.".into())
        }
    }

    fn ticker_income(&self) -> Vec<f64> {
        let mut income: Vec<f64> = Vec::new();
        for i in 0..self.ticker.open.len() {
            income.push(self.ticker.close[i] - self.ticker.open[i]);
        }

        income
    }

    fn capm_income(&self, ticker_income: Vec<f64>, rate: f64, beta: f64) -> f64 {
        let mut income_sum: f64 = 0.0;
        for i in 0..ticker_income.len() {
            income_sum += ticker_income[i];
        }
        rate + beta * (income_sum - rate)
    }

    fn utility_state(&self, income_sum: f64, volatility: f64) -> Utility {
        let utility_value: f64 = income_sum - 0.5 * self.risk_value * volatility;

        return if utility_value >= 1.0 {
            Utility::HOLD
        } else if utility_value < 1.0 && utility_value > 0.0 {
            Utility::EXPECT
        } else {
            Utility::ESCAPE
        }
    }

    // TODO: realise result function -> U = E(R) - 0.5 * A * q^2  || U >= 1 \ 0 < U < 1 | U < 0
    pub fn result(&self) -> Utility {
        let bound_rate: Result<f64, Box<dyn Error>> = self.bound_rate(); // https://www.cbr.ru/hd_base/zcyc_params/zcyc/
        match bound_rate {
            Ok(rate) => {
                println!("BOUND MEAN RATE = {:?}", rate);
                let ticker_income: Vec<f64> = self.ticker_income();
                let beta: f64 = beta(&ticker_income.clone(), &self.ticker.close.clone());
                let volatility: f64 = std(&ticker_income);
                let camp_income: f64 = self.capm_income(ticker_income, rate, beta);

                self.utility_state(camp_income, volatility)
            }
            Err(e) => { panic!("Utility calc error {:?}", e) }
        }

    }
}