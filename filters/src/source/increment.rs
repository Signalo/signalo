// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ops::AddAssign;

use signalo_traits::source::Source;

#[derive(Default, Clone)]
pub struct Increment<T> {
    state: T,
    interval: T,
}

impl<T> Increment<T> {
    #[inline]
    pub fn new(initial: T, interval: T) -> Self {
        Self { state: initial, interval }
    }
}

impl<T> Source for Increment<T>
where
    T: Copy + AddAssign<T>,
{
    type Output = T;

    #[inline]
    fn source(&mut self) -> Option<Self::Output> {
        let output = self.state.clone();
        self.state += self.interval;
        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source() {
        let source = Increment::new(42, 2);
        let subject: Vec<_> = (0..5).scan(source, |source, _| {
            source.source()
        }).collect();
        let expected = vec![42, 44, 46, 48, 50];
        assert_eq!(subject, expected);
    }
}
