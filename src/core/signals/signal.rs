use crate::core::utils::states::States;

pub(crate) trait Signal {
    fn arima_or_kalman(&self, price_data: Vec<f64>) -> States;
    fn sma(&self, sma_5_data: Vec<f64>, sma_12_data: Vec<f64>) -> States;
}


pub struct TradeSignal;


impl Signal for TradeSignal {
    fn arima_or_kalman(&self, price_data: Vec<f64>) -> States {
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

    fn sma(&self, sma_5_data: Vec<f64>, sma_12_data: Vec<f64>) -> States {
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
}