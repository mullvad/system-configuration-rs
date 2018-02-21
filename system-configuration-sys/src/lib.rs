// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Low level bindings to the Apple [SystemConfiguration] framework. Generated with bindgen.
//! For a safe, higher level, API, check out the [`system-configuration`] crate.
//!
//! [SystemConfiguration]: https://developer.apple.com/documentation/systemconfiguration?language=objc
//! [`system-configuration`]: https://crates.io/crates/system-configuration

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate core_foundation_sys;

pub mod dynamic_store;
pub mod preferences;
pub mod network_configuration;
