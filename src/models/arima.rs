pub struct Arima {
    pub price_data: Vec<f64>,
}

impl Arima {
    pub fn new(price_data: Vec<f64>) -> Self {
        Self { price_data }
    }

    fn auto_regression_parameter(&self) -> f64 {
        let mut f1 = 0.0;
        let mut f2 = 0.0;
        for i in 1..self.price_data.len() {
            f1 += self.price_data[i] * self.price_data[i - 1];
            f2 += self.price_data[i - 1] * self.price_data[i - 1];
        }
        f1 / f2
    }

    fn model_residuals(&self) -> Vec<f64> {
        let fi = self.auto_regression_parameter();
        let mut model_residuals = vec![0.0];
        for i in 1..self.price_data.len() {
            let tmp_residual = self.price_data[i] - (fi * self.price_data[i - 1]);
            model_residuals.push(tmp_residual);
        }
        model_residuals
    }

    fn auto_correlation_residual_value(&self) -> f64 {
        let mut p1 = 0.0;
        let model_residuals = self.model_residuals();
        let mean_residuals = model_residuals.iter().sum::<f64>() / (model_residuals.len() - 1) as f64;
        for i in 1..model_residuals.len() {
            p1 += ((model_residuals[i] - mean_residuals) * (model_residuals[i - 1] - mean_residuals))
                / ((model_residuals[i] - mean_residuals) * (model_residuals[i] - mean_residuals));
        }
        p1 / (1.0 - p1)
    }

    pub fn model_prediction_time_series(&self) -> Vec<f64> {
        let mut model_prediction = Vec::new();
        let fi = self.auto_regression_parameter();
        let theta1 = self.auto_correlation_residual_value();
        let model_residuals = self.model_residuals();
        for i in 1..self.price_data.len() {
            let time_prediction = fi * self.price_data[i - 1] + theta1 * model_residuals[i - 1] + model_residuals[i];
            model_prediction.push(time_prediction);
        }
        model_prediction
    }
}