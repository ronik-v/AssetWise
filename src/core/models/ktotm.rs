use crate::core::data::moex_parser::Ticker;

const Q: f64 = 1e-5;
const R: f64 = 0.01 * 0.01;

pub struct KTOTM {
    pub ticker: Ticker
}

// TODO: -
impl KTOTM {
    pub fn new(&self, ticker: Ticker) -> Self { Self { ticker } }

    fn kalman_filter(&self) -> Vec<f64> {
        let n = self.ticker.close.len();
        let mut x: Vec<f64> = vec![0.0; n];
        let mut p: Vec<f64> = vec![0.0; n];
        let mut k: Vec<f64> = vec![0.0; n];
        x[0] = self.ticker.close[0]; p[0] = 1.0;

        for t in 1..n {
            x[t] = x[t - 1]; p[t] = p[t - 1] + Q;
            k[t] = p[t] / (p[t] + R); // Kalman rate
            x[t] = x[t] + k[t] * (self.ticker.close[t] - x[t]); // update trend state
            p[t] = (1.0 - k[t]) * p[t]; // update error state
        }

        x
    }

    fn detect_trend(&self) -> Vec<i8> {
        let kalman_filter_values: Vec<f64> = self.kalman_filter();
        let mut trend_states: Vec<i8> = Vec::new();
        
        for t in 1..kalman_filter_values.len() {
            if kalman_filter_values[t] > kalman_filter_values[t - 1]  {
                trend_states.push(1)
            } else if kalman_filter_values[t] < kalman_filter_values[t - 1] {
                trend_states.push(-1)
            } else {
                trend_states.push(0)
            }
        }
        trend_states
    }
}