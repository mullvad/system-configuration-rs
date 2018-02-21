// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bindings to [`SCPreferences`].
//!
//! See the examples directory for examples how to use this module.
//!
//! [`SCPreferences`]: https://developer.apple.com/documentation/systemconfiguration/scpreferences-ft8


use core_foundation::base::CFAllocatorRef;
use core_foundation::base::TCFType;
use core_foundation::string::CFString;

pub use system_configuration_sys::preferences::*;

use std::ptr;


declare_TCFType!{
    /// The handle to an open preferences session for accessing system configuration preferences.
    SCPreferences, SCPreferencesRef
}

impl_TCFType!(SCPreferences, SCPreferencesRef, SCPreferencesGetTypeID);


impl SCPreferences {
    /// Initiates access to the per-system set of configuration preferences.
    pub fn new(allocator: CFAllocatorRef, name: &str, prefs_id: Option<&str>) -> Self {
        let prefs_id = match prefs_id {
            Some(prefs_id) => CFString::new(prefs_id).as_concrete_TypeRef(),
            None => ptr::null(),
        };

        unsafe {
            SCPreferences::wrap_under_get_rule(SCPreferencesCreate(
                allocator,
                CFString::new(name).as_concrete_TypeRef(),
                prefs_id,
            ))
        }
    }
}
