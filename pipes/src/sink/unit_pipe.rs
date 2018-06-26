// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::BitOr;

use signalo_traits::filter::Filter;
use signalo_traits::sink::Sink;
use sink::Pipe;

/// A `UnitPipe` is a simple container wrapping a `Source`
///
/// ```plain
/// ╠════════════
/// ║ ╭────────╮
/// ║ │ Source │
/// ║ ╰────────╯
/// ╠════════════
/// └─┬────────┘
///   └ UnitPipe
/// ```
#[derive(Default, Clone, Debug)]
pub struct UnitPipe<T> {
    sink: T,
}

impl<T> UnitPipe<T>
{
    /// Creates a new unit pipe wrapping `sink`.
    #[inline]
    pub fn new(sink: T) -> Self {
        Self { sink }
    }
}

impl<T, Rhs> BitOr<Rhs> for UnitPipe<T> {
    type Output = Pipe<Self, Rhs>;

    #[inline]
    fn bitor(self, rhs: Rhs) -> Self::Output {
        Pipe::new(self, rhs)
    }
}

impl<T, I> Sink<I> for UnitPipe<T>
where
    T: Sink<I>,
{
    type Output = T::Output;

    #[inline]
    fn sink(&mut self, input: I) {
        self.sink.sink(input)
    }

    #[inline]
    fn finalize(self) -> Self::Output {
        self.sink.finalize()
    }
}

impl<T, I> Filter<I> for UnitPipe<T>
where
    T: Sink<I>,
{
    type Output = ();

    #[inline]
    fn filter(&mut self, input: I) -> Self::Output {
        self.sink(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Value = usize;

    struct DummySink {
        sum: usize
    }

    impl Sink<Value> for DummySink {
        type Output = Value;

        #[inline]
        fn sink(&mut self, input: Value) {
            self.sum += input;
        }

        #[inline]
        fn finalize(self) -> Self::Output {
            self.sum
        }
    }

    #[test]
    fn sink() {
        let input = vec![0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7];
        let sink = DummySink { sum: 0 };
        let mut pipe = UnitPipe::new(sink);
        for i in input {
            pipe.sink(i);
        }
        let subject = pipe.finalize();
        let expected = 196;
        assert_eq!(subject, expected);
    }
}
