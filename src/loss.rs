pub fn ms_error(desired: &Vec<f64>, actual: &Vec<f64>) -> f64 {
    assert!(desired.len() == actual.len());
    let mut sum = 0.0;
    for (des, act) in std::iter::zip(desired.iter(), actual.iter()) {
        sum += (act - des).powi(2);
    }
    sum / (desired.len() as f64)
}
