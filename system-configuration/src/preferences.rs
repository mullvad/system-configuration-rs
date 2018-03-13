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
//! [`SCPreferences`]: https://developer.apple.com/documentation/systemconfiguration/scpreferences


use core_foundation::base::{CFAllocatorRef, TCFType};
use core_foundation::string::CFString;

pub use system_configuration_sys::preferences::*;

use std::ptr;


declare_TCFType!{
    /// The handle to an open preferences session for accessing system configuration preferences.
    SCPreferences, SCPreferencesRef
}

impl_TCFType!(SCPreferences, SCPreferencesRef, SCPreferencesGetTypeID);


impl SCPreferences {
    /// Initiates access to the default system preferences using the default allocator.
    pub fn default(calling_process_name: &CFString) -> Self {
        Self::with_allocator(ptr::null(), calling_process_name, None)
    }

    /// Initiates access to the given (`prefs_id`) group of configuration preferences using the
    /// default allocator. To access the default system preferences, use the [`default`]
    /// constructor.
    ///
    /// [`default`]: #method.default
    pub fn group(calling_process_name: &CFString, prefs_id: &CFString) -> Self {
        Self::with_allocator(ptr::null(), calling_process_name, Some(prefs_id))
    }

    /// Initiates access to the per-system set of configuration preferences with a given
    /// allocator and preference group to access.
    pub fn with_allocator(
        allocator: CFAllocatorRef,
        calling_process_name: &CFString,
        prefs_id: Option<&CFString>,
    ) -> Self {
        let prefs_id_ptr = match prefs_id {
            Some(prefs_id) => prefs_id.as_concrete_TypeRef(),
            None => ptr::null(),
        };

        unsafe {
            SCPreferences::wrap_under_create_rule(SCPreferencesCreate(
                allocator,
                calling_process_name.as_concrete_TypeRef(),
                prefs_id_ptr,
            ))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retain_count() {
        let preferences = SCPreferences::default(&CFString::new("test"));
        assert_eq!(preferences.retain_count(), 1);
    }
}
