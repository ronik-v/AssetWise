/*
   Strategies with directional movement system indicator
*/
use crate::utils::states::States;
// TODO: refactor!!!


pub fn trade_signal_adx(
    directional_indicators_plus: Vec<f64>, directional_indicators_minus: Vec<f64>, adx: Vec<f64>, is_fast: bool
) -> States {
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
