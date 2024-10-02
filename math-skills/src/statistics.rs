pub fn sum(data: &[f64]) -> f64 {
    data.iter().sum()
}

pub fn average(data: &[f64]) -> u64 {
    (sum(data) / data.len() as f64).round() as u64
}

pub fn median(data: &[f64]) -> i32 {
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = data.len() / 2;
    ((sorted_data[mid - 1] + sorted_data[mid]) / 2.0).round() as i32
}

pub fn variance(data: &[f64]) -> i64 {
    let avg = average(data) as f64;
    let variance_sum: f64 = data.iter().map(|&val| (val - avg).powi(2)).sum();
    (variance_sum / data.len() as f64).round() as i64
}

pub fn standard_deviation(data: &[f64]) -> i64 {
    (f64::from(variance(data) as f32) as f64).sqrt().round() as i64
}
