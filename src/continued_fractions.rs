use std::ops;
use std::iter::Iterator;
// use self::ContinuedFraction::*;

pub enum NormalizedContinuedFraction {
    Whole(u64),
    Continuation(u64, Box<NormalizedContinuedFraction>), // NormalizedContinuation(w, c) -> w + 1/c
}

pub enum ContinuedFraction {
    Continuation(u64, u64, Box<ContinuedFraction>), //Continuation(w, n, d) -> w + n/d
    Normalized(NormalizedContinuedFraction),
}

impl NormalizedContinuedFraction {
    fn new(num: u64, denom: u64) -> NormalizedContinuedFraction {
        use self::NormalizedContinuedFraction::*;
        match (num/denom, num%denom) {
            (w, 0) => Whole(w),
            (w, rem) => Continuation(w, Box::new(NormalizedContinuedFraction::new(denom, rem))),
        }
    }

    fn whole(self) -> NormalizedContinuedFraction {
        use self::NormalizedContinuedFraction::*;
        match self {
            Whole(w) => Whole(w),
            Continuation(w, f) => Whole(w),
        }
    }

    fn fractional_part(self) -> NormalizedContinuedFraction {
        use self::NormalizedContinuedFraction::*;
        match self {
            Whole(w) => Whole(0),
            Continuation(w, f) => *f,
        }
    }

    fn inverted(self) -> NormalizedContinuedFraction {
        use self::NormalizedContinuedFraction::*;
        match self {
            Whole(w) => Continuation(0, Box::new(Whole(w))),
            Continuation(0, f) => *f,
            cf => Continuation(0, Box::new(cf)),
        }
    }

    fn normalized(self) -> NormalizedContinuedFraction {
        self
    }

    fn value(self) -> f64 {
        use self::NormalizedContinuedFraction::*;
        match self {
            Whole(w) => w as f64,
            Continuation(w, f) => (w as f64) + 1.0/(*f).value(),
        }
    }

    fn as_rational(self) -> (u64, u64) {
        match self {
            Whole(w) => (w, 1),
            Continuation(w, f) => {
                val (d, n) = (*f).as_rational(); // w + 1/(d/n) -> w + n/d -> (w*d + n) / d
                (w*d + n , d)
            }
        }
    }
}


// impl ops::Add<u64> for ContinuedFraction {
//     type Output = ContinuedFraction;
//
//     fn add(self, other: u64) -> ContinuedFraction {
//         if(other == 0) { self } else {
//             match self {
//                 Whole(w) => Whole(w + other),
//                 NormalizedFraction(f) => Continuation(other, f),
//                 Continuation(w, f) => Continuation(w + other, f),
//             }
//         }
//     }
// }

// impl ops::Mul<u64> for ContinuedFraction {
//     type Output = ContinuedFraction;
//
//     fn mul(self, other: u64){
//         match self {
//             Whole(w) => Whole(w * other),
//             NormalizedFraction(f) => Fraction(other, f),
//             Fraction(n, d) => Fraction(n * other, f),
//             Continuation(w, f) => Continuation(w * other, )
//         }
//     }
// }

// impl ops::Add<ContinuedFraction> for u64 {
//     type Output = ContinuedFraction;
//
//     fn add(self, other: ContinuedFraction) -> ContinuedFraction {
//         other + self
//     }
// }
