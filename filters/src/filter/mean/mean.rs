// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Moving average filters.

use std::fmt;

use arraydeque::{Array, ArrayDeque, Wrapping};

use num_traits::Num;

use signalo_traits::filter::Filter;

/// A filter producing the moving median over a given signal.
#[derive(Clone, Default)]
pub struct Mean<A>
where
    A: Array,
{
    state: Option<A::Item>,
    buffer: ArrayDeque<A, Wrapping>,
    weight: A::Item,
}

impl<T, A> fmt::Debug for Mean<A>
where
    T: fmt::Debug,
    A: Array<Item = T> + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Mean")
            .field("state", &self.state)
            .field("buffer", &self.buffer)
            .field("weight", &self.weight)
            .finish()
    }
}

impl<T, A> Filter<T> for Mean<A>
where
    T: Copy + Num,
    A: Array<Item=T>,
{
    type Output = T;

    fn filter(&mut self, input: T) -> Self::Output {
        let old_mean = self.state.unwrap_or(input);
        let old_weight = self.weight;
        let (mean, weight) = if let Some(old_input) = self.buffer.push_back(input) {
            let mean = old_mean - old_input + input;
            (mean, old_weight)
        } else {
            let mean = old_mean + input;
            let weight = old_weight + T::one();
            (mean, weight)
        };
        self.state = Some(mean);
        self.weight = weight;

        mean / weight
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
            0.000, 0.500, 2.667, 3.333, 4.667, 5.000, 9.667, 9.000, 12.667, 9.333, 13.000,
            9.667, 10.667, 11.667, 14.333, 12.667, 11.000, 12.000, 17.333, 15.667, 11.333,
            9.667, 12.333, 13.333, 16.000, 14.333, 48.000, 46.333, 49.000, 18.000, 47.333,
            43.000, 45.667, 14.667, 17.333, 15.667, 18.333, 21.000, 25.333, 21.000, 50.333,
            41.667, 48.667, 17.667, 20.333, 16.000, 45.333, 43.667, 46.333, 19.667
        ]
    }

    #[test]
    fn test() {
        let filter: Mean<[f32; 3]> = Mean::default();
        // Sequence: https://en.wikipedia.org/wiki/Collatz_conjecture
        let input = get_input();
        let output: Vec<_> = input.iter().scan(filter, |filter, &input| {
            Some(filter.filter(input))
        }).collect();
        assert_nearly_eq!(output, get_output(), 0.001);
    }
}
