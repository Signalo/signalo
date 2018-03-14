use std::ops::BitOr;

use filter::pipe::Pipe;
use filter::Filter;

#[derive(Default, Clone)]
pub struct Identity;

impl_pipe!(Identity);

impl<T> Filter<T> for Identity {
    type Output = T;

    #[inline]
    fn apply(&mut self, input: T) -> Self::Output {
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer() {
        let filter = Identity::default();
        let input = vec![1, 1, 2, 3, 5, 8, 13, 21, 34];
        let output: Vec<_> = input.iter().scan(filter, |filter, &input| {
            Some(filter.apply(input))
        }).collect();
        assert_eq!(output, vec![1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn float() {
        let filter = Identity::default();
        let input = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        let output: Vec<_> = input.iter().scan(filter, |filter, &input| {
            Some(filter.apply(input))
        }).collect();
        assert_nearly_eq!(output, vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0]);
    }
}
