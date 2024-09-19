use crate::core::utils::stat_functions::std;

const Q: f64 = 1e-5;
const R: f64 = 0.01 * 0.01;

pub struct KTOTM {
    pub data: Vec<f64>
}


impl KTOTM {
    pub fn new(data: Vec<f64>) -> Self { Self { data } }

    fn kalman_filter(&self) -> Vec<f64> {
        let n = self.data.len();
        let mut x: Vec<f64> = vec![0.0; n];
        let mut p: Vec<f64> = vec![0.0; n];
        let mut k: Vec<f64> = vec![0.0; n];
        x[0] = self.data[0]; p[0] = 1.0;

        for t in 1..n {
            x[t] = x[t - 1]; p[t] = p[t - 1] + Q;
            k[t] = p[t] / (p[t] + R); // Kalman rate
            x[t] = x[t] + k[t] * (self.data[t] - x[t]); // update trend state
            p[t] = (1.0 - k[t]) * p[t]; // update error state
        }

        x
    }

    fn detect_trend(&self) -> Vec<i8> {
        let kalman_filter_values: Vec<f64> = self.kalman_filter();
        let mut trend_states: Vec<i8> = Vec::new();
        trend_states.push(0);

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

    fn std_line(&self) -> Vec<f64> {
        // Day value std
        let mut std_line: Vec<f64> = Vec::new();
        for day in 0..self.data.len() {
            std_line.push(std(&self.data[0..day].to_vec()));
        }

        std_line
    }

    pub fn prediction_trend(&self) -> Vec<f64> {
        let std_line: Vec<f64> = self.std_line();
        let detect_trend: Vec<i8> = self.detect_trend();

        let mut prediction_trend_values: Vec<f64> = Vec::new();

        for i in 0..self.data.len() {
            match detect_trend[i] {
                1 => { prediction_trend_values.push(self.data[i] + std_line[i]) }
                0 => { prediction_trend_values.push(self.data[i]) }
                -1 => { prediction_trend_values.push(self.data[i] - std_line[i]) }
                _ => {}
            }
        }

        prediction_trend_values
    }
}