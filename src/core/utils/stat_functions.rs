pub fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

pub fn std(data: &[f64]) -> f64 {
    let mean = mean(data);
    let variance: f64 = data.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / data.len() as f64;
    variance.sqrt()
}

pub fn beta(data_x: &[f64], data_y: &[f64]) -> f64 {
    assert_eq!(data_x.len(), data_y.len());

    let mean_x = mean(data_x);
    let mean_y = mean(data_y);

    let covariance: f64 = data_x.iter()
        .zip(data_y.iter())
        .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
        .sum::<f64>() / data_x.len() as f64;

    covariance / (std(data_x) * std(data_y))
}
