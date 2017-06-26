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
   pub fn new(num: u64, denom: u64) -> NormalizedContinuedFraction {
        use self::NormalizedContinuedFraction::*;
        match (num/denom, num%denom) {
            (quot, 0) => Whole(quot),
            (quot, rem) => Continuation(quot, Box::new(NormalizedContinuedFraction::new(denom, rem))),
        }
    }

   pub fn whole(&self) -> NormalizedContinuedFraction {
        use self::NormalizedContinuedFraction::*;
        match *self {
            Whole(w) => Whole(w),
            Continuation(w, _) => Whole(w),
        }
    }

   pub fn fractional_part(self) -> NormalizedContinuedFraction {
        use self::NormalizedContinuedFraction::*;
        match self {
            Whole(w) => Whole(0),
            Continuation(w, f) => Continuation(0, f),
        }
    }

   pub fn inverted(self) -> Option<NormalizedContinuedFraction> {
        use self::NormalizedContinuedFraction::*;
        match self {
            Whole(0) => None,
            Whole(w) => Some(Continuation(0, Box::new(Whole(w)))),
            Continuation(0, f) => Some(*f),
            cf => Some(Continuation(0, Box::new(cf))),
        }
    }

   pub fn value(&self) -> f64 {
        use self::NormalizedContinuedFraction::*;
        match *self {
            Whole(w) => w as f64,
            Continuation(w, ref f) => (w as f64) + 1.0/(*f).value(),
        }
    }

   pub fn as_rational(&self) -> (u64, u64) {
        use self::NormalizedContinuedFraction::*;
        match *self {
            Whole(w) => (w, 1),
            Continuation(w, ref f) => {
                let (d, n) = (*f).as_rational(); // w + 1/(d/n) -> w + n/d -> (w*d + n) / d
                (w*d + n , d)
            }
        }
    }
}

impl ContinuedFraction {
   pub fn new(num: u64, denom: u64) -> ContinuedFraction {
        use self::ContinuedFraction::*;
        use self::NormalizedContinuedFraction as NCF;
        match (num/denom, num%denom) {
            (quot, 0) => Normalized(NCF::Whole(quot)),
            (quot, 1) => Normalized(NCF::Continuation(quot, Box::new(NCF::Whole(denom)))),
            (quot, rem) => Continuation(quot, rem, Box::new(Normalized(NCF::Whole(denom)))),
        }
    }

   pub fn inverted(self) -> Option<ContinuedFraction> {
        use self::ContinuedFraction::*;
        match self {
            Normalized(ncf) => ncf.inverted().map(|f| Normalized(f)),
            Continuation(0, 1, f) => Some(*f),
            cf => Some(Continuation(0, 1, Box::new(cf))),
        }
    }

   pub fn normalized(self) -> NormalizedContinuedFraction {
       use self::ContinuedFraction::*;
       match self {
           Normalized(ncf) => ncf,
           ref cf => {
               let (num, denom) = cf.as_rational();
               NormalizedContinuedFraction::new(num, denom)
           }
       }
    }

   pub fn value(&self) -> f64 {
        use self::ContinuedFraction::*;
        match *self {
            Normalized(ref ncf) => ncf.value(),
            Continuation(w, m, ref f) => (w as f64) + (m as f64)/(*f).value(),
        }
    }

   pub fn as_rational(&self) -> (u64, u64) {
        use self::ContinuedFraction::*;
        match *self {
            Normalized(ref ncf) => ncf.as_rational(),
            Continuation(w, m, ref f) => {
                let (d, n) = (*f).as_rational(); // w + m/(d/n) -> w + m*n/d -> (w*d + m*n) / d
                (w*d + m*n , d)
            }
        }
    }
}


// impl ops::Add<u64> for ContinuedFraction {
//     type Output = ContinuedFraction;
//
//    pub fn add(self, other: u64) -> ContinuedFraction {
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
//    pub fn mul(self, other: u64){
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
//    pub fn add(self, other: ContinuedFraction) -> ContinuedFraction {
//         other + self
//     }
// }
