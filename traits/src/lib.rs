// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A collection of traits used in 'signalo' umbrella crate.

#![cfg_attr(feature = "missing_mpl", feature(plugin))]
#![cfg_attr(feature = "missing_mpl", plugin(missing_mpl))]
#![cfg_attr(feature = "missing_mpl", deny(missing_mpl))]

#![warn(missing_docs)]

pub mod source;
pub mod filter;
pub mod sink;

/// The crate's prelude.
pub mod prelude {
    pub use source::Source;
    pub use sink::Sink;
    pub use filter::Filter;
}
