/*
    Implementation of a simple moving average taking into account offset
    To build the strategy, an offset of 5 and 12 is used
*/

pub struct Sma {
    pub data: Vec<f64>,
    pub split: usize,
}

impl Sma {
    pub fn new(data: Vec<f64>, split: usize) -> Self {
        Sma { data, split }
    }

    pub fn values(&self) -> Vec<f64> {
        let mut moving_averages = Vec::new();

        if self.data.len() < self.split {
            return moving_averages;
        }

        for time_index in 0..=(self.data.len() - self.split) {
            let window = &self.data[time_index..time_index + self.split];
            let average: f64 = window.iter().copied().sum::<f64>() / window.len() as f64;
            moving_averages.push(average);
        }

        moving_averages
    }
}
