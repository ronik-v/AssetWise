use crate::models::arima::Arima;
use crate::models::sma::Sma;
/*
    Predictions models
    ARIMA - integrated autoregressive model - moving average -
            model and methodology for time series analysis
    SMA - moving average (rolling average or running average or moving mean[1] or rolling mean)
            is a calculation to analyze  data points by creating a series of averages of different
            selections of the full data set.
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