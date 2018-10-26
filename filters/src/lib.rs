// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A collection of filters used in 'signalo' umbrella crate.

#![cfg_attr(feature = "missing_mpl", feature(plugin))]
#![cfg_attr(feature = "missing_mpl", plugin(missing_mpl))]
#![cfg_attr(feature = "missing_mpl", deny(missing_mpl))]
#![cfg_attr(feature = "nightly", feature(try_from))]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate core as std;

extern crate num_traits;

extern crate arraydeque;

extern crate dimensioned;

#[macro_use]
extern crate generic_array;

#[cfg(test)]
#[macro_use]
extern crate nearly_eq;

pub extern crate signalo_traits;

pub use signalo_traits as traits;

/// The crate's prelude.
pub mod prelude {}

pub mod classify;
pub mod convolve;
pub mod delay;
pub mod differentiate;
pub mod hampel;
pub mod identity;
pub mod integrate;
pub mod mean;
pub mod median;
pub mod observe;
pub mod ops;
pub mod unit_system;
pub mod wavelet;
