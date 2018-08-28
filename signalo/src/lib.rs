// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The 'signalo' umbrella crate.

#![cfg_attr(feature = "missing_mpl", feature(plugin))]
#![cfg_attr(feature = "missing_mpl", plugin(missing_mpl))]
#![cfg_attr(feature = "missing_mpl", deny(missing_mpl))]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate core as std;

extern crate signalo_filters;
extern crate signalo_pipes;
extern crate signalo_traits;

/// The crate's prelude.
pub mod prelude {
    pub use signalo_filters::prelude as filters;
    pub use signalo_pipes::prelude as pipes;
    pub use signalo_traits::prelude as traits;
}
