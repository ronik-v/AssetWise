use crate::core::data::moex_parser::Ticker;
use crate::core::utils::states::Utility;
use crate::core::utils::stat_functions::std;
use crate::core::utils::stat_functions::beta;

pub struct UtilityFunction {
    pub ticker: Ticker,
    pub risk_value: f64
}

impl UtilityFunction {
    pub fn new(ticker: Ticker, risk_value: f64) -> Self {
        Self { ticker, risk_value }
    }

    fn ticker_income(&self) -> Vec<f64> {
        let mut income: Vec<f64> = Vec::new();
        for i in 0..self.ticker.open.len() {
            income.push(self.ticker.close[i] - self.ticker.open[i]);
        }

        income
    }

    // TODO: realise result function -> U = E(R) - 0.5 * A * q^2  || U >= 1 \ 0 < U < 1 | U < 0
    pub fn result(&self) -> Utility {
        let bound_rate: f64 = 14.31; // https://www.cbr.ru/hd_base/zcyc_params/zcyc/
        let ticker_income: Vec<f64> = self.ticker_income();
        let beta: f64 = beta(ticker_income.clone(), self.ticker.close.clone());
        let volatility: f64 = std(ticker_income.clone());
        let mut income_sum: f64 = 0.0;
        for i in 0..ticker_income.len() {
            income_sum += ticker_income[i];
        }
        income_sum = bound_rate + beta * (income_sum - bound_rate);

        let utility_value: f64 = income_sum - 0.5 * self.risk_value * volatility;

        return if utility_value >= 1.0 {
            Utility::HOLD
        } else if utility_value < 1.0 && utility_value > 0.0 {
            Utility::EXPECT
        } else {
            Utility::ESCAPE
        }
    }
}