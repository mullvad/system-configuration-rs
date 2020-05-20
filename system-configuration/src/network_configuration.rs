//! Bindings for [`SCNetworkConfiguration`].
//!
//! [`SCNetworkConfiguration`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
//!
use core_foundation::{
    array::CFArray,
    base::{CFRelease, CFRetain, TCFType},
    string::CFString,
};
use system_configuration_sys::network_configuration::{
    SCNetworkInterfaceCopyAll, SCNetworkInterfaceGetBSDName, SCNetworkInterfaceGetInterfaceType,
    SCNetworkInterfaceGetLocalizedDisplayName, SCNetworkInterfaceRef,
};

use std::ffi::c_void;

/// Represents a network interface.
///
/// See [`SCNetworkInterfaceRef`] and it's [methods] for details.
///
/// [`SCNetworkInterfaceRef`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfaceref?language=objc
/// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
pub struct NetworkInterface {
    ptr: SCNetworkInterfaceRef,
}

// TODO: implement all the other methods a NetworkInterface has
impl NetworkInterface {
    unsafe fn from_void_ptr(ptr: *const c_void) -> Self {
        NetworkInterface { ptr }
    }

    /// Get type of the network interface, if the type is recognized, returns `None` otherwise.
    ///
    /// See [`SCNetworkInterfaceGetInterfaceType`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterfaceType`]: https://developer.apple.com/documentation/systemconfiguration/1517371-scnetworkinterfacegetinterfacety?language=objc
    pub fn interface_type(&self) -> Option<NetworkInterfaceType> {
        NetworkInterfaceType::from_cfstring(&self.raw_interface_type())
    }

    /// Returns the raw interface type identifier.
    ///
    /// See [`SCNetworkInterfaceGetInterfaceType`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterfaceType`]: https://developer.apple.com/documentation/systemconfiguration/1517371-scnetworkinterfacegetinterfacety?language=objc
    pub fn raw_interface_type(&self) -> CFString {
        unsafe { CFString::wrap_under_create_rule(SCNetworkInterfaceGetInterfaceType(self.ptr)) }
    }

    /// Returns the _BSD_ name for the interface, such as `en0`.
    ///
    /// See [`SCNetworkInterfaceGetBSDName`] for details.
    ///
    /// [`SCNetworkInterfaceGetBSDName`]: https://developer.apple.com/documentation/systemconfiguration/1516854-scnetworkinterfacegetbsdname?language=objc
    pub fn bsd_name(&self) -> CFString {
        unsafe { CFString::wrap_under_get_rule(SCNetworkInterfaceGetBSDName(self.ptr)) }
    }

    /// Returns the localized display name for the interface.
    ///
    /// See [`SCNetworkInterfaceGetLocalizedDisplayName`] for details.
    ///
    /// [`SCNetworkInterfaceGetLocalizedDisplayName`]: https://developer.apple.com/documentation/systemconfiguration/1517060-scnetworkinterfacegetlocalizeddi?language=objc
    pub fn display_name(&self) -> CFString {
        unsafe {
            CFString::wrap_under_get_rule(SCNetworkInterfaceGetLocalizedDisplayName(self.ptr))
        }
    }
}

impl Drop for NetworkInterface {
    fn drop(&mut self) {
        unsafe { CFRelease(self.ptr) }
    }
}

/// Represents the possible network interface types.
///
/// See [_Network Interface Types_] documentation for details.
///
/// [_Network Interface Types_]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration/network_interface_types?language=objc
#[derive(Debug)]
pub enum NetworkInterfaceType {
    /// A 6to4 interface.
    SixToFour,
    /// Bluetooth intreface
    Bluetooth,
    /// Ethernet bond interface
    Bond,
    /// Bridge interface
    Bridge,
    /// Ethernet interface
    Ethernet,
    /// FireWire interface
    FireWire,
    /// IEEE80211 interface
    IEEE80211,
    /// IPSec interface
    IPSec,
    /// IrDA interface
    IrDA,
    /// L2TP interface
    L2TP,
    /// Modem interface
    Modem,
    /// PPP interface
    PPP,
    /// PPTP interface
    /// Deprecated, one should use the PPP variant.
    PPTP,
    /// Serial interface
    Serial,
    /// VLAN interace
    VLAN,
    /// WWAN interace
    WWAN,
    /// IPv4 interface
    IPv4,
}

impl NetworkInterfaceType {
    /// Tries to construct a type by matching it to string constants used to identify a network
    /// interface type. If no constants match it, `None` is returned.
    pub fn from_cfstring(type_id: &CFString) -> Option<Self> {
        use system_configuration_sys::network_configuration::*;

        // Filthy hack to not bump the reference counter every time a comparison with a static
        // string is made.
        let id_is_equal_to = |const_str| -> bool {
            let const_str = unsafe { CFString::wrap_under_create_rule(const_str) };
            let ret = &const_str == type_id;
            std::mem::forget(const_str);
            ret
        };
        unsafe {
            if id_is_equal_to(kSCNetworkInterfaceType6to4) {
                Some(Self::SixToFour)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeBluetooth) {
                Some(Self::Bluetooth)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeBond) {
                Some(Self::Bond)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeBridge) {
                Some(Self::Bridge)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeEthernet) {
                Some(Self::Ethernet)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeFireWire) {
                Some(Self::FireWire)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIEEE80211) {
                Some(Self::IEEE80211)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIPSec) {
                Some(Self::IPSec)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIrDA) {
                Some(Self::IrDA)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeL2TP) {
                Some(Self::L2TP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeModem) {
                Some(Self::Modem)
            } else if id_is_equal_to(kSCNetworkInterfaceTypePPP) {
                Some(Self::PPP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypePPTP) {
                Some(Self::PPTP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeSerial) {
                Some(Self::Serial)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeVLAN) {
                Some(Self::VLAN)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeWWAN) {
                Some(Self::WWAN)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIPv4) {
                Some(Self::IPv4)
            } else {
                None
            }
        }
    }
}

/// Retrieve all current network interfaces
///
/// See [`SCNetworkInterfaceCopyAll`] for more details.
///
/// [`SCNetworkInterfaceCopyAll`]: https://developer.apple.com/documentation/systemconfiguration/1517090-scnetworkinterfacecopyall?language=objc
pub fn get_interfaces() -> Vec<NetworkInterface> {
    unsafe {
        let array: CFArray<SCNetworkInterfaceRef> =
            CFArray::wrap_under_create_rule(SCNetworkInterfaceCopyAll());
        array
            .get_all_values()
            .into_iter()
            .map(|item| {
                CFRetain(item);
                NetworkInterface::from_void_ptr(item)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_all_interfaces() {
        let _ = get_interfaces();
    }

    #[test]
    fn test_get_type() {
        for iface in get_interfaces().into_iter() {
            if iface.interface_type().is_none() {
                panic!(
                    "Interface  {} ({}) has unrecognized type {}",
                    iface.display_name(),
                    iface.bsd_name(),
                    iface.raw_interface_type()
                )
            }
        }
    }
}
