use crate::core::utils::states::States;

// TODO: Need to refactor

pub(crate) trait Signal {
    fn arima_or_kalman(&self, price_data: Vec<f64>) -> Vec<States>;
    fn sma(&self, sma_5_data: Vec<f64>, sma_12_data: Vec<f64>) -> Vec<States>;
    fn arima_or_kalman_last(&self, price_data: Vec<f64>) -> States;
    fn sma_last(&self, sma_5_data: Vec<f64>, sma_12_data: Vec<f64>) -> States;
    fn garch(&self, price_data: Vec<f64>, volatile_up: Vec<f64>, volatile_down: Vec<f64>) -> Vec<States>;
}


pub struct TradeSignal;


impl Signal for TradeSignal {
    fn arima_or_kalman(&self, price_data: Vec<f64>) -> Vec<States> {
        let mut states: Vec<States> = Vec::new();
        for t in 1..price_data.len() {
            if price_data[t - 1] < price_data[t] {
                states.push(States::BUY)
            } else if price_data[t - 1] > price_data[t] {
                states.push(States::SELL)
            } else {
                states.push(States::WAIT)
            }
        }

        states
    }

    fn sma(&self, sma_5_data: Vec<f64>, sma_12_data: Vec<f64>) -> Vec<States> {
        let mut states: Vec<States> = Vec::new();
        let start_index = sma_5_data.len() - sma_12_data.len();

        for i in 0..sma_12_data.len() {
            let sma_5_value = sma_5_data[start_index + i];
            let sma_12_value = sma_12_data[i];

            if sma_5_value > sma_12_value {
                states.push(States::BUY);
            } else if sma_5_value < sma_12_value {
                states.push(States::SELL);
            } else {
                states.push(States::WAIT);
            }
        }

        states
    }

    fn arima_or_kalman_last(&self, price_data: Vec<f64>) -> States {
        // Autoregressive moving average strategy (1, 0, 1) / KalmanFilter
        let last_index = price_data.len() - 1;
        let penultimate_index = price_data.len() - 2;
        return if price_data[last_index] > price_data[penultimate_index] {
            States::BUY
        } else if price_data[last_index] < price_data[penultimate_index] {
            States::SELL
        } else {
            States::WAIT
        }
    }

    fn sma_last(&self, sma_5_data: Vec<f64>, sma_12_data: Vec<f64>) -> States {
        // Moving average strategy with implementations of SMA5 and SMA12 pairs
        let last_sma5_index = sma_5_data.len() - 1;
        let last_sma12_index = sma_12_data.len() - 1;
        return if sma_5_data[last_sma5_index] > sma_12_data[last_sma12_index] {
            States::BUY
        } else if sma_5_data[last_sma5_index] < sma_12_data[last_sma12_index] {
            States::SELL
        } else {
            States::WAIT
        }
    }

    fn garch(&self, price_data: Vec<f64>, volatile_up: Vec<f64>, volatile_down: Vec<f64>) -> Vec<States> {
        let mut states: Vec<States> = Vec::new();
        for i in 1..price_data.len() {
            let volatile_diff_up: bool = (price_data[i] - volatile_up[i]) < (price_data[i - 1] - volatile_up[i - 1]);
            let volatile_diff_down: bool = (price_data[i] - volatile_down[i]) < (price_data[i - 1] - volatile_down[i - 1]);

            if price_data[i] > price_data[i - 1] && volatile_diff_up && volatile_diff_down {
                states.push(States::BUY)
            } else if !volatile_diff_up || !volatile_diff_down {
                states.push(States::WAIT)
            } else {
                states.push(States::SELL)
            }
        }

        states
    }
}