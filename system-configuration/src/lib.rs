// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # SystemConfiguration bindings
//!
//! This crate is a high level binding to the Apple [SystemConfiguration] framework. For low level
//! FFI bindings, check out the [`system-configuration-sys`] crate.
//!
//! This crate only implements a small part of the [SystemConfiguration] framework so far. If you
//! need a yet unimplemented part, feel free to submit a pull request!
//!
//! [SystemConfiguration]: https://developer.apple.com/documentation/systemconfiguration?language=objc
//! [`system-configuration-sys`]: https://crates.io/crates/system-configuration-sys
#![feature(test)]
#![deny(missing_docs)]

#[macro_use]
extern crate core_foundation;
extern crate system_configuration_sys;

pub mod dynamic_store;
pub mod preferences;
pub mod network_configuration;
