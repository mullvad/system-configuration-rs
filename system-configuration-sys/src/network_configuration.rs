use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::Boolean;
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::string::CFStringRef;

use preferences::SCPreferencesRef;

use std::os::raw::{c_int, c_void};

pub type __SCNetworkInterface = c_void;
pub type SCNetworkInterfaceRef = *const __SCNetworkInterface;
pub type SCBondInterfaceRef = SCNetworkInterfaceRef;
pub type SCVLANInterfaceRef = SCNetworkInterfaceRef;

pub type __SCBondStatus = c_void;
pub type SCBondStatusRef = *const __SCBondStatus;

pub type __SCNetworkProtocol = c_void;
pub type SCNetworkProtocolRef = *const __SCNetworkProtocol;

pub type __SCNetworkService = c_void;
pub type SCNetworkServiceRef = *const __SCNetworkService;

pub type __SCNetworkSet = c_void;
pub type SCNetworkSetRef = *const __SCNetworkSet;

#[link(name = "SystemConfiguration", kind = "framework")]
extern "C" {
    pub fn SCNetworkServiceCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;
    pub fn SCNetworkServiceCopy(
        prefs: SCPreferencesRef,
        serviceID: CFStringRef,
    ) -> SCNetworkServiceRef;
    pub fn SCNetworkServiceGetEnabled(service: SCNetworkServiceRef) -> Boolean;
    pub fn SCNetworkServiceGetInterface(service: SCNetworkServiceRef) -> SCNetworkInterfaceRef;
    pub fn SCNetworkServiceGetName(service: SCNetworkServiceRef) -> CFStringRef;
    pub fn SCNetworkServiceGetServiceID(service: SCNetworkServiceRef) -> CFStringRef;
    pub fn SCNetworkSetGetServiceOrder(set: SCNetworkSetRef) -> CFArrayRef;
    pub fn SCNetworkSetCopyServices(set: SCNetworkSetRef) -> CFArrayRef;
    pub fn SCNetworkSetCopyCurrent(prefs: SCPreferencesRef) -> SCNetworkSetRef;

    pub fn SCNetworkInterfaceCopyAll() -> CFArrayRef;
    pub fn SCNetworkInterfaceCopyMTU(
        interface: SCNetworkInterfaceRef,
        mtu_cur: *mut c_int,
        mtu_min: *mut c_int,
        mtu_max: *mut c_int,
    ) -> Boolean;
    pub fn SCNetworkInterfaceCopyMediaOptions(
        interface: SCNetworkInterfaceRef,
        urrent: *mut CFDictionaryRef,
        active: *mut CFDictionaryRef,
        available: *mut CFArrayRef,
        filter: Boolean,
    ) -> Boolean;
    pub fn SCNetworkInterfaceGetBSDName(interface: SCNetworkInterfaceRef) -> CFStringRef;
    pub fn SCNetworkInterfaceGetInterfaceType(interface: SCNetworkInterfaceRef) -> CFStringRef;
    pub fn SCNetworkInterfaceGetHardwareAddressString(
        interface: SCNetworkInterfaceRef,
    ) -> CFStringRef;

    pub fn SCNetworkInterfaceGetConfiguration(interface: SCNetworkInterfaceRef) -> CFDictionaryRef;
    pub fn SCNetworkInterfaceGetExtendedConfiguration(
        interface: SCNetworkInterfaceRef,
        extendedType: CFStringRef,
    ) -> CFDictionaryRef;

    pub fn SCNetworkInterfaceSetConfiguration(
        interface: SCNetworkInterfaceRef,
        config: CFDictionaryRef,
    ) -> Boolean;
    pub fn SCNetworkInterfaceSetExtendedConfiguration(
        interface: SCNetworkInterfaceRef,
        extendedType: CFStringRef,
        config: CFDictionaryRef,
    ) -> Boolean;
}
