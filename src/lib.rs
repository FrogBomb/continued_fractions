pub mod continued_fractions;

#[cfg(test)]
mod tests {
    use continued_fractions::{NormalizedContinuedFraction, ContinuedFraction};
    #[test]
    fn it_works() {
    }
    #[test]
    fn somethingEasy(){
        let test_cf = NormalizedContinuedFraction::new(55, 34);
        assert_eq!(55.0/34.0, test_cf.value());
        // assert_eq!((55, 34), test_cf.as_rational());

    }
}
