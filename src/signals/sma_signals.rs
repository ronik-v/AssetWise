use crate::utils::states::States;

pub fn trade_signal_sma(sma_5_data: Vec<f64>, sma_12_data: Vec<f64>) -> States {
    let last_sma5_index = sma_5_data.len() - 1;
    let last_sma12_index = sma_12_data.len() - 1;
    if sma_5_data[last_sma5_index] > sma_12_data[last_sma12_index] {
        return States::BUY
    } else if sma_5_data[last_sma5_index] < sma_12_data[last_sma12_index] {
        return States::SELL
    } else {
        return States::WAIT
    }
}