use sink::Sink;

pub struct Last<T> {
    state: Option<T>,
}

impl<T> Last<T> {
    #[inline]
    pub fn new() -> Self {
        Last { state: None }
    }
}

impl<T> Sink<T> for Last<T> {
    type Output = Option<T>;

    #[inline]
    fn sink(&mut self, input: T) {
        self.state = Some(input);
    }

    #[inline]
    fn finalize(self) -> Self::Output {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sink() {
        // Sequence: https://en.wikipedia.org/wiki/Collatz_conjecture
        let input = vec![0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7];
        let mut sink = Last::new();
        for input in input {
            sink.sink(input);
        }
        let subject = sink.finalize();
        assert_eq!(subject, Some(7));
    }
}
