// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # SystemConfiguration bindings
//!
//! This crate provides a bit safer bindings to the Apple [SystemConfiguration] framework. For
//! direct bindings, check out the [`objc2-system-configuration`] crate (re-exported as [`sys`]).
//!
//! This crate only implements a small part of the [SystemConfiguration] framework so far. If you
//! need a yet unimplemented part, feel free to submit a pull request!
//!
//! [SystemConfiguration]: https://developer.apple.com/documentation/systemconfiguration?language=objc
//! [`objc2-system-configuration`]: objc2_system_configuration

#![deny(missing_docs)]

/// CoreFoundation wrappers.
pub use objc2_core_foundation as core_foundation;
/// Auto-generated SystemConfiguration bindings.
pub use objc2_system_configuration as sys;

pub mod dynamic_store;
pub mod network_configuration;
pub mod network_reachability;
pub mod preferences;
