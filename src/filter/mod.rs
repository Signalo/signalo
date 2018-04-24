mod identity;
mod differentiate;

pub mod ops;
pub mod dichotomy;
pub mod mean;
pub mod median;
pub mod kalman;
pub mod convolve;

pub use self::identity::Identity;
pub use self::differentiate::Differentiate;
pub use self::convolve::Convolve;

pub trait LinearPhase {
    #[inline]
    fn phase_shift() -> isize {
        0 // specialize for linearly phase-shifting filter types
    }
}

pub trait Filter<Input>: Sized {
    type Output;

    fn filter(&mut self, input: Input) -> Self::Output;

    #[inline]
    fn reset(&mut self) {
        // specialize for stateful filter types
    }

    #[inline]
    fn phase_shift(&self) -> isize {
        0 // specialize for phase-shifting filter types
    }
}

impl<F, T, U> Filter<T> for F
where
    F: FnMut(T) -> U,
{
    type Output = U;

    fn filter(&mut self, input: T) -> Self::Output {
        self(input)
    }
}
