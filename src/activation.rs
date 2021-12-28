pub type Activation = fn(f64) -> f64;

pub fn linear(i: f64) -> f64 {
    i
}

pub fn d_linear(i: f64) -> f64 {
    1.0
}

pub fn relu(i: f64) -> f64 {
    if i > 0.0 {
        i
    } else {
        0.0
    }
}

pub fn d_relu(i: f64) -> f64 {
    if i > 0.0 {
        1.0
    } else {
        0.0
    }
}
