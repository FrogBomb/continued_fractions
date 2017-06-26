pub mod continued_fractions;

#[cfg(test)]
mod tests {
    use continued_fractions::{NormalizedContinuedFraction, ContinuedFraction};
    #[test]
    fn something_easy() {
        let test_cf = NormalizedContinuedFraction::new(55, 34);
        assert_eq!(55.0/34.0, test_cf.value());
        assert_eq!((55, 34), test_cf.as_rational());
        assert_eq!((34, 21), test_cf.fractional_part().inverted().unwrap().as_rational());
    }
    #[test]
    fn bigger_test() {
        use self::NormalizedContinuedFraction::*;
        let mut test_cf = NormalizedContinuedFraction::new(1134903170, 701408733);
        loop {
            let value = test_cf.value();
            test_cf = match test_cf {
                Whole(x) => {
                    assert_eq!(x, 2, "{}", value);
                    break
                },
                Continuation(x, f) => {
                    assert_eq!(x, 1, "{}", value);
                    *f
                },
            }
        }

    }
}
