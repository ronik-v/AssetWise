use std::fmt::Display;

pub const STRATEGY_SMA: &str = "Sma";
pub const STRATEGY_ARIMA: &str = "Arima";
pub const STRATEGY_KALMAN_FILTER: &str = "KalmanFilter";


#[derive(PartialEq, Default)]
pub enum Page {
    #[default]
    Home,
    Strategy,
    Settings,
}

#[derive(PartialEq)]
pub enum ChartType {
    Line,
    Candlestick,
}