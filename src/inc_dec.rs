use num_traits::Num;
use std::ops::{AddAssign, SubAssign};

#[cfg(test)]
mod unit_tests;

pub trait IncDec {
    fn post_dec(&mut self, inc: Self) -> Self;
    fn post_inc(&mut self, inc: Self) -> Self;
    fn pre_dec(&mut self, inc: Self) -> Self;
    fn pre_inc(&mut self, inc: Self) -> Self;

}

impl<T> IncDec for T where T: AddAssign + Copy + Num + SubAssign {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn post_dec(&mut self, dec: Self) -> Self {
        let tmp = *self;
        self.pre_dec(dec);
        tmp
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn post_inc(&mut self, inc: Self) -> Self {
        let tmp = *self;
        self.pre_inc(inc);
        tmp
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn pre_dec(&mut self, dec: Self) -> Self {
        *self -= dec;
        *self
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn pre_inc(&mut self, inc: Self) -> Self {
        *self += inc;
        *self
    }
}