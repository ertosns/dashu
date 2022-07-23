//! Operators on the sign of [IBig].

use crate::{
    ibig::IBig,
    ops::{Abs, UnsignedAbs},
    ubig::UBig,
};
use core::ops::{Mul, MulAssign, Neg};

/// An enum representing the sign of a number
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Sign {
    Positive,
    Negative,
}

use Sign::*;

impl Neg for Sign {
    type Output = Sign;

    #[inline]
    fn neg(self) -> Sign {
        match self {
            Positive => Negative,
            Negative => Positive,
        }
    }
}

impl Mul<Sign> for Sign {
    type Output = Sign;

    #[inline]
    fn mul(self, rhs: Sign) -> Sign {
        match (self, rhs) {
            (Positive, Positive) => Positive,
            (Positive, Negative) => Negative,
            (Negative, Positive) => Negative,
            (Negative, Negative) => Positive,
        }
    }
}

impl MulAssign<Sign> for Sign {
    #[inline]
    fn mul_assign(&mut self, rhs: Sign) {
        *self = *self * rhs;
    }
}

impl IBig {
    /// A number representing the sign of `self`.
    ///
    /// * -1 if the number is negative
    /// * 0 if the number is zero
    /// * 1 if the number is positive
    ///
    /// # Examples
    /// ```
    /// # use dashu_int::ibig;
    /// assert_eq!(ibig!(-500).signum(), ibig!(-1));
    /// ```
    #[inline]
    pub fn signum(&self) -> IBig {
        IBig(self.0.signum())
    }
}

impl Neg for IBig {
    type Output = IBig;

    #[inline]
    fn neg(self) -> IBig {
        IBig(self.0.neg())
    }
}

impl Neg for &IBig {
    type Output = IBig;

    #[inline]
    fn neg(self) -> IBig {
        IBig(self.0.clone().neg())
    }
}

impl Abs for IBig {
    type Output = IBig;

    #[inline]
    fn abs(self) -> IBig {
        IBig(self.0.with_sign(Sign::Positive))
    }
}

impl Abs for &IBig {
    type Output = IBig;

    #[inline]
    fn abs(self) -> IBig {
        IBig(self.0.clone().with_sign(Sign::Positive))
    }
}

impl UnsignedAbs for IBig {
    type Output = UBig;

    #[inline]
    fn unsigned_abs(self) -> UBig {
        UBig(self.0.with_sign(Sign::Positive))
    }
}

impl UnsignedAbs for &IBig {
    type Output = UBig;

    #[inline]
    fn unsigned_abs(self) -> UBig {
        UBig(self.0.clone().with_sign(Sign::Positive))
    }
}
