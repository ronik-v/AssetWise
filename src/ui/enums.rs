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