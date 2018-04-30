use source::Source;

#[derive(Default, Clone)]
pub struct Constant<T> {
    value: T,
}

impl<T> Constant<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Constant { value }
    }
}

impl<T> From<T> for Constant<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self { value }
    }
}

impl<T> Source for Constant<T>
where
    T: Clone,
{
    type Output = T;

    #[inline]
    fn source(&mut self) -> Option<Self::Output> {
        Some(self.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source() {
        const VALUE: usize = 42;
        const COUNT: usize = 3;
        let source = Constant::new(VALUE);
        let subject: Vec<_> = (0..COUNT).scan(source, |source, _| {
            source.source()
        }).collect();
        let expected = vec![VALUE; COUNT];
        assert_eq!(subject, expected);
    }
}
