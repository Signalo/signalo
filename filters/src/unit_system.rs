// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Dimensional analysis wrapper filters.

#![cfg(dimensioned)]

use dimensioned::{
    traits::{Dimensioned, MapUnsafe},
    unit_systems::{cgs::CGS, fps::FPS, mks::MKS, si::SI, ucum::UCUM},
};

use signalo_traits::Filter;
use signalo_traits::{
    Config as ConfigTrait, ConfigClone, ConfigRef, FromGuts, Guts, IntoGuts, Reset,
    State as StateTrait, StateMut, WithConfig,
};

/// A differentiate filter that produces the derivative of the signal.
#[derive(Clone, Debug)]
pub struct UnitSystem<T> {
    /// Inner filter.
    pub inner: T,
}

impl<T> From<T> for UnitSystem<T> {
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> Default for UnitSystem<T>
where
    T: Default,
{
    fn default() -> Self {
        let inner = T::default();
        Self { inner }
    }
}

impl<T> ConfigTrait for UnitSystem<T>
where
    T: ConfigTrait,
{
    type Config = T::Config;
}

impl<T> StateTrait for UnitSystem<T>
where
    T: StateTrait,
{
    type State = T::State;
}

impl<T> WithConfig for UnitSystem<T>
where
    T: WithConfig<Output = T>,
{
    type Output = Self;

    fn with_config(config: Self::Config) -> Self::Output {
        let inner = T::with_config(config);
        Self { inner }
    }
}

impl<T> ConfigRef for UnitSystem<T>
where
    T: ConfigRef,
{
    fn config_ref(&self) -> &Self::Config {
        self.inner.config_ref()
    }
}

impl<T> ConfigClone for UnitSystem<T>
where
    T: ConfigClone,
{
    fn config(&self) -> Self::Config {
        self.inner.config()
    }
}

impl<T> StateMut for UnitSystem<T>
where
    T: StateMut,
{
    unsafe fn state_mut(&mut self) -> &mut Self::State {
        self.inner.state_mut()
    }
}

impl<T> Guts for UnitSystem<T>
where
    T: Guts,
{
    type Guts = T::Guts;
}

impl<T> FromGuts for UnitSystem<T>
where
    T: FromGuts,
{
    unsafe fn from_guts(guts: Self::Guts) -> Self {
        let inner = T::from_guts(guts);
        Self { inner }
    }
}

impl<T> IntoGuts for UnitSystem<T>
where
    T: IntoGuts,
{
    fn into_guts(self) -> Self::Guts {
        self.inner.into_guts()
    }
}

impl<T> Reset for UnitSystem<T>
where
    T: Reset,
{
    fn reset(mut self) -> Self {
        self.inner.reset_mut();
        self
    }
}

macro_rules! impl_dimensioned {
    ($t:ident) => {
        impl<T, U, V> Filter<$t<V, U>> for UnitSystem<T>
        where
            T: Filter<V, Output = V>,
            $t<V, U>: Dimensioned<Value = V, Units = U>,
            $t<V, U>: MapUnsafe<V, U, Output = $t<V, U>>,
        {
            type Output = $t<V, U>;

            fn filter(&mut self, input: $t<V, U>) -> Self::Output {
                input.map_unsafe(|unitless| self.inner.filter(unitless))
            }
        }
    };
}

impl_dimensioned!(CGS);
impl_dimensioned!(FPS);
impl_dimensioned!(MKS);
impl_dimensioned!(SI);
impl_dimensioned!(UCUM);

#[cfg(test)]
mod tests {
    use super::*;

    use dimensioned::unit_systems::si::Meter;

    struct AddFourPointTwo;

    impl Filter<f32> for AddFourPointTwo {
        type Output = f32;

        #[inline]
        fn filter(&mut self, input: f32) -> Self::Output {
            input + 4.2
        }
    }

    #[test]
    fn test() {
        let add_fourty_two = AddFourPointTwo;

        let filter = UnitSystem::from(add_fourty_two);
        // Sequence: https://en.wikipedia.org/wiki/Collatz_conjecture

        let input: Vec<_> = vec![
            0.0, 1.0, 7.0, 2.0, 5.0, 8.0, 16.0, 3.0, 19.0, 6.0, 14.0, 9.0, 9.0, 17.0, 17.0, 4.0,
            12.0, 20.0, 20.0, 7.0,
        ].into_iter()
        .map(|unitless| Meter::new(unitless))
        .collect();

        let expected = vec![
            4.200, 5.200, 11.200, 6.200, 9.200, 12.200, 20.200, 7.200, 23.200, 10.200, 18.200,
            13.200, 13.200, 21.200, 21.200, 8.200, 16.200, 24.200, 24.200, 11.200,
        ];

        let output: Vec<_> = input
            .iter()
            .scan(filter, |filter, &input| Some(filter.filter(input)))
            .map(|dimensioned| dimensioned.value_unsafe)
            .collect();
        assert_nearly_eq!(output, expected);
    }
}
