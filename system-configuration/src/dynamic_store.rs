// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bindings to [`SCDynamicStore`].
//!
//! See the examples directory for examples how to use this module.
//!
//! [`SCDynamicStore`]: https://developer.apple.com/documentation/systemconfiguration/scdynamicstore?language=objc

use crate::sys::{
    self, kSCDynamicStoreUseSessionKeys, SCDynamicStoreCallBack, SCDynamicStoreContext,
};
use objc2_core_foundation::{
    kCFAllocatorDefault, CFArray, CFBoolean, CFDictionary, CFPropertyList, CFRetained,
    CFRunLoopSource, CFString, CFType,
};
use std::{
    ffi::c_void,
    ptr::{self, NonNull},
};

/// Struct describing the callback happening when a watched value in the dynamic store is changed.
pub struct SCDynamicStoreCallBackContext<T> {
    /// The callback function that will be called when a watched value in the dynamic store is
    /// changed.
    pub callout: SCDynamicStoreCallBackT<T>,

    /// The argument passed to each `callout` call. Can be used to keep state between
    /// callbacks.
    pub info: T,
}

/// Signature for callback functions getting called when a watched value in the dynamic store is
/// changed.
///
/// This is the safe callback definition, abstracting over the lower level `SCDynamicStoreCallBack`
/// from the `objc2-system-configuration` crate.
pub type SCDynamicStoreCallBackT<T> =
    fn(store: &SCDynamicStore, changed_keys: &CFArray<CFString>, info: &mut T);

/// Builder for [`SCDynamicStore`] sessions.
///
/// [`SCDynamicStore`]: struct.SCDynamicStore.html
pub struct SCDynamicStoreBuilder<T> {
    name: CFRetained<CFString>,
    session_keys: bool,
    callback_context: Option<SCDynamicStoreCallBackContext<T>>,
}

impl SCDynamicStoreBuilder<()> {
    /// Creates a new builder. `name` is used as the name parameter when creating the
    /// [`SCDynamicStore`] session.
    ///
    /// [`SCDynamicStore`]: struct.SCDynamicStore.html
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        SCDynamicStoreBuilder {
            name: CFString::from_str(name.as_ref()),
            session_keys: false,
            callback_context: None,
        }
    }
}

impl<T> SCDynamicStoreBuilder<T> {
    /// Set whether or not the created [`SCDynamicStore`] should have session keys or not.
    /// See [`SCDynamicStoreCreateWithOptions`] for details.
    ///
    /// Defaults to `false`.
    ///
    /// [`SCDynamicStore`]: struct.SCDynamicStore.html
    /// [`SCDynamicStoreCreateWithOptions`]: https://developer.apple.com/documentation/systemconfiguration/1437818-scdynamicstorecreatewithoptions?language=objc
    pub fn session_keys(mut self, session_keys: bool) -> Self {
        self.session_keys = session_keys;
        self
    }

    /// Set a callback context (callback function and data to pass to each callback call).
    ///
    /// Defaults to having callbacks disabled.
    pub fn callback_context<T2>(
        self,
        callback_context: SCDynamicStoreCallBackContext<T2>,
    ) -> SCDynamicStoreBuilder<T2> {
        SCDynamicStoreBuilder {
            name: self.name,
            session_keys: self.session_keys,
            callback_context: Some(callback_context),
        }
    }

    /// Create the dynamic store session.
    pub fn build(mut self) -> Option<SCDynamicStore> {
        let store_options = self.create_store_options();
        if let Some(callback_context) = self.callback_context.take() {
            SCDynamicStore::create(
                &self.name,
                store_options.as_opaque(),
                Some(convert_callback::<T>),
                &mut self.create_context(callback_context),
            )
        } else {
            SCDynamicStore::create(&self.name, store_options.as_opaque(), None, ptr::null_mut())
        }
    }

    fn create_store_options(&self) -> CFRetained<CFDictionary<CFString, CFType>> {
        let key = unsafe { kSCDynamicStoreUseSessionKeys };
        let value = CFBoolean::new(self.session_keys);
        CFDictionary::from_slices(&[key], &[&**value])
    }

    fn create_context(
        &self,
        callback_context: SCDynamicStoreCallBackContext<T>,
    ) -> SCDynamicStoreContext {
        // move the callback context struct to the heap and "forget" it.
        // It will later be brought back into the Rust typesystem and freed in
        // `release_callback_context`
        let info_ptr = Box::into_raw(Box::new(callback_context));

        SCDynamicStoreContext {
            version: 0,
            info: info_ptr as *mut _ as *mut c_void,
            retain: None,
            release: Some(release_callback_context::<T>),
            copyDescription: None,
        }
    }
}

