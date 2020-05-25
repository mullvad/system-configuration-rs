//! Bindings for [`SCNetworkConfiguration`].
//!
//! [`SCNetworkConfiguration`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
//!
use core_foundation::{
    array::CFArray,
    base::{ItemRef, TCFType},
    string::CFString,
};
use system_configuration_sys::network_configuration::{
    SCNetworkInterfaceCopyAll, SCNetworkInterfaceGetBSDName, SCNetworkInterfaceGetInterfaceType,
    SCNetworkInterfaceGetLocalizedDisplayName, SCNetworkInterfaceGetTypeID, SCNetworkInterfaceRef,
};

core_foundation::declare_TCFType!(
    /// Represents a network interface.
    ///
    /// See [`SCNetworkInterfaceRef`] and it's [methods] for details.
    ///
    /// [`SCNetworkInterfaceRef`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfaceref?language=objc
    /// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
    NetworkInterface,
    SCNetworkInterfaceRef
);
core_foundation::impl_TCFType!(
    NetworkInterface,
    SCNetworkInterfaceRef,
    SCNetworkInterfaceGetTypeID
);

// TODO: implement all the other methods a NetworkInterface has
impl NetworkInterface {
    /// Get type of the network interface, if the type is recognized, returns `None` otherwise.
    ///
    /// See [`SCNetworkInterfaceGetInterfaceType`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterfaceType`]: https://developer.apple.com/documentation/systemconfiguration/1517371-scnetworkinterfacegetinterfacety?language=objc
    pub fn interface_type(&self) -> Option<NetworkInterfaceType> {
        NetworkInterfaceType::from_cfstring(&self.raw_interface_type()?)
    }

    /// Returns the raw interface type identifier.
    ///
    /// See [`SCNetworkInterfaceGetInterfaceType`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterfaceType`]: https://developer.apple.com/documentation/systemconfiguration/1517371-scnetworkinterfacegetinterfacety?language=objc
    pub fn raw_interface_type(&self) -> Option<CFString> {
        unsafe {
            unsafe_map_ptr(
                SCNetworkInterfaceGetInterfaceType(self.0),
                CFString::wrap_under_get_rule,
            )
        }
    }

    /// Returns the _BSD_ name for the interface, such as `en0`.
    ///
    /// See [`SCNetworkInterfaceGetBSDName`] for details.
    ///
    /// [`SCNetworkInterfaceGetBSDName`]: https://developer.apple.com/documentation/systemconfiguration/1516854-scnetworkinterfacegetbsdname?language=objc
    pub fn bsd_name(&self) -> Option<CFString> {
        unsafe {
            unsafe_map_ptr(
                SCNetworkInterfaceGetBSDName(self.0),
                CFString::wrap_under_get_rule,
            )
        }
    }

    /// Returns the localized display name for the interface.
    ///
    /// See [`SCNetworkInterfaceGetLocalizedDisplayName`] for details.
    ///
    /// [`SCNetworkInterfaceGetLocalizedDisplayName`]: https://developer.apple.com/documentation/systemconfiguration/1517060-scnetworkinterfacegetlocalizeddi?language=objc
    pub fn display_name(&self) -> Option<CFString> {
        unsafe {
            unsafe_map_ptr(
                SCNetworkInterfaceGetLocalizedDisplayName(self.0),
                CFString::wrap_under_get_rule,
            )
        }
    }
}

unsafe fn unsafe_map_ptr<P, T>(ptr: *const P, f: unsafe fn(*const P) -> T) -> Option<T> {
    if ptr.is_null() {
        None
    } else {
        Some(f(ptr))
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

        let id_is_equal_to = |const_str| -> bool {
            let const_str = unsafe { CFString::wrap_under_get_rule(const_str) };
            &const_str == type_id
        };
        unsafe {
            if id_is_equal_to(kSCNetworkInterfaceType6to4) {
                Some(NetworkInterfaceType::SixToFour)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeBluetooth) {
                Some(NetworkInterfaceType::Bluetooth)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeBond) {
                Some(NetworkInterfaceType::Bond)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeBridge) {
                Some(NetworkInterfaceType::Bridge)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeEthernet) {
                Some(NetworkInterfaceType::Ethernet)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeFireWire) {
                Some(NetworkInterfaceType::FireWire)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIEEE80211) {
                Some(NetworkInterfaceType::IEEE80211)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIPSec) {
                Some(NetworkInterfaceType::IPSec)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIrDA) {
                Some(NetworkInterfaceType::IrDA)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeL2TP) {
                Some(NetworkInterfaceType::L2TP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeModem) {
                Some(NetworkInterfaceType::Modem)
            } else if id_is_equal_to(kSCNetworkInterfaceTypePPP) {
                Some(NetworkInterfaceType::PPP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypePPTP) {
                Some(NetworkInterfaceType::PPTP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeSerial) {
                Some(NetworkInterfaceType::Serial)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeVLAN) {
                Some(NetworkInterfaceType::VLAN)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeWWAN) {
                Some(NetworkInterfaceType::WWAN)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIPv4) {
                Some(NetworkInterfaceType::IPv4)
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
        CFArray::<SCNetworkInterfaceRef>::wrap_under_create_rule(SCNetworkInterfaceCopyAll())
            .iter()
            .map(|item: ItemRef<SCNetworkInterfaceRef>| {
                NetworkInterface::wrap_under_get_rule(*item)
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
                    "Interface  {:?} ({:?}) has unrecognized type {:?}",
                    iface.display_name(),
                    iface.bsd_name(),
                    iface.raw_interface_type()
                )
            }
        }
    }
}
