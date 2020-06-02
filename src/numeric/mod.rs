pub fn float_equal(f1: f64, f2: f64, epsilon: f64) -> bool {
    if f64::abs(f1 - f2) < epsilon {
        true
    } else {
        false
    }
}
