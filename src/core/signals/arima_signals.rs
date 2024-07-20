use crate::core::utils::states::States;

pub fn trade_signal_arima(price_data: Vec<f64>) -> States {
    // Autoregressive moving average strategy (1, 0, 1)
    let last_index = price_data.len() - 1;
    let penultimate_index = price_data.len() - 2;
    if price_data[last_index] > price_data[penultimate_index] {
        return States::BUY
    } else if price_data[last_index] < price_data[penultimate_index] {
        return States::SELL
    } else {
        return States::WAIT
    }
}