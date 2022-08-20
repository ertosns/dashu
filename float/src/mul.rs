use crate::{
    fbig::FBig,
    error::check_inf_operands,
    repr::{Context, Repr, Word},
    round::{Round, Rounded},
};
use core::ops::{Mul, MulAssign};

impl<'l, 'r, const B: Word, R: Round> Mul<&'r FBig<B, R>> for &'l FBig<B, R> {
    type Output = FBig<B, R>;

    #[inline]
    fn mul(self, rhs: &FBig<B, R>) -> Self::Output {
        check_inf_operands(&self.repr, &rhs.repr);

        let context = Context::max(self.context, rhs.context);
        let repr = Repr::new(
            &self.repr.significand * &rhs.repr.significand,
            self.repr.exponent + rhs.repr.exponent,
        );
        FBig::new_raw(context.repr_round(repr).value(), context)
    }
}

impl<'r, const B: Word, R: Round> Mul<&'r FBig<B, R>> for FBig<B, R> {
    type Output = FBig<B, R>;

    #[inline]
    fn mul(self, rhs: &FBig<B, R>) -> Self::Output {
        check_inf_operands(&self.repr, &rhs.repr);

        let context = Context::max(self.context, rhs.context);
        let repr = Repr::new(
            self.repr.significand * &rhs.repr.significand,
            self.repr.exponent + rhs.repr.exponent,
        );
        FBig::new_raw(context.repr_round(repr).value(), context)
    }
}

impl<'l, const B: Word, R: Round> Mul<FBig<B, R>> for &'l FBig<B, R> {
    type Output = FBig<B, R>;

    #[inline]
    fn mul(self, rhs: FBig<B, R>) -> Self::Output {
        check_inf_operands(&self.repr, &rhs.repr);

        let context = Context::max(self.context, rhs.context);
        let repr = Repr::new(
            &self.repr.significand * rhs.repr.significand,
            self.repr.exponent + rhs.repr.exponent,
        );
        FBig::new_raw(context.repr_round(repr).value(), context)
    }
}

impl<const B: Word, R: Round> Mul<FBig<B, R>> for FBig<B, R> {
    type Output = FBig<B, R>;

    #[inline]
    fn mul(self, rhs: FBig<B, R>) -> Self::Output {
        check_inf_operands(&self.repr, &rhs.repr);

        let context = Context::max(self.context, rhs.context);
        let repr = Repr::new(
            self.repr.significand * rhs.repr.significand,
            self.repr.exponent + rhs.repr.exponent,
        );
        FBig::new_raw(context.repr_round(repr).value(), context)
    }
}

impl<const B: Word, R: Round> MulAssign for FBig<B, R> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = core::mem::take(self) * rhs
    }
}
impl<const B: Word, R: Round> MulAssign<&FBig<B, R>> for FBig<B, R> {
    #[inline]
    fn mul_assign(&mut self, rhs: &FBig<B, R>) {
        *self = core::mem::take(self) * rhs
    }
}

impl<R: Round> Context<R> {
    pub fn mul<const B: Word>(
        &self,
        lhs: &FBig<B, R>,
        rhs: &FBig<B, R>,
    ) -> Rounded<FBig<B, R>> {
        check_inf_operands(&lhs.repr, &rhs.repr);

        // TODO: shrink lhs and rhs to at most double the precision before mul
        let repr = Repr::new(
            &lhs.repr.significand * &rhs.repr.significand,
            lhs.repr.exponent + rhs.repr.exponent,
        );
        self.repr_round(repr).map(|v| FBig::new_raw(v, *self))
    }
}
