//! Bindings for [`SCNetworkConfiguration`].
//!
//! [`SCNetworkConfiguration`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
#[allow(deprecated)]
use crate::sys::kSCNetworkInterfaceTypePPTP;
use crate::sys::{
    self, kSCNetworkInterfaceType6to4, kSCNetworkInterfaceTypeBluetooth,
    kSCNetworkInterfaceTypeBond, kSCNetworkInterfaceTypeEthernet, kSCNetworkInterfaceTypeFireWire,
    kSCNetworkInterfaceTypeIEEE80211, kSCNetworkInterfaceTypeIPSec, kSCNetworkInterfaceTypeIPv4,
    kSCNetworkInterfaceTypeL2TP, kSCNetworkInterfaceTypeModem, kSCNetworkInterfaceTypePPP,
    kSCNetworkInterfaceTypeSerial, kSCNetworkInterfaceTypeVLAN, kSCNetworkInterfaceTypeWWAN,
};
use objc2_core_foundation::{CFArray, CFRetained, CFString};

use crate::preferences::SCPreferences;

/// Represents a network interface.
///
/// See [`SCNetworkInterfaceRef`] and its [methods] for details.
///
/// [`SCNetworkInterfaceRef`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfaceref?language=objc
/// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SCNetworkInterface(pub CFRetained<sys::SCNetworkInterface>);

// TODO: implement all the other methods a SCNetworkInterface has
impl SCNetworkInterface {
    /// Get type of the network interface, if the type is recognized, returns `None` otherwise.
    ///
    /// See [`SCNetworkInterfaceGetInterfaceType`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterfaceType`]: https://developer.apple.com/documentation/systemconfiguration/1517371-scnetworkinterfacegetinterfacety?language=objc
    pub fn interface_type(&self) -> Option<SCNetworkInterfaceType> {
        SCNetworkInterfaceType::from_cfstring(&*self.interface_type_string()?)
    }

    /// Returns the raw interface type identifier.
    ///
    /// See [`SCNetworkInterfaceGetInterfaceType`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterfaceType`]: https://developer.apple.com/documentation/systemconfiguration/1517371-scnetworkinterfacegetinterfacety?language=objc
    pub fn interface_type_string(&self) -> Option<CFRetained<CFString>> {
        self.0.interface_type()
    }

    /// Returns the _BSD_ name for the interface, such as `en0`.
    ///
    /// See [`SCNetworkInterfaceGetBSDName`] for details.
    ///
    /// [`SCNetworkInterfaceGetBSDName`]: https://developer.apple.com/documentation/systemconfiguration/1516854-scnetworkinterfacegetbsdname?language=objc
    pub fn bsd_name(&self) -> Option<CFRetained<CFString>> {
        self.0.bsd_name()
    }

    /// Returns the localized display name for the interface.
    ///
    /// See [`SCNetworkInterfaceGetLocalizedDisplayName`] for details.
    ///
    /// [`SCNetworkInterfaceGetLocalizedDisplayName`]: https://developer.apple.com/documentation/systemconfiguration/1517060-scnetworkinterfacegetlocalizeddi?language=objc
    pub fn display_name(&self) -> Option<CFRetained<CFString>> {
        self.0.localized_display_name()
    }
}

/// Represents the possible network interface types.
///
/// See [_Network Interface Types_] documentation for details.
///
/// [_Network Interface Types_]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration/network_interface_types?language=objc
#[derive(Debug)]
pub enum SCNetworkInterfaceType {
    /// A 6to4 interface.
    SixToFour,
    /// Bluetooth interface.
    Bluetooth,
    /// Bridge interface.
    Bridge,
    /// Ethernet bond interface.
    Bond,
    /// Ethernet interface.
    Ethernet,
    /// FireWire interface.
    FireWire,
    /// IEEE80211 interface.
    IEEE80211,
    /// IPSec interface.
    IPSec,
    /// IrDA interface.
    IrDA,
    /// L2TP interface.
    L2TP,
    /// Modem interface.
    Modem,
    /// PPP interface.
    PPP,
    /// PPTP interface.
    ///
    /// Deprecated, one should use the PPP variant.
    PPTP,
    /// Serial interface.
    Serial,
    /// VLAN interface.
    VLAN,
    /// WWAN interface.
    WWAN,
    /// IPv4 interface.
    IPv4,
}

/// Bridge interface type referred to as `kSCNetworkInterfaceTypeBridge` in private headers.
static BRIDGE_INTERFACE_TYPE_ID: &str = "Bridge";

/// IrDA interface referenced as `kSCNetworkInterfaceTypeIrDA` but deprecated since macOS 12.
static IRDA_INTERFACE_TYPE_ID: &str = "IrDA";

