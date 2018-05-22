// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Exponential moving average filters.

mod mean;
mod mean_variance;

pub use self::mean::Mean;
pub use self::mean_variance::MeanVariance;