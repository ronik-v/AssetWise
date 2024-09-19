use egui::emath::round_to_decimals;
use crate::core::utils::stat_functions::{mean, std};
use crate::core::utils::states::States;

pub struct StrategyMetadata {
    pub close_prices: Vec<f64>,
    pub trade_states: Vec<States>
}

impl StrategyMetadata {
    pub fn new(close_prices: Vec<f64>, trade_states: Vec<States>) -> Self { Self { close_prices, trade_states } }

    pub fn volatile(&self) -> (f64, f64) {
        let mean = mean(&self.close_prices);
        let std = std(&self.close_prices);

        (round_to_decimals(mean, 2), round_to_decimals((std / mean) * 100.0, 2))
    }

    pub fn income(&self) -> f64 {
        let mut income: f64 = 0.0;
        let mut last_buy_price: Option<f64> = None;
        let states_split = (self.close_prices.len() as i32 - self.trade_states.len() as i32).abs() as usize;

        for t in 0..self.trade_states.len() {
            match self.trade_states[t] {
                States::BUY => {
                    if last_buy_price.is_none() {
                        last_buy_price = Some(self.close_prices[t + states_split]);
                    }
                }
                States::SELL => {
                    if let Some(buy_price) = last_buy_price {
                        income += self.close_prices[t + states_split] - buy_price;
                        last_buy_price = None;
                    }
                }
                States::WAIT => {}
            }
        }

        round_to_decimals((income / mean(&self.close_prices)) * 100.0, 2)
    }
}