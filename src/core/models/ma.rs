/*
    Mean average model for adx model calculation
    Ma.values used in adx func with classic lag = 14
*/

pub struct MA {
    lags: usize,
    data: Vec<f64>,
    ma_values: Vec<f64>,
}

impl MA {
    // Конструктор для создания новой модели MA
    pub fn new(lags: usize, data: Vec<f64>) -> Self {
        let ma_values = MA::calculate_ma(lags, &data);
        MA { lags, data, ma_values }
    }

    // MA values calculate
    fn calculate_ma(lags: usize, data: &Vec<f64>) -> Vec<f64> {
        let mut ma_values = Vec::new();

        for i in 0..data.len() {
            if i >= lags {
                let mut sum = 0.0;
                for j in 0..lags {
                    sum += data[i - j];
                }
                ma_values.push(sum / lags as f64);
            }
        }

        ma_values
    }

    pub fn values(&self) -> Vec<f64> {
        // MA values for adx model
        self.ma_values.clone()
    }
}
