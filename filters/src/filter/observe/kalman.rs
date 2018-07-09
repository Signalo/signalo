// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Kalman filters.

use num_traits::{Zero, One, Num};

use signalo_traits::filter::Filter;

/// A 1-dimensional Kalman filter.
#[derive(Clone, Debug)]
pub struct Kalman<T> {
    r: T, /// Process noise covariance
    q: T, /// Measurement noise covariance
    a: T, /// State transition
    b: T, /// Control transition
    c: T, /// Measurement
    cov: T, /// Covariance (uncertainty)
    x: Option<T>,
}

impl<T> Kalman<T>
where
    T: Zero
{
    /// Creates a new `Kalman` filter with given `r`, `q`, `a`, `b`, and `c` coefficients.
    ///
    /// Coefficients:
    /// - `r`: Process noise covariance
    /// - `q`: Measurement noise covariance
    /// - `a`: State transition
    /// - `b`: Control transition
    /// - `c`: Measurement
    #[inline]
    pub fn new(r: T, q: T, a: T, b: T, c: T) -> Self {
        let cov = T::zero();
        let x = None;
        Kalman { r, q, a, b, c, cov, x }
    }
}

impl<T> Kalman<T>
where
    T: Copy + Num
{
    fn process(&mut self, (input, control): (T, T)) -> T {
        let c_squared = self.c * self.c;
        let (x, cov) = match self.x {
            None => {
                let x = input / self.c;
                let cov = self.q / c_squared;
                (x, cov)
            },
            Some(mut x) => {
                // Compute prediction:
                let pred_state = (self.a * x) + (self.b * control);
                let pred_cov = (self.a * self.cov * self.a) + self.r;

                // Compute Kalman gain:
                let k = pred_cov * self.c / ((pred_cov * c_squared) + self.q);

                // Correction:
                let x = pred_state + k * (input - (self.c * pred_state));
                let cov = pred_cov - (k * self.c * pred_cov);
                (x, cov)
            },
        };
        self.x = Some(x);
        self.cov = cov;
        x
    }
}

impl<T> Default for Kalman<T>
where
    T: Zero + One
{
    #[inline]
    fn default() -> Self {
        let r = T::one();
        let q = T::one();
        let a = T::one();
        let b = T::zero();
        let c = T::one();
        Kalman::new(r, q, a, b, c)
    }
}

impl<T> Filter<T> for Kalman<T>
where
    T: Copy + Num
{
    type Output = T;

    fn filter(&mut self, input: T) -> Self::Output {
        self.process((input, T::zero()))
    }
}

impl<T> Filter<(T, T)> for Kalman<T>
where
    T: Copy + Num
{
    type Output = T;

    fn filter(&mut self, (input, control): (T, T)) -> Self::Output {
        self.process((input, control))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<f32> {
        vec![
            0.0, 1.0, 7.0, 2.0, 5.0, 8.0, 16.0, 3.0, 19.0, 6.0, 14.0, 9.0, 9.0, 17.0, 17.0, 4.0,
            12.0, 20.0, 20.0, 7.0, 7.0, 15.0, 15.0, 10.0, 23.0, 10.0, 111.0, 18.0, 18.0, 18.0,
            106.0, 5.0, 26.0, 13.0, 13.0, 21.0, 21.0, 21.0, 34.0, 8.0, 109.0, 8.0, 29.0, 16.0,
            16.0, 16.0, 104.0, 11.0, 24.0, 24.0
        ]
    }

    fn get_output() -> Vec<f32> {
        vec![
            0.000, 0.524, 3.012, 2.682, 3.375, 4.693, 7.837, 6.510, 9.912, 8.851, 10.245,
            9.908, 9.663, 11.646, 13.092, 10.636, 11.004, 13.435, 15.208, 12.991, 11.372,
            12.352, 13.068, 12.239, 15.146, 13.756, 40.027, 34.076, 29.733, 26.563, 48.024,
            36.401, 33.591, 28.028, 23.968, 23.166, 22.581, 22.154, 25.354, 20.666, 44.530,
            34.661, 33.132, 28.503, 25.126, 22.660, 44.635, 35.548, 32.428, 30.151
        ]
    }

    #[test]
    fn floating_point() {
        let r = 0.0001; // Process noise
        let q = 0.001; // Measurement noise
        let a = 1.0; // State
        let b = 0.0; // Control
        let c = 1.0; // Measurement
        let filter = Kalman::new(r, q, a, b, c);

        // Sequence: https://en.wikipedia.org/wiki/Collatz_conjecture
        let input = get_input();
        let output: Vec<_> = input.iter().scan(filter, |filter, &input| {
            Some(filter.filter(input))
        }).collect();

        assert_nearly_eq!(output, get_output(), 0.001);
    }
}
