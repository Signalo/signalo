// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::Sub;

use num_traits::Zero;

use signalo_traits::filter::Filter;

use traits::{InitialState, Resettable, Stateful, StatefulUnsafe};

/// A differentiate filter's internal state.
#[derive(Clone, Debug)]
pub struct State<T> {
    pub value: Option<T>,
}

/// A filter that produces the derivative of the signal.
#[derive(Clone, Debug)]
pub struct Differentiate<T> {
    state: State<T>,
}

impl<T> Default for Differentiate<T>
where
    T: Default,
{
    fn default() -> Self {
        let state = Self::initial_state(());
        Self { state }
    }
}

impl<T> Stateful for Differentiate<T> {
    type State = State<T>;
}

unsafe impl<T> StatefulUnsafe for Differentiate<T> {
    unsafe fn state(&self) -> &Self::State {
        &self.state
    }

    unsafe fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }
}

impl<T> InitialState<()> for Differentiate<T> {
    fn initial_state(_: ()) -> Self::State {
        let value = None;
        State { value }
    }
}

impl<T> Resettable for Differentiate<T> {
    fn reset(&mut self) {
        self.state = Self::initial_state(());
    }
}

impl<T> Filter<T> for Differentiate<T>
where
    T: Copy + Sub<T, Output = T> + Zero,
{
    type Output = <T as Sub<T>>::Output;

    fn filter(&mut self, input: T) -> Self::Output {
        let output = match self.state.value {
            None => T::zero(),
            Some(state) => input - state,
        };
        self.state.value = Some(input);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let filter = Differentiate::default();
        // Sequence: https://en.wikipedia.org/wiki/Collatz_conjecture
        let input = vec![
            0.0, 1.0, 7.0, 2.0, 5.0, 8.0, 16.0, 3.0, 19.0, 6.0, 14.0, 9.0, 9.0, 17.0, 17.0, 4.0,
            12.0, 20.0, 20.0, 7.0,
        ];
        let output: Vec<_> = input
            .iter()
            .scan(filter, |filter, &input| Some(filter.filter(input)))
            .collect();
        assert_nearly_eq!(
            output,
            vec![
                0.0, 1.0, 6.0, -5.0, 3.0, 3.0, 8.0, -13.0, 16.0, -13.0, 8.0, -5.0, 0.0, 8.0, 0.0,
                -13.0, 8.0, 8.0, 0.0, -13.0
            ]
        );
    }
}
