pub fn mean(data: Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 0..data.len() {
        sum += data[i]
    }

    sum / data.len() as f64
}

pub fn std(data: Vec<f64>) -> f64 {
    // VAR = M[X^ 2] - M[X] ^ 2 \ Q = VAR ^ 0.5
    let mut mean_square: f64 = 0.0;
    let mean: f64 = mean(data.clone());
    for i in 0..data.len() {
        mean_square += data[i] * data[i];
    }

    (mean_square - mean) * (mean_square - mean)
}

pub fn beta(data_x: Vec<f64>, data_y: Vec<f64>) -> f64 {
    assert_eq!(data_x.len(), data_y.len());

    let mut cov: f64 = 0.0;
    let mean_x = mean(data_x.clone());
    let mean_y = mean(data_y.clone());

    for i in 0..data_x.len() {
        cov += (data_x[i] - mean_x) * (data_y[i] - mean_y);
    }

    cov / (std(data_x.clone()) * std(data_y.clone()))
}