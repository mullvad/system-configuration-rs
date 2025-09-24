use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::Boolean;
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::string::CFStringRef;
use crate::network_configuration::SCNetworkInterfaceRef;
use crate::preferences::SCPreferencesRef;

pub type SCBridgeInterfaceRef = SCNetworkInterfaceRef;

extern "C" {
    pub static kSCNetworkInterfaceTypeBridge: CFStringRef;

    pub static kSCNetworkInterfaceTypeLoopback: CFStringRef;

    pub static kSCNetworkInterfaceIPv4: SCNetworkInterfaceRef;

    pub static kSCNetworkInterfaceTypeVPN: CFStringRef;

    pub static SCBridgeInterfaceRef: SCNetworkInterfaceRef;
}

extern "C" {
    pub fn SCBridgeInterfaceCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;

    pub fn SCBridgeInterfaceCopyAvailableMemberInterfaces(prefs: SCPreferencesRef) -> CFArrayRef;

    pub fn SCBridgeInterfaceCreate(prefs: SCPreferencesRef) -> SCBridgeInterfaceRef;

    pub fn SCBridgeInterfaceRemove(bridge: SCBridgeInterfaceRef) -> Boolean;

    pub fn SCBridgeInterfaceGetMemberInterfaces(bridge: SCBridgeInterfaceRef) -> CFArrayRef;

    pub fn SCBridgeInterfaceGetOptions(bridge: SCBridgeInterfaceRef) -> CFDictionaryRef;

    pub fn SCBridgeInterfaceSetAllowConfiguredMembers(bridge: SCBridgeInterfaceRef,
                                                      enable: Boolean) -> Boolean;

    pub fn SCBridgeInterfaceGetAllowConfiguredMembers(bridge: SCBridgeInterfaceRef) -> Boolean;

    pub fn SCBridgeInterfaceSetMemberInterfaces(bridge: SCBridgeInterfaceRef,
                                                members: CFArrayRef) -> Boolean;

    pub fn SCBridgeInterfaceSetLocalizedDisplayName(bridge: SCBridgeInterfaceRef,
                                                    newName: CFStringRef) -> Boolean;

    pub fn SCBridgeInterfaceSetOptions(bridge: SCBridgeInterfaceRef,
                                       newOptions: CFDictionaryRef) -> Boolean;
}