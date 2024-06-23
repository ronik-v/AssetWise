/*
    Info from - https://ru.wikipedia.org/wiki/%D0%A1%D0%B8%D1%81%D1%82%D0%B5%D0%BC%D0%B0_%D0%BD%D0%B0%D0%BF%D1%80%D0%B0%D0%B2%D0%BB%D0%B5%D0%BD%D0%BD%D0%BE%D0%B3%D0%BE_%D0%B4%D0%B2%D0%B8%D0%B6%D0%B5%D0%BD%D0%B8%D1%8F
    Average Directional Movement Index, ADX.

    For alternative positions:
        Buy if +DI > -DI and ADX is rising.
        Sell if +DI < -DI or ADX falls.
    For the presence of positions:
        Sell short if +DI < -DI and ADX is rising.
        Close your short position if +DI > -DI or ADX falls.
*/
use crate::data::moex_parser::Ticker;
use crate::models::ema::Ema;
use crate::models::ma::MA;

pub struct Adx {
    pub ticker: Ticker,
    pub split: i32
}

impl Adx {
    pub fn new(ticker: Ticker, split: i32) -> Self {
        Self { ticker, split }
    }

    fn movement(&self, option: bool) -> Vec<f64> {
        // {For ±DM} - if option is true we calc (+M[t]) else calc (-M[t])
        let mut movement_data: Vec<f64> = Vec::new();
        if option {
            for i in 1..self.ticker.high.len() {
                let value: f64 = self.ticker.high[i] - self.ticker.high[i - 1];
                movement_data.push(value);
            }
        } else {
            for i in 1..self.ticker.low.len() {
                let value: f64 = self.ticker.low[i - 1] - self.ticker.low[i];
                movement_data.push(value);
            }
        }
        movement_data
    }

    fn directional_movement(&self, option: bool) -> Vec<f64> {
        let mut directional_movement_data: Vec<f64> = Vec::new();
        let movement_plus: Vec<f64> = self.movement(true);
        let movement_minus: Vec<f64> = self.movement(false);
         if option {
             for i in 0..movement_plus.len() {
                 if movement_plus[i] > movement_minus[i] && movement_plus[i] > 0.0 {
                     directional_movement_data.push(movement_plus[i]);
                 } else {
                     directional_movement_data.push(0.0);
                 }
             }
         } else {
             for i in 0..movement_minus.len() {
                 if movement_minus[i] < movement_plus[i] || movement_minus[i] < 0.0 {
                     directional_movement_data.push(0.0);
                 } else {
                     directional_movement_data.push(movement_minus[i]);
                 }
             }
         }

        directional_movement_data
    }
    
    fn true_range(&self) -> Vec<f64> {
        // Demonstrates the maximum dispersion of prices for transactions since the close of the previous period
        let mut true_range_values: Vec<f64> = Vec::new();
        for i in 1..self.ticker.open.len() {
            let value: f64 = self.ticker.high[i].max(self.ticker.close[i -1]) - self.ticker.low[i].min(self.ticker.close[i - 1]);
            true_range_values.push(value);
        }
        true_range_values
    }

    pub fn directional_indicators(&self, option: bool) -> Vec<f64> {
        let mut directional_indicators_data: Vec<f64> = Vec::new();
        let true_range: Vec<f64> = self.true_range();
        let directional_movement_plus: Vec<f64> = self.directional_movement(option);
        for i in 0..directional_movement_plus.len() {
            let value: f64 = directional_movement_plus[i] / true_range[i];
            directional_indicators_data.push(value);
        }
        // calc exponential mean average
        let ema: Ema = Ema::new(directional_indicators_data);

        ema.values()
    }

    pub fn adx(&self) -> Vec<f64> {
        // Average directional movement index function
        let directional_indicators_data: Vec<f64> = self.directional_indicators(true);
        let ma = MA::new(14, directional_indicators_data); // classic lag value
        ma.values()
    }
}
