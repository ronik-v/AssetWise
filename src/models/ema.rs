pub struct Ema {
    pub data: Vec<f64>
}

impl Ema {
    pub fn new(data: Vec<f64>) -> Self {
        Ema { data }
    }

    pub fn values(&self) -> Vec<f64> {
        // Calculate ema values - standard formula
        let mut ema_values: Vec<f64> = Vec::new();
        let ema_first: f64 = self.data[0];
        let alpha: f64 = 2.0 / (self.data.len() as f64 + 1.0);

        ema_values.push(ema_first);
        for i in 1..self.data.len() {
            let ema_value = alpha * self.data[i] + (1.0 - alpha) * ema_first;
            ema_values.push(ema_value);
        }

        ema_values
    }
}