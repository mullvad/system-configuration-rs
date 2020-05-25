// automatically generated by rust-bindgen

// Generated using:
// bindgen 0.54.0
// macOS SDK 10.15.4.

#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
use crate::dispatch_queue_t;
pub use crate::network_configuration::{
    SCNetworkReachabilityCallBack, SCNetworkReachabilityContext, SCNetworkReachabilityFlags,
    SCNetworkReachabilityRef,
};
use core_foundation_sys::base::{Boolean, CFAllocatorRef, CFTypeID};
use core_foundation_sys::runloop::CFRunLoopRef;
use core_foundation_sys::string::CFStringRef;
use libc::{c_char, c_uchar, sockaddr};

pub type __uint8_t = c_uchar;
pub type sa_family_t = __uint8_t;
pub const kSCNetworkReachabilityFlagsTransientConnection: _bindgen_ty_64 = 1;
pub const kSCNetworkReachabilityFlagsReachable: _bindgen_ty_64 = 2;
pub const kSCNetworkReachabilityFlagsConnectionRequired: _bindgen_ty_64 = 4;
pub const kSCNetworkReachabilityFlagsConnectionOnTraffic: _bindgen_ty_64 = 8;
pub const kSCNetworkReachabilityFlagsInterventionRequired: _bindgen_ty_64 = 16;
pub const kSCNetworkReachabilityFlagsConnectionOnDemand: _bindgen_ty_64 = 32;
pub const kSCNetworkReachabilityFlagsIsLocalAddress: _bindgen_ty_64 = 65536;
pub const kSCNetworkReachabilityFlagsIsDirect: _bindgen_ty_64 = 131072;
pub const kSCNetworkReachabilityFlagsIsWWAN: _bindgen_ty_64 = 262144;
pub const kSCNetworkReachabilityFlagsConnectionAutomatic: _bindgen_ty_64 = 8;
pub type _bindgen_ty_64 = u32;
extern "C" {
    pub fn SCNetworkReachabilityCreateWithAddress(
        allocator: CFAllocatorRef,
        address: *const sockaddr,
    ) -> SCNetworkReachabilityRef;

    pub fn SCNetworkReachabilityCreateWithAddressPair(
        allocator: CFAllocatorRef,
        localAddress: *const sockaddr,
        remoteAddress: *const sockaddr,
    ) -> SCNetworkReachabilityRef;

    pub fn SCNetworkReachabilityCreateWithName(
        allocator: CFAllocatorRef,
        nodename: *const c_char,
    ) -> SCNetworkReachabilityRef;

    pub fn SCNetworkReachabilityGetTypeID() -> CFTypeID;

    pub fn SCNetworkReachabilityGetFlags(
        target: SCNetworkReachabilityRef,
        flags: *mut SCNetworkReachabilityFlags,
    ) -> Boolean;

    pub fn SCNetworkReachabilitySetCallback(
        target: SCNetworkReachabilityRef,
        callout: SCNetworkReachabilityCallBack,
        context: *mut SCNetworkReachabilityContext,
    ) -> Boolean;

    pub fn SCNetworkReachabilityScheduleWithRunLoop(
        target: SCNetworkReachabilityRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;

    pub fn SCNetworkReachabilityUnscheduleFromRunLoop(
        target: SCNetworkReachabilityRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;

    pub fn SCNetworkReachabilitySetDispatchQueue(
        target: SCNetworkReachabilityRef,
        queue: dispatch_queue_t,
    ) -> Boolean;
}
