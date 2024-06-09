use crate::models::arima::Arima;
use crate::models::sma::Sma;
/*
    Predictions models
*/

trait Oracle {
    fn arima(&self, price_data: Vec<f64>) -> Arima;
    fn sma(&self, data: Vec<f64>, split: usize) -> Sma;
}


impl dyn Oracle {
    fn arima(&self, price_data: Vec<f64>) -> Arima {
        Arima { price_data }
    }

    fn sma(&self, data: Vec<f64>, split: usize) -> Sma {
        Sma { data, split }
    }
}