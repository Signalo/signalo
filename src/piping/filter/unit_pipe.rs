use std::ops::BitOr;

use filter::Filter;
use piping::filter::Pipe;

/// A `UnitPipe` is a simple container wrapping a `Filter`
///
/// ```plain
/// ════════════
///  ╭────────╮
///  │ Filter │
///  ╰────────╯
/// ════════════
/// └─┬────────┘
///   └ UnitPipe
/// ```
#[derive(Clone, Debug)]
pub struct UnitPipe<T> {
    filter: T,
}

impl<T> UnitPipe<T> {
    #[inline]
    pub fn new(filter: T) -> Self {
        Self { filter }
    }
}

impl<T, Rhs> BitOr<Rhs> for UnitPipe<T> {
    type Output = Pipe<Self, Rhs>;

    #[inline]
    fn bitor(self, rhs: Rhs) -> Self::Output {
        Pipe::new(self, rhs)
    }
}

impl<T, I> Filter<I> for UnitPipe<T>
where
    T: Filter<I>,
{
    type Output = T::Output;

    #[inline]
    fn filter(&mut self, input: I) -> Self::Output {
        self.filter.filter(input)
    }

    #[inline]
    fn phase_shift(&self) -> isize {
        self.filter.phase_shift()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Value = usize;

    struct DummyFilter;

    impl Filter<Value> for DummyFilter {
        type Output = Value;

        #[inline]
        fn filter(&mut self, input: Value) -> Self::Output {
            input + 1
        }
    }

    #[test]
    fn filter() {
        let input = vec![0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7];
        let filter = DummyFilter;
        let pipe = UnitPipe::new(filter);
        let subject: Vec<_> = input.iter().scan(pipe, |pipe, &input| {
            Some(pipe.filter(input))
        }).collect();
        let expected = vec![1, 2, 8, 3, 6, 9, 17, 4, 20, 7, 15, 10, 10, 18, 18, 5, 13, 21, 21, 8];
        assert_eq!(subject, expected);
    }
}
