/* automatically generated by rust-bindgen 0.69.4 */

// Generated using:
// bindgen 0.69.4
// macOS SDK 14.5.

use core::ffi::c_void;
use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::{Boolean, CFAllocatorRef, CFIndex, CFTypeID};
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::runloop::CFRunLoopRef;
use core_foundation_sys::string::CFStringRef;

use crate::dispatch_queue_t;
use crate::preferences::SCPreferencesRef;
use libc::{sockaddr, socklen_t};

pub type __SCNetworkConnection = c_void;
pub type __SCNetworkInterface = c_void;
pub type __SCBondStatus = c_void;
pub type __SCNetworkProtocol = c_void;
pub type __SCNetworkService = c_void;
pub type __SCNetworkSet = c_void;

pub type SCNetworkConnectionFlags = u32;
extern "C" {
    pub fn SCNetworkCheckReachabilityByAddress(
        address: *const sockaddr,
        addrlen: socklen_t,
        flags: *mut SCNetworkConnectionFlags,
    ) -> Boolean;

    pub fn SCNetworkCheckReachabilityByName(
        nodename: *const ::core::ffi::c_char,
        flags: *mut SCNetworkConnectionFlags,
    ) -> Boolean;

    pub fn SCNetworkInterfaceRefreshConfiguration(ifName: CFStringRef) -> Boolean;
}
pub type SCNetworkConnectionRef = *const __SCNetworkConnection;
#[repr(C)]
pub struct SCNetworkConnectionContext {
    pub version: CFIndex,
    pub info: *mut ::core::ffi::c_void,
    pub retain: Option<
        unsafe extern "C" fn(info: *const ::core::ffi::c_void) -> *const ::core::ffi::c_void,
    >,
    pub release: Option<unsafe extern "C" fn(info: *const ::core::ffi::c_void)>,
    pub copyDescription:
        Option<unsafe extern "C" fn(info: *const ::core::ffi::c_void) -> CFStringRef>,
}
pub type SCNetworkConnectionStatus = i32;
pub type SCNetworkConnectionCallBack = Option<
    unsafe extern "C" fn(
        connection: SCNetworkConnectionRef,
        status: SCNetworkConnectionStatus,
        info: *mut ::core::ffi::c_void,
    ),
>;
extern "C" {
    pub fn SCNetworkConnectionGetTypeID() -> CFTypeID;

    pub fn SCNetworkConnectionCopyUserPreferences(
        selectionOptions: CFDictionaryRef,
        serviceID: *mut CFStringRef,
        userOptions: *mut CFDictionaryRef,
    ) -> Boolean;

    pub fn SCNetworkConnectionCreateWithServiceID(
        allocator: CFAllocatorRef,
        serviceID: CFStringRef,
        callout: SCNetworkConnectionCallBack,
        context: *mut SCNetworkConnectionContext,
    ) -> SCNetworkConnectionRef;

    pub fn SCNetworkConnectionCopyServiceID(connection: SCNetworkConnectionRef) -> CFStringRef;

    pub fn SCNetworkConnectionGetStatus(
        connection: SCNetworkConnectionRef,
    ) -> SCNetworkConnectionStatus;

    pub fn SCNetworkConnectionCopyExtendedStatus(
        connection: SCNetworkConnectionRef,
    ) -> CFDictionaryRef;

    pub fn SCNetworkConnectionCopyStatistics(connection: SCNetworkConnectionRef)
        -> CFDictionaryRef;

    pub fn SCNetworkConnectionStart(
        connection: SCNetworkConnectionRef,
        userOptions: CFDictionaryRef,
        linger: Boolean,
    ) -> Boolean;

    pub fn SCNetworkConnectionStop(
        connection: SCNetworkConnectionRef,
        forceDisconnect: Boolean,
    ) -> Boolean;

    pub fn SCNetworkConnectionCopyUserOptions(
        connection: SCNetworkConnectionRef,
    ) -> CFDictionaryRef;

    pub fn SCNetworkConnectionScheduleWithRunLoop(
        connection: SCNetworkConnectionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;

    pub fn SCNetworkConnectionUnscheduleFromRunLoop(
        connection: SCNetworkConnectionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;

    pub fn SCNetworkConnectionSetDispatchQueue(
        connection: SCNetworkConnectionRef,
        queue: dispatch_queue_t,
    ) -> Boolean;
}
pub type SCNetworkInterfaceRef = *const __SCNetworkInterface;
extern "C" {
    pub static kSCNetworkInterfaceType6to4: CFStringRef;

    pub static kSCNetworkInterfaceTypeBluetooth: CFStringRef;

    pub static kSCNetworkInterfaceTypeBond: CFStringRef;

    pub static kSCNetworkInterfaceTypeEthernet: CFStringRef;

    pub static kSCNetworkInterfaceTypeFireWire: CFStringRef;

    pub static kSCNetworkInterfaceTypeIEEE80211: CFStringRef;

    pub static kSCNetworkInterfaceTypeIPSec: CFStringRef;

    pub static kSCNetworkInterfaceTypeIrDA: CFStringRef;

    pub static kSCNetworkInterfaceTypeL2TP: CFStringRef;

    pub static kSCNetworkInterfaceTypeModem: CFStringRef;

    pub static kSCNetworkInterfaceTypePPP: CFStringRef;

    pub static kSCNetworkInterfaceTypePPTP: CFStringRef;

    pub static kSCNetworkInterfaceTypeSerial: CFStringRef;

    pub static kSCNetworkInterfaceTypeVLAN: CFStringRef;

    pub static kSCNetworkInterfaceTypeWWAN: CFStringRef;

    pub static kSCNetworkInterfaceTypeIPv4: CFStringRef;

    pub static kSCNetworkInterfaceIPv4: SCNetworkInterfaceRef;
}
pub type SCBondInterfaceRef = SCNetworkInterfaceRef;
pub type SCBondStatusRef = *const __SCBondStatus;
pub const kSCBondStatusOK: _bindgen_ty_286 = 0;
pub const kSCBondStatusLinkInvalid: _bindgen_ty_286 = 1;
pub const kSCBondStatusNoPartner: _bindgen_ty_286 = 2;
pub const kSCBondStatusNotInActiveGroup: _bindgen_ty_286 = 3;
pub const kSCBondStatusUnknown: _bindgen_ty_286 = 999;
pub type _bindgen_ty_286 = ::core::ffi::c_uint;
extern "C" {
    pub static kSCBondStatusDeviceAggregationStatus: CFStringRef;

    pub static kSCBondStatusDeviceCollecting: CFStringRef;

    pub static kSCBondStatusDeviceDistributing: CFStringRef;
}
pub type SCNetworkProtocolRef = *const __SCNetworkProtocol;
pub type SCNetworkServiceRef = *const __SCNetworkService;
pub type SCNetworkSetRef = *const __SCNetworkSet;
extern "C" {
    pub fn SCNetworkInterfaceGetTypeID() -> CFTypeID;

    pub fn SCNetworkInterfaceCopyAll() -> CFArrayRef;

    pub fn SCNetworkInterfaceGetSupportedInterfaceTypes(
        interface: SCNetworkInterfaceRef,
    ) -> CFArrayRef;

    pub fn SCNetworkInterfaceGetSupportedProtocolTypes(
        interface: SCNetworkInterfaceRef,
    ) -> CFArrayRef;

    pub fn SCNetworkInterfaceCreateWithInterface(
        interface: SCNetworkInterfaceRef,
        interfaceType: CFStringRef,
    ) -> SCNetworkInterfaceRef;

    pub fn SCNetworkInterfaceGetBSDName(interface: SCNetworkInterfaceRef) -> CFStringRef;

    pub fn SCNetworkInterfaceGetConfiguration(interface: SCNetworkInterfaceRef) -> CFDictionaryRef;

    pub fn SCNetworkInterfaceGetExtendedConfiguration(
        interface: SCNetworkInterfaceRef,
        extendedType: CFStringRef,
    ) -> CFDictionaryRef;

    pub fn SCNetworkInterfaceGetHardwareAddressString(
        interface: SCNetworkInterfaceRef,
    ) -> CFStringRef;

    pub fn SCNetworkInterfaceGetInterface(
        interface: SCNetworkInterfaceRef,
    ) -> SCNetworkInterfaceRef;

    pub fn SCNetworkInterfaceGetInterfaceType(interface: SCNetworkInterfaceRef) -> CFStringRef;

    pub fn SCNetworkInterfaceGetLocalizedDisplayName(
        interface: SCNetworkInterfaceRef,
    ) -> CFStringRef;

    pub fn SCNetworkInterfaceSetConfiguration(
        interface: SCNetworkInterfaceRef,
        config: CFDictionaryRef,
    ) -> Boolean;

    pub fn SCNetworkInterfaceSetExtendedConfiguration(
        interface: SCNetworkInterfaceRef,
        extendedType: CFStringRef,
        config: CFDictionaryRef,
    ) -> Boolean;

    pub fn SCNetworkInterfaceCopyMediaOptions(
        interface: SCNetworkInterfaceRef,
        current: *mut CFDictionaryRef,
        active: *mut CFDictionaryRef,
        available: *mut CFArrayRef,
        filter: Boolean,
    ) -> Boolean;

    pub fn SCNetworkInterfaceCopyMediaSubTypes(available: CFArrayRef) -> CFArrayRef;

    pub fn SCNetworkInterfaceCopyMediaSubTypeOptions(
        available: CFArrayRef,
        subType: CFStringRef,
    ) -> CFArrayRef;

    pub fn SCNetworkInterfaceCopyMTU(
        interface: SCNetworkInterfaceRef,
        mtu_cur: *mut ::core::ffi::c_int,
        mtu_min: *mut ::core::ffi::c_int,
        mtu_max: *mut ::core::ffi::c_int,
    ) -> Boolean;

    pub fn SCNetworkInterfaceSetMediaOptions(
        interface: SCNetworkInterfaceRef,
        subtype: CFStringRef,
        options: CFArrayRef,
    ) -> Boolean;

    pub fn SCNetworkInterfaceSetMTU(
        interface: SCNetworkInterfaceRef,
        mtu: ::core::ffi::c_int,
    ) -> Boolean;

    pub fn SCNetworkInterfaceForceConfigurationRefresh(interface: SCNetworkInterfaceRef)
        -> Boolean;

    pub fn SCBondInterfaceCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;

    pub fn SCBondInterfaceCopyAvailableMemberInterfaces(prefs: SCPreferencesRef) -> CFArrayRef;

    pub fn SCBondInterfaceCreate(prefs: SCPreferencesRef) -> SCBondInterfaceRef;

    pub fn SCBondInterfaceRemove(bond: SCBondInterfaceRef) -> Boolean;

    pub fn SCBondInterfaceGetMemberInterfaces(bond: SCBondInterfaceRef) -> CFArrayRef;

    pub fn SCBondInterfaceGetOptions(bond: SCBondInterfaceRef) -> CFDictionaryRef;

    pub fn SCBondInterfaceSetMemberInterfaces(
        bond: SCBondInterfaceRef,
        members: CFArrayRef,
    ) -> Boolean;

    pub fn SCBondInterfaceSetLocalizedDisplayName(
        bond: SCBondInterfaceRef,
        newName: CFStringRef,
    ) -> Boolean;

    pub fn SCBondInterfaceSetOptions(
        bond: SCBondInterfaceRef,
        newOptions: CFDictionaryRef,
    ) -> Boolean;

    pub fn SCBondInterfaceCopyStatus(bond: SCBondInterfaceRef) -> SCBondStatusRef;

    pub fn SCNetworkProtocolGetTypeID() -> CFTypeID;

    pub fn SCNetworkProtocolGetConfiguration(protocol: SCNetworkProtocolRef) -> CFDictionaryRef;

    pub fn SCNetworkProtocolGetEnabled(protocol: SCNetworkProtocolRef) -> Boolean;

    pub fn SCNetworkProtocolGetProtocolType(protocol: SCNetworkProtocolRef) -> CFStringRef;

    pub fn SCNetworkProtocolSetConfiguration(
        protocol: SCNetworkProtocolRef,
        config: CFDictionaryRef,
    ) -> Boolean;

    pub fn SCNetworkProtocolSetEnabled(protocol: SCNetworkProtocolRef, enabled: Boolean)
        -> Boolean;

    pub fn SCNetworkServiceGetTypeID() -> CFTypeID;

    pub fn SCNetworkServiceAddProtocolType(
        service: SCNetworkServiceRef,
        protocolType: CFStringRef,
    ) -> Boolean;

    pub fn SCNetworkServiceCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;

    pub fn SCNetworkServiceCopyProtocols(service: SCNetworkServiceRef) -> CFArrayRef;

    pub fn SCNetworkServiceCreate(
        prefs: SCPreferencesRef,
        interface: SCNetworkInterfaceRef,
    ) -> SCNetworkServiceRef;

    pub fn SCNetworkServiceCopy(
        prefs: SCPreferencesRef,
        serviceID: CFStringRef,
    ) -> SCNetworkServiceRef;

    pub fn SCNetworkServiceEstablishDefaultConfiguration(service: SCNetworkServiceRef) -> Boolean;

    pub fn SCNetworkServiceGetEnabled(service: SCNetworkServiceRef) -> Boolean;

    pub fn SCNetworkServiceGetInterface(service: SCNetworkServiceRef) -> SCNetworkInterfaceRef;

    pub fn SCNetworkServiceGetName(service: SCNetworkServiceRef) -> CFStringRef;

    pub fn SCNetworkServiceCopyProtocol(
        service: SCNetworkServiceRef,
        protocolType: CFStringRef,
    ) -> SCNetworkProtocolRef;

    pub fn SCNetworkServiceGetServiceID(service: SCNetworkServiceRef) -> CFStringRef;

    pub fn SCNetworkServiceRemove(service: SCNetworkServiceRef) -> Boolean;

    pub fn SCNetworkServiceRemoveProtocolType(
        service: SCNetworkServiceRef,
        protocolType: CFStringRef,
    ) -> Boolean;

    pub fn SCNetworkServiceSetEnabled(service: SCNetworkServiceRef, enabled: Boolean) -> Boolean;

    pub fn SCNetworkServiceSetName(service: SCNetworkServiceRef, name: CFStringRef) -> Boolean;

    pub fn SCNetworkSetGetTypeID() -> CFTypeID;

    pub fn SCNetworkSetAddService(set: SCNetworkSetRef, service: SCNetworkServiceRef) -> Boolean;

    pub fn SCNetworkSetContainsInterface(
        set: SCNetworkSetRef,
        interface: SCNetworkInterfaceRef,
    ) -> Boolean;

    pub fn SCNetworkSetCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;

    pub fn SCNetworkSetCopyCurrent(prefs: SCPreferencesRef) -> SCNetworkSetRef;

    pub fn SCNetworkSetCopyServices(set: SCNetworkSetRef) -> CFArrayRef;

    pub fn SCNetworkSetCreate(prefs: SCPreferencesRef) -> SCNetworkSetRef;

    pub fn SCNetworkSetCopy(prefs: SCPreferencesRef, setID: CFStringRef) -> SCNetworkSetRef;

    pub fn SCNetworkSetGetName(set: SCNetworkSetRef) -> CFStringRef;

    pub fn SCNetworkSetGetSetID(set: SCNetworkSetRef) -> CFStringRef;

    pub fn SCNetworkSetGetServiceOrder(set: SCNetworkSetRef) -> CFArrayRef;

    pub fn SCNetworkSetRemove(set: SCNetworkSetRef) -> Boolean;

    pub fn SCNetworkSetRemoveService(set: SCNetworkSetRef, service: SCNetworkServiceRef)
        -> Boolean;

    pub fn SCNetworkSetSetCurrent(set: SCNetworkSetRef) -> Boolean;

    pub fn SCNetworkSetSetName(set: SCNetworkSetRef, name: CFStringRef) -> Boolean;

    pub fn SCNetworkSetSetServiceOrder(set: SCNetworkSetRef, newOrder: CFArrayRef) -> Boolean;
}
