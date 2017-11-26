// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFDictionary;
use core_foundation::propertylist::{CFPropertyList, CFPropertyListSubClass};
use core_foundation::string::CFString;
use core_foundation_sys::base::{CFRelease, kCFAllocatorDefault};

use system_configuration_sys::dynamic_store::*;

use std::ptr;

/// Access to the key-value pairs in the dynamic store of a running system.
pub struct SCDynamicStore(SCDynamicStoreRef);

impl Drop for SCDynamicStore {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl_TCFType!(SCDynamicStore, SCDynamicStoreRef, SCDynamicStoreGetTypeID);

impl SCDynamicStore {
    /// Creates a new session used to interact with the dynamic store maintained by the System
    /// Configuration server.
    pub fn create<S: Into<CFString>>(name: S) -> Self {
        let cf_name = name.into();
        unsafe {
            let store = SCDynamicStoreCreate(
                kCFAllocatorDefault,
                cf_name.as_concrete_TypeRef(),
                None,
                ptr::null_mut(),
            );
            SCDynamicStore::wrap_under_create_rule(store)
        }
    }

    /// Creates a new session used to interact with the dynamic store maintained by the System
    /// Configuration server. Uses [`SCDynamicStoreCreateWithOptions`] underneath and sets
    /// `kSCDynamicStoreUseSessionKeys` to true.
    ///
    /// [`SCDynamicStoreCreateWithOptions`]: https://developer.apple.com/documentation/systemconfiguration/1437818-scdynamicstorecreatewithoptions?language=objc
    pub fn create_with_session_keys<S: Into<CFString>>(name: S) -> Self {
        let cf_name = name.into();
        unsafe {
            let store_options = CFDictionary::from_CFType_pairs(&[
                (
                    CFString::wrap_under_create_rule(kSCDynamicStoreUseSessionKeys),
                    CFBoolean::true_value(),
                ),
            ]);
            let store = SCDynamicStoreCreateWithOptions(
                kCFAllocatorDefault,
                cf_name.as_concrete_TypeRef(),
                store_options.as_concrete_TypeRef(),
                None,
                ptr::null_mut(),
            );
            SCDynamicStore::wrap_under_create_rule(store)
        }
    }

    /// If the given key exists in the store, the associated value is returned.
    ///
    /// Use `CFPropertyList::downcast` to cast the result into the correct type.
    pub fn get<S: Into<CFString>>(&self, key: S) -> Option<CFPropertyList> {
        let cf_key = key.into();
        unsafe {
            let dict_ref =
                SCDynamicStoreCopyValue(self.as_concrete_TypeRef(), cf_key.as_concrete_TypeRef());
            if dict_ref != ptr::null() {
                Some(CFPropertyList::wrap_under_create_rule(dict_ref))
            } else {
                None
            }
        }
    }

    /// Sets the value of the given key. Overwrites existing values.
    /// Returns `true` on success, false on failure.
    pub fn set<S: Into<CFString>, R, V: CFPropertyListSubClass<R>>(
        &self,
        key: S,
        value: &V,
    ) -> bool {
        self.set_raw(key, &value.to_CFPropertyList())
    }

    /// Sets the value of the given key. Overwrites existing values.
    /// Returns `true` on success, false on failure.
    pub fn set_raw<S: Into<CFString>>(&self, key: S, value: &CFPropertyList) -> bool {
        let cf_key = key.into();
        let success = unsafe {
            SCDynamicStoreSetValue(
                self.as_concrete_TypeRef(),
                cf_key.as_concrete_TypeRef(),
                value.as_concrete_TypeRef(),
            )
        };
        success != 0
    }
}
