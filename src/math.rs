//! TODO does this need to exist?
//! Used for dev but should probably be replaced with something faster.

// TODO: make generic
// TODO: benchmark
pub fn inner_product(a: &[f64], b: &[f64]) -> f64 {
    // TODO handle panic as Result
    assert_eq!(a.len(), b.len());
    assert!(a.len() > 0);

    a.iter()
        .zip(b.iter())
        .fold(0.0, |acc, (ai, bi)| acc + ai * bi)
}

#[cfg(test)]
mod test {
    use super::*;
    use float_eq::float_eq;

    // double equality comparison ratio epsilon
    const DOUBLE_EQ_EPSILON: f64 = 0.000_000_1;

    #[test]
    fn test_inner_product() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 1.0];
        let b: Vec<f64> = vec![1.0, 2.0, 4.0, 1.0];

        let c = inner_product(&a, &b);

        assert!(float_eq!(c, 18.0, rel <= DOUBLE_EQ_EPSILON));
    }
}