/// Access to the key-value pairs in the dynamic store of a running system.
///
/// Use the [`SCDynamicStoreBuilder`] to create instances of this.
///
/// [`SCDynamicStoreBuilder`]: struct.SCDynamicStoreBuilder.html
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SCDynamicStore(pub CFRetained<sys::SCDynamicStore>);

impl SCDynamicStore {
    /// Creates a new session used to interact with the dynamic store maintained by the System
    /// Configuration server.
    fn create(
        name: &CFString,
        store_options: &CFDictionary,
        callout: SCDynamicStoreCallBack,
        context: *mut SCDynamicStoreContext,
    ) -> Option<Self> {
        unsafe {
            sys::SCDynamicStore::with_options(
                kCFAllocatorDefault,
                name,
                Some(store_options),
                callout,
                context,
            )
            .map(Self)
        }
    }

    /// Returns the keys that represent the current dynamic store entries that match the specified
    /// pattern. Or `None` if an error occurred.
    ///
    /// `pattern` - A regular expression pattern used to match the dynamic store keys.
    pub fn get_keys<S: AsRef<str>>(&self, pattern: S) -> Option<CFRetained<CFArray<CFString>>> {
        let cf_pattern = CFString::from_str(pattern.as_ref());
        let array = sys::SCDynamicStore::key_list(Some(&self.0), &cf_pattern);
        array.map(|array| unsafe { CFRetained::cast_unchecked::<CFArray<CFString>>(array) })
    }

    /// Returns the key-value pairs that represent the current internet proxy settings. Or `None` if
    /// no proxy settings have been defined or if an error occurred.
    pub fn get_proxies(&self) -> Option<CFRetained<CFDictionary<CFString, CFType>>> {
        let dict = sys::SCDynamicStore::proxies(Some(&self.0));
        dict.map(|dict| unsafe {
            CFRetained::cast_unchecked::<CFDictionary<CFString, CFType>>(dict)
        })
    }

    /// If the given key exists in the store, the associated value is returned.
    ///
    /// Use `CFRetained::downcast` to cast the result into the correct type.
    pub fn get<S: AsRef<str>>(&self, key: S) -> Option<CFRetained<CFPropertyList>> {
        let cf_key = CFString::from_str(key.as_ref());
        sys::SCDynamicStore::value(Some(&self.0), &cf_key)
    }

    /// Sets the value of the given key. Overwrites existing values.
    /// Returns `true` on success, false on failure.
    pub fn set(&self, key: &CFString, value: &CFPropertyList) -> bool {
        unsafe { sys::SCDynamicStore::set_value(Some(&self.0), key, value) }
    }

    /// Removes the value of the specified key from the dynamic store.
    pub fn remove<S: AsRef<str>>(&self, key: S) -> bool {
        let cf_key = CFString::from_str(key.as_ref());
        sys::SCDynamicStore::remove_value(Some(&self.0), &cf_key)
    }

    /// Specifies a set of keys and key patterns that should be monitored for changes.
    pub fn set_notification_keys(
        &self,
        keys: &CFArray<CFString>,
        patterns: &CFArray<CFString>,
    ) -> bool {
        unsafe {
            self.0
                .set_notification_keys(Some(keys.as_opaque()), Some(patterns.as_opaque()))
        }
    }

    /// Creates a run loop source object that can be added to the application's run loop.
    pub fn create_run_loop_source(&self) -> Option<CFRetained<CFRunLoopSource>> {
        sys::SCDynamicStore::new_run_loop_source(unsafe { kCFAllocatorDefault }, &self.0, 0)
    }
}

/// The raw callback used by the safe `SCDynamicStore` to convert from the `SCDynamicStoreCallBack`
/// to the `SCDynamicStoreCallBackT`
unsafe extern "C-unwind" fn convert_callback<T>(
    store_ref: NonNull<sys::SCDynamicStore>,
    changed_keys_ref: NonNull<CFArray>,
    context_ptr: *mut c_void,
) {
    let store = store_ref.cast::<SCDynamicStore>().as_ref();
    let changed_keys = changed_keys_ref.as_ref();
    let changed_keys = changed_keys.cast_unchecked::<CFString>();

    let context = &mut *(context_ptr as *mut _ as *mut SCDynamicStoreCallBackContext<T>);

    (context.callout)(store, changed_keys, &mut context.info);
}

// Release function called by core foundation on release of the dynamic store context.
unsafe extern "C-unwind" fn release_callback_context<T>(context_ptr: NonNull<c_void>) {
    // Bring back the context object from raw ptr so it is correctly freed.
    let _context = Box::from_raw(context_ptr.as_ptr() as *mut SCDynamicStoreCallBackContext<T>);
}