impl SCNetworkInterfaceType {
    /// Tries to construct a type by matching it to string constants used to identify a network
    /// interface type. If no constants match it, `None` is returned.
    #[allow(deprecated)]
    pub fn from_cfstring(type_id: &CFString) -> Option<Self> {
        unsafe {
            if type_id == kSCNetworkInterfaceType6to4 {
                Some(SCNetworkInterfaceType::SixToFour)
            } else if type_id == kSCNetworkInterfaceTypeBluetooth {
                Some(SCNetworkInterfaceType::Bluetooth)
            } else if *type_id == *CFString::from_static_str(BRIDGE_INTERFACE_TYPE_ID) {
                Some(SCNetworkInterfaceType::Bridge)
            } else if type_id == kSCNetworkInterfaceTypeBond {
                Some(SCNetworkInterfaceType::Bond)
            } else if type_id == kSCNetworkInterfaceTypeEthernet {
                Some(SCNetworkInterfaceType::Ethernet)
            } else if type_id == kSCNetworkInterfaceTypeFireWire {
                Some(SCNetworkInterfaceType::FireWire)
            } else if type_id == kSCNetworkInterfaceTypeIEEE80211 {
                Some(SCNetworkInterfaceType::IEEE80211)
            } else if type_id == kSCNetworkInterfaceTypeIPSec {
                Some(SCNetworkInterfaceType::IPSec)
            } else if *type_id == *CFString::from_static_str(IRDA_INTERFACE_TYPE_ID) {
                Some(SCNetworkInterfaceType::IrDA)
            } else if type_id == kSCNetworkInterfaceTypeL2TP {
                Some(SCNetworkInterfaceType::L2TP)
            } else if type_id == kSCNetworkInterfaceTypeModem {
                Some(SCNetworkInterfaceType::Modem)
            } else if type_id == kSCNetworkInterfaceTypePPP {
                Some(SCNetworkInterfaceType::PPP)
            } else if type_id == kSCNetworkInterfaceTypePPTP {
                Some(SCNetworkInterfaceType::PPTP)
            } else if type_id == kSCNetworkInterfaceTypeSerial {
                Some(SCNetworkInterfaceType::Serial)
            } else if type_id == kSCNetworkInterfaceTypeVLAN {
                Some(SCNetworkInterfaceType::VLAN)
            } else if type_id == kSCNetworkInterfaceTypeWWAN {
                Some(SCNetworkInterfaceType::WWAN)
            } else if type_id == kSCNetworkInterfaceTypeIPv4 {
                Some(SCNetworkInterfaceType::IPv4)
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
pub fn get_interfaces() -> impl Iterator<Item = SCNetworkInterface> {
    let array = unsafe {
        CFRetained::cast_unchecked::<CFArray<sys::SCNetworkInterface>>(
            sys::SCNetworkInterface::all(),
        )
    };
    array.into_iter().map(SCNetworkInterface)
}

/// Represents a network service.
///
/// See [`SCNetworkInterfaceRef`] and its [methods] for details.
///
/// [`SCNetworkInterfaceRef`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkserviceref?language=objc
/// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SCNetworkService(pub CFRetained<sys::SCNetworkService>);

impl SCNetworkService {
    /// Returns an array of all network services
    pub fn get_services(prefs: &SCPreferences) -> Vec<Self> {
        if let Some(array) = sys::SCNetworkService::all(&prefs.0) {
            // SAFETY: The array is documented to contain `SCNetworkService`.
            let array = unsafe { array.cast_unchecked::<sys::SCNetworkService>() };
            array.iter().map(Self).collect()
        } else {
            Vec::new()
        }
    }

    /// Returns true if the network service is currently enabled
    pub fn enabled(&self) -> bool {
        self.0.enabled()
    }

    /// Returns the network interface backing this network service, if it has one.
    pub fn network_interface(&self) -> Option<SCNetworkInterface> {
        self.0.interface().map(SCNetworkInterface)
    }

    /// Returns the service identifier.
    pub fn id(&self) -> Option<CFRetained<CFString>> {
        self.0.service_id()
    }
}

/// Represents a complete network configuration for a particular host.
///
/// See [`SCNetworkSet`] for details.
///
/// [`SCNetworkSet`]: https://developer.apple.com/documentation/systemconfiguration/scnetworksetref?language=objc
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SCNetworkSet(pub CFRetained<sys::SCNetworkSet>);

impl SCNetworkSet {
    /// Constructs a new set of network services from the preferences.
    pub fn new(prefs: &SCPreferences) -> Self {
        let set = sys::SCNetworkSet::current(&prefs.0).unwrap();
        Self(set)
    }

    /// Returns an list of network service identifiers, ordered by their priority.
    pub fn service_order(&self) -> CFRetained<CFArray<CFString>> {
        if let Some(array) = self.0.service_order() {
            // SAFETY: The array is documented to contain `CFString`.
            unsafe { CFRetained::cast_unchecked::<CFArray<CFString>>(array) }
        } else {
            CFArray::empty()
        }
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
        for iface in get_interfaces() {
            if iface.interface_type().is_none() {
                panic!(
                    "Interface  {:?} ({:?}) has unrecognized type {:?}",
                    iface.display_name(),
                    iface.bsd_name(),
                    iface.interface_type_string()
                )
            }
        }
    }

    #[test]
    fn test_service_order() {
        let prefs = SCPreferences::default(&CFString::from_static_str("test"));
        let services = SCNetworkService::get_services(&prefs);
        let set = SCNetworkSet::new(&prefs);
        let service_order = set.service_order();

        assert!(service_order.iter().all(|service_id| {
            services
                .iter()
                .any(|service| service.id().as_deref() == Some(&*service_id))
        }))
    }
}
