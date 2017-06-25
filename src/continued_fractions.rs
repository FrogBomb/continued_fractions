use std::ops;
use self::NormalizedContinuedFraction::*;

pub enum NormalizedContinuedFraction {
    Whole(u32),
    Fraction(Box<NormalizedContinuedFraction>),
    Continuation(u32, Box<NormalizedContinuedFraction>),
}

impl NormalizedContinuedFraction {
    fn new() -> NormalizedContinuedFraction {
        Whole(0)
    }

    fn whole(self) -> NormalizedContinuedFraction {
        match self {
            Whole(w) => Whole(w),
            Fraction(f) => Whole(0),
            Continuation(w, f) => Whole(w),
        }
    }

    fn fraction(self) -> NormalizedContinuedFraction {
        match self {
            Whole(w) => Whole(0),
            Fraction(f) => Fraction(f),
            Continuation(w, f) => Fraction(f),
        }
    }

    fn invert(self) -> NormalizedContinuedFraction {
        match self {
            Whole(w) => Fraction(Box::new(Whole(w))),
            Fraction(f) => *f,
            Continuation(0, f) => *f,
            cf => Fraction(Box::new(cf)),
        }
    }

    fn value(self) -> f32 {
        match self {
            Whole(w) => w as f32,
            Fraction(f) => 1.0/(*f).value(),
            Continuation(w, f) => (w as f32) + 1.0/(*f).value(),
        }
    }
}

impl ops::Add<u32> for NormalizedContinuedFraction{
    type Output = NormalizedContinuedFraction;

    fn add(self, other: u32) -> NormalizedContinuedFraction {
        if(other == 0) { self } else {
            match self {
                Whole(w) => Whole(w + other),
                Fraction(f) => Continuation(other, f),
                Continuation(w, f) => Continuation(w + other, f),
            }
        }
    }

}
