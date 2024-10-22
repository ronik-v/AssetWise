use crate::core::utils::stat_functions::mean;

const W: f64 = 0.1;
const A: f64 = 0.1;
const B: f64 = 0.8; // Инерция (волатильность)

pub struct GARCH {
    pub price_data: Vec<f64>, // GARCH(1, 1)
}

impl GARCH {
    pub fn new(price_data: Vec<f64>) -> Self { Self { price_data } }

    fn log_income(&self) -> Vec<f64> {
        let mut income: Vec<f64> = Vec::new();

        for i in 1..self.price_data.len() {
            let income_value: f64 = (self.price_data[i] / self.price_data[i - 1]).ln();
            income.push(income_value);
        }

        income
    }

    fn model_errors(&self, log_income: &[f64]) -> Vec<f64> {
        let mean_price: f64 = mean(log_income);

        log_income.iter().map(|&inc| inc - mean_price).collect::<Vec<f64>>()
    }

    pub fn volatile(&self) -> Vec<f64> {
        let log_income: Vec<f64> = self.log_income();
        let model_errors: Vec<f64> = self.model_errors(&log_income);

        let mut volatile_data: Vec<f64> = Vec::new();
        for i in 0..model_errors.len() {
            if i > 0 {
                let value: f64 = W + A * (model_errors[i] * model_errors[i]) + B * volatile_data[i - 1];
                volatile_data.push(value);
            } else {
                let value: f64 = W + A * (model_errors[i] * model_errors[i]);
                volatile_data.push(value);
            }
        }

        volatile_data
    }
}