#[derive(Debug, Clone)]
pub enum States {
    BUY, SELL, WAIT
}

#[derive(Debug)]
pub enum Utility {
    HOLD, ESCAPE, EXPECT
}