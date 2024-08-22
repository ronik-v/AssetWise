use crate::core::utils::states::States;

pub(crate) trait Signal {
    fn adx(
        &self, directional_indicators_plus: Vec<f64>, directional_indicators_minus: Vec<f64>, adx: Vec<f64>, is_fast: bool
    ) -> States;
    fn arima(&self, price_data: Vec<f64>) -> States;
    fn sma(&self, sma_5_data: Vec<f64>, sma_12_data: Vec<f64>) -> States;
}


pub struct TradeSignal;


impl Signal for TradeSignal {
    fn adx(&self, directional_indicators_plus: Vec<f64>, directional_indicators_minus: Vec<f64>, adx: Vec<f64>, is_fast: bool) -> States {
        // Bullishit code
        let last_di_plus_index: usize = directional_indicators_plus.len() - 1;
        let last_di_minus_index: usize = directional_indicators_minus.len() - 1;
        let last_adx_index: usize = adx.len() - 1;

        // Values - +D, -D, ADX[t] > ADX[t - 1]
        let di_plus_val: f64 = directional_indicators_plus[last_di_plus_index];
        let di_minus_val: f64 = directional_indicators_minus[last_di_minus_index];
        let is_adx_val_growth: bool = adx[last_adx_index] > adx[last_adx_index - 1];

        // If strategy with short position
        if is_fast {
            if di_plus_val > di_minus_val && is_adx_val_growth {
                States::BUY
            } else if di_plus_val < di_minus_val && !is_adx_val_growth {
                States::SELL
            } else {
                States::WAIT
            }
            // If strategy with long position
        } else {
            if di_plus_val < di_minus_val && is_adx_val_growth {
                States::BUY
            } else if di_plus_val > di_minus_val && !is_adx_val_growth {
                States::SELL
            } else {
                States::WAIT
            }
        }
    }

    fn arima(&self, price_data: Vec<f64>) -> States {
        // Autoregressive moving average strategy (1, 0, 1)
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