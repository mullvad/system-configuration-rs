use core_foundation_sys::base::CFAllocatorRef;
use core_foundation_sys::string::CFStringRef;

use std::os::raw::c_void;


pub type __SCPreferences = c_void;
pub type SCPreferencesRef = *const __SCPreferences;


#[link(name = "SystemConfiguration", kind = "framework")]
extern "C" {
    pub fn SCPreferencesCreate(allocator: CFAllocatorRef,
                               name: CFStringRef,
                               prefsID: CFStringRef) -> SCPreferencesRef;
}