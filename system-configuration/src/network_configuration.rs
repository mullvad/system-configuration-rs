//! Bindings for [`SCNetworkConfiguration`].
//!
//! [`SCNetworkConfiguration`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc

use std::mem;
use core_foundation::{
    array::CFArray,
    base::{Boolean, TCFType, ToVoid, TCFTypeRef, CFType},
    string::CFString,
    dictionary::CFDictionary,
};
use sys::network_configuration::{SCNetworkInterfaceCopyAll, SCNetworkInterfaceGetBSDName, SCNetworkInterfaceGetHardwareAddressString, SCNetworkInterfaceGetInterface, SCNetworkInterfaceGetInterfaceType, SCNetworkInterfaceGetLocalizedDisplayName, SCNetworkInterfaceGetSupportedInterfaceTypes, SCNetworkInterfaceGetSupportedProtocolTypes, SCNetworkInterfaceGetTypeID, SCNetworkInterfaceRef, SCNetworkProtocolGetConfiguration, SCNetworkProtocolGetEnabled, SCNetworkProtocolGetProtocolType, SCNetworkProtocolGetTypeID, SCNetworkProtocolRef, SCNetworkProtocolSetConfiguration, SCNetworkProtocolSetEnabled, SCNetworkServiceAddProtocolType, SCNetworkServiceCopy, SCNetworkServiceCopyAll, SCNetworkServiceCopyProtocol, SCNetworkServiceCopyProtocols, SCNetworkServiceCreate, SCNetworkServiceEstablishDefaultConfiguration, SCNetworkServiceGetEnabled, SCNetworkServiceGetInterface, SCNetworkServiceGetServiceID, SCNetworkServiceGetTypeID, SCNetworkServiceRef, SCNetworkServiceRemove, SCNetworkServiceSetEnabled, SCNetworkSetAddService, SCNetworkSetContainsInterface, SCNetworkSetCopy, SCNetworkSetCopyAll, SCNetworkSetCopyCurrent, SCNetworkSetCopyServices, SCNetworkSetGetName, SCNetworkSetGetServiceOrder, SCNetworkSetGetSetID, SCNetworkSetGetTypeID, SCNetworkSetRef, SCNetworkSetRemove, SCNetworkSetRemoveService, SCNetworkSetSetCurrent, SCNetworkSetSetServiceOrder};

use crate::preferences::SCPreferences;
use crate::helpers::create_empty_array;

#[cfg(feature = "private")]
pub use crate::private::network_configuration_private::*;

/// Trait for all subclasses of [`SCNetworkInterface`].
///
/// [`SCNetworkInterface`]: struct.SCNetworkInterface.html
pub unsafe trait SCNetworkInterfaceSubClass: TCFType {
    /// Determines what the type subclass of [`SCNetworkInterface`] this is.
    const INTERFACE_TYPE: SCNetworkInterfaceType;

    /// Create an instance of the superclass type [`SCNetworkInterface`] for this instance.
    ///
    /// [`SCNetworkInterface`]: struct.SCNetworkInterface.html
    #[inline]
    fn to_SCNetworkInterface(&self) -> SCNetworkInterface {
        unsafe { SCNetworkInterface::wrap_under_get_rule(self.as_concrete_TypeRef().as_void_ptr()) }
    }

    /// Equal to [`to_SCNetworkInterface`], but consumes self and avoids changing the reference count.
    ///
    /// [`to_SCNetworkInterface`]: #method.to_SCNetworkInterface
    #[inline]
    fn into_SCNetworkInterface(self) -> SCNetworkInterface
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef().as_void_ptr();
        mem::forget(self);
        unsafe { SCNetworkInterface::wrap_under_create_rule(reference) }
    }
}

core_foundation::declare_TCFType!(
    /// Represents a network interface.
    ///
    /// See [`SCNetworkInterfaceRef`] and its [methods] for details.
    ///
    /// [`SCNetworkInterfaceRef`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfaceref?language=objc
    /// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
    SCNetworkInterface,
    SCNetworkInterfaceRef
);
core_foundation::impl_TCFType!(
    SCNetworkInterface,
    SCNetworkInterfaceRef,
    SCNetworkInterfaceGetTypeID
);
core_foundation::impl_CFTypeDescription!(SCNetworkInterface);

// TODO: implement all the other methods a SCNetworkInterface has
impl SCNetworkInterface {
    /// Retrieve all current network interfaces
    ///
    /// See [`SCNetworkInterfaceCopyAll`] for more details.
    ///
    /// [`SCNetworkInterfaceCopyAll`]: https://developer.apple.com/documentation/systemconfiguration/1517090-scnetworkinterfacecopyall?language=objc
    pub fn get_interfaces() -> CFArray<Self> {
        get_interfaces()
    }

    /// Try to downcast the [`SCNetworkInterface`] to a subclass. Checking if the instance is the
    /// correct subclass happens at runtime and `None` is returned if it is not the correct type.
    /// Works similar to [`CFPropertyList::downcast`](core_foundation::propertylist::CFPropertyList::downcast)
    /// and [`CFType::downcast`](core_foundation::base::CFType::downcast).
    pub fn downcast_SCNetworkInterface<T: SCNetworkInterfaceSubClass>(&self) -> Option<T> {
        if self.instance_of::<T>() && self.interface_type()? == T::INTERFACE_TYPE {
            unsafe {
                let subclass_ref = T::Ref::from_void_ptr(self.0);
                Some(T::wrap_under_get_rule(subclass_ref))
            }
        } else {
            None
        }
    }

    /// Similar to [`downcast_SCNetworkInterface`], but consumes self and can thus avoid touching
    /// the retain count.
    ///
    /// [`downcast_SCNetworkInterface`]: #method.downcast_SCNetworkInterface
    pub fn downcast_into_SCNetworkInterface<T: SCNetworkInterfaceSubClass>(self) -> Option<T> {
        if self.instance_of::<T>() && self.interface_type()? == T::INTERFACE_TYPE {
            unsafe {
                let subclass_ref = T::Ref::from_void_ptr(self.0);
                mem::forget(self);
                Some(T::wrap_under_create_rule(subclass_ref))
            }
        } else {
            None
        }
    }

    /// Get type of the network interface, if the type is recognized, returns `None` otherwise.
    ///
    /// See [`SCNetworkInterfaceGetInterfaceType`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterfaceType`]: https://developer.apple.com/documentation/systemconfiguration/1517371-scnetworkinterfacegetinterfacety?language=objc
    pub fn interface_type(&self) -> Option<SCNetworkInterfaceType> {
        SCNetworkInterfaceType::from_cfstring(&self.interface_type_string()?)
    }

    /// Returns the raw interface type identifier.
    ///
    /// See [`SCNetworkInterfaceGetInterfaceType`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterfaceType`]: https://developer.apple.com/documentation/systemconfiguration/1517371-scnetworkinterfacegetinterfacety?language=objc
    pub fn interface_type_string(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkInterfaceGetInterfaceType(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns the _BSD_ name for the interface, such as `en0`.
    ///
    /// See [`SCNetworkInterfaceGetBSDName`] for details.
    ///
    /// [`SCNetworkInterfaceGetBSDName`]: https://developer.apple.com/documentation/systemconfiguration/1516854-scnetworkinterfacegetbsdname?language=objc
    pub fn bsd_name(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkInterfaceGetBSDName(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns the underlying interface, for layered network interfaces. Or `None` if the specified
    /// interface is a leaf interface.
    ///
    /// See [`SCNetworkInterfaceGetInterface`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterface`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfacegetinterface(_:)?language=objc
    pub fn underlying_interface(&self) -> Option<Self> {
        unsafe {
            let ptr = SCNetworkInterfaceGetInterface(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(Self::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns a displayable link layer address for the specified interface, i.e. the hardware
    /// MAC (Media Access Control) address for the interface.
    ///
    /// See [`SCNetworkInterfaceGetHardwareAddressString`] for details.
    ///
    /// [`SCNetworkInterfaceGetHardwareAddressString`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfacegethardwareaddressstring(_:)?language=objc
    pub fn hardware_address_string(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkInterfaceGetHardwareAddressString(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }
    /// Returns the localized display name for the interface.
    ///
    /// See [`SCNetworkInterfaceGetLocalizedDisplayName`] for details.
    ///
    /// [`SCNetworkInterfaceGetLocalizedDisplayName`]: https://developer.apple.com/documentation/systemconfiguration/1517060-scnetworkinterfacegetlocalizeddi?language=objc
    pub fn display_name(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkInterfaceGetLocalizedDisplayName(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Get all the raw network interface type identifiers, such as PPP, that can be layered on top
    /// of the specified interface.
    ///
    /// See [`SCNetworkInterfaceGetSupportedInterfaceTypes`] for details.
    ///
    /// [`SCNetworkInterfaceGetSupportedInterfaceTypes`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfacegetsupportedinterfacetypes(_:)?language=objc
    pub fn supported_interface_type_strings(&self) -> CFArray<CFString> {
        unsafe {
            let array_ptr = SCNetworkInterfaceGetSupportedInterfaceTypes(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<CFString>::wrap_under_get_rule(array_ptr)
        }
    }

    /// Get all the raw network protocol type identifiers, such as IPv4 and IPv6, that can be
    /// layered on top of the specified interface.
    ///
    /// See [`SCNetworkInterfaceGetSupportedProtocolTypes`] for details.
    ///
    /// [`SCNetworkInterfaceGetSupportedProtocolTypes`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfacegetsupportedprotocoltypes(_:)?language=objc
    pub fn supported_protocol_type_strings(&self) -> CFArray<CFString> {
        unsafe {
            let array_ptr = SCNetworkInterfaceGetSupportedProtocolTypes(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<CFString>::wrap_under_get_rule(array_ptr)
        }
    }
}

/// Represents the possible network interface types.
///
/// See [_Network Interface Types_] documentation for details.
///
/// [_Network Interface Types_]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration/network_interface_types?language=objc
#[derive(Debug, PartialEq, Eq)]
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
#[cfg(not(feature = "private"))]
static BRIDGE_INTERFACE_TYPE_ID: &str = "Bridge";


/// IrDA interface referenced as `kSCNetworkInterfaceTypeIrDA` but deprecated since macOS 12.
static IRDA_INTERFACE_TYPE_ID: &str = "IrDA";

impl SCNetworkInterfaceType {
    /// Tries to construct a type by matching it to string constants used to identify a network
    /// interface type. If no constants match it, `None` is returned.
    pub fn from_cfstring(type_id: &CFString) -> Option<Self> {
        #[cfg(feature = "private")]
        use system_configuration_sys::private::network_configuration_private::*;
        use system_configuration_sys::network_configuration::*;

        let id_is_equal_to = |const_str| -> bool {
            let const_str = unsafe { CFString::wrap_under_get_rule(const_str) };
            &const_str == type_id
        };
        unsafe {
            if id_is_equal_to(kSCNetworkInterfaceType6to4) {
                Some(SCNetworkInterfaceType::SixToFour)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeBluetooth) {
                Some(SCNetworkInterfaceType::Bluetooth)
            } else if {
                #[cfg(feature = "private")]
                let matches = id_is_equal_to(kSCNetworkInterfaceTypeBridge);

                #[cfg(not(feature = "private"))]
                let matches = type_id == &BRIDGE_INTERFACE_TYPE_ID;

                matches
            } {
                Some(SCNetworkInterfaceType::Bridge)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeBond) {
                Some(SCNetworkInterfaceType::Bond)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeEthernet) {
                Some(SCNetworkInterfaceType::Ethernet)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeFireWire) {
                Some(SCNetworkInterfaceType::FireWire)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIEEE80211) {
                Some(SCNetworkInterfaceType::IEEE80211)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIPSec) {
                Some(SCNetworkInterfaceType::IPSec)
            } else if type_id == &IRDA_INTERFACE_TYPE_ID {
                Some(SCNetworkInterfaceType::IrDA)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeL2TP) {
                Some(SCNetworkInterfaceType::L2TP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeModem) {
                Some(SCNetworkInterfaceType::Modem)
            } else if id_is_equal_to(kSCNetworkInterfaceTypePPP) {
                Some(SCNetworkInterfaceType::PPP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypePPTP) {
                Some(SCNetworkInterfaceType::PPTP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeSerial) {
                Some(SCNetworkInterfaceType::Serial)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeVLAN) {
                Some(SCNetworkInterfaceType::VLAN)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeWWAN) {
                Some(SCNetworkInterfaceType::WWAN)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIPv4) {
                Some(SCNetworkInterfaceType::IPv4)
            } else {
                None
            }
        }
    }

    /// Returns the string constants used to identify this network interface type.
    pub fn to_cfstring(&self) -> CFString {
        #[cfg(feature = "private")]
        use system_configuration_sys::private::network_configuration_private::*;
        use system_configuration_sys::network_configuration::*;
        let wrap_const = |const_str| unsafe { CFString::wrap_under_get_rule(const_str) };
        unsafe {
            match self {
                SCNetworkInterfaceType::SixToFour => wrap_const(kSCNetworkInterfaceType6to4),
                SCNetworkInterfaceType::Bluetooth => wrap_const(kSCNetworkInterfaceTypeBluetooth),
                SCNetworkInterfaceType::Bridge => {
                    #[cfg(feature = "private")]
                    let val = wrap_const(kSCNetworkInterfaceTypeBridge);

                    #[cfg(not(feature = "private"))]
                    let val = BRIDGE_INTERFACE_TYPE_ID.into();

                    val
                },
                SCNetworkInterfaceType::Bond => wrap_const(kSCNetworkInterfaceTypeBond),
                SCNetworkInterfaceType::Ethernet => wrap_const(kSCNetworkInterfaceTypeEthernet),
                SCNetworkInterfaceType::FireWire => wrap_const(kSCNetworkInterfaceTypeFireWire),
                SCNetworkInterfaceType::IEEE80211 => wrap_const(kSCNetworkInterfaceTypeIEEE80211),
                SCNetworkInterfaceType::IPSec => wrap_const(kSCNetworkInterfaceTypeIPSec),
                SCNetworkInterfaceType::IrDA => IRDA_INTERFACE_TYPE_ID.into(),
                SCNetworkInterfaceType::L2TP => wrap_const(kSCNetworkInterfaceTypeL2TP),
                SCNetworkInterfaceType::Modem => wrap_const(kSCNetworkInterfaceTypeModem),
                SCNetworkInterfaceType::PPP => wrap_const(kSCNetworkInterfaceTypePPP),
                SCNetworkInterfaceType::PPTP => wrap_const(kSCNetworkInterfaceTypePPTP),
                SCNetworkInterfaceType::Serial => wrap_const(kSCNetworkInterfaceTypeSerial),
                SCNetworkInterfaceType::VLAN => wrap_const(kSCNetworkInterfaceTypeVLAN),
                SCNetworkInterfaceType::WWAN => wrap_const(kSCNetworkInterfaceTypeWWAN),
                SCNetworkInterfaceType::IPv4 => wrap_const(kSCNetworkInterfaceTypeIPv4),
            }
        }
    }
}

/// Retrieve all current network interfaces
///
/// See [`SCNetworkInterfaceCopyAll`] for more details.
///
/// [`SCNetworkInterfaceCopyAll`]: https://developer.apple.com/documentation/systemconfiguration/1517090-scnetworkinterfacecopyall?language=objc
pub fn get_interfaces() -> CFArray<SCNetworkInterface> {
    unsafe { CFArray::<SCNetworkInterface>::wrap_under_create_rule(SCNetworkInterfaceCopyAll()) }
}

core_foundation::declare_TCFType!(
    /// Represents a network protocol.
    ///
    /// See [`SCNetworkProtocolRef`] and its [methods] for details.
    ///
    /// [`SCNetworkProtocolRef`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkprotocol?language=objc
    /// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
    SCNetworkProtocol,
    SCNetworkProtocolRef
);
core_foundation::impl_TCFType!(
    SCNetworkProtocol,
    SCNetworkProtocolRef,
    SCNetworkProtocolGetTypeID
);
core_foundation::impl_CFTypeDescription!(SCNetworkProtocol);

// TODO: implement all the other methods a SCNetworkProtocol has
impl SCNetworkProtocol {
    /// Returns a [`bool`] value indicating whether the specified protocol is enabled.
    pub fn enabled(&self) -> bool {
        unsafe { SCNetworkProtocolGetEnabled(self.0) != 0 }
    }

    /// Get type of the network protocol, if the type is recognized, returns `None` otherwise.
    ///
    /// See [`SCNetworkProtocolGetProtocolType`] for details.
    ///
    /// [`SCNetworkProtocolGetProtocolType`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkprotocolgetprotocoltype(_:)?language=objc
    pub fn protocol_type(&self) -> Option<SCNetworkProtocolType> {
        SCNetworkProtocolType::from_cfstring(&self.protocol_type_string()?)
    }

    /// Returns the raw protocol type identifier.
    ///
    /// See [`SCNetworkProtocolGetProtocolType`] for details.
    ///
    /// [`SCNetworkProtocolGetProtocolType`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkprotocolgetprotocoltype(_:)?language=objc
    pub fn protocol_type_string(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkProtocolGetProtocolType(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns the configuration settings associated with the specified protocol. Or `None` if no
    /// configuration settings are associated with the protocol or an error occurred.
    ///
    /// See [`SCNetworkProtocolGetConfiguration`] for details.
    ///
    /// [`SCNetworkProtocolGetConfiguration`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkprotocolgetconfiguration(_:)?language=objc
    pub fn configuration(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let dictionary_ref = SCNetworkProtocolGetConfiguration(self.as_concrete_TypeRef());
            if !dictionary_ref.is_null() {
                Some(CFDictionary::wrap_under_get_rule(dictionary_ref))
            } else {
                None
            }
        }
    }

    /// Enables or disables the specified protocol.
    ///
    /// Returns: `true` if the enabled status was saved; `false` if an error occurred.
    pub fn set_enabled(&mut self, enabled: bool) -> bool {
        (unsafe { SCNetworkProtocolSetEnabled(self.0, enabled as Boolean) }) != 0
    }

    /// Stores the configuration settings for the specified network protocol.
    ///
    /// Returns: `true` if the configuration was stored; `false` if an error occurred.
    pub fn set_configuration(&mut self, config: &CFDictionary<CFString, CFType>) -> bool {
        (unsafe { SCNetworkProtocolSetConfiguration(self.0, config.as_concrete_TypeRef()) }) != 0
    }
}

/// Represents the possible network protocol types.
///
/// See [_Network Protocol Types_] documentation for details.
///
/// [_Network Protocol Types_]: https://developer.apple.com/documentation/systemconfiguration/network-protocol-types?language=objc
#[derive(Debug, PartialEq, Eq)]
pub enum SCNetworkProtocolType {
    /// DNS protocol.
    DNS,
    /// IPv4 protocol.
    IPv4,
    /// IPv6 protocol.
    IPv6,
    /// Protocol proxies.
    Proxies,
    /// SMB protocol.
    SMB,
}

impl SCNetworkProtocolType {
    /// Tries to construct a type by matching it to string constants used to identify a network
    /// protocol type. If no constants match it, `None` is returned.
    pub fn from_cfstring(type_id: &CFString) -> Option<Self> {
        use system_configuration_sys::network_configuration::*;

        let id_is_equal_to = |const_str| -> bool {
            let const_str = unsafe { CFString::wrap_under_get_rule(const_str) };
            &const_str == type_id
        };
        unsafe {
            if id_is_equal_to(kSCNetworkProtocolTypeDNS) {
                Some(SCNetworkProtocolType::DNS)
            } else if id_is_equal_to(kSCNetworkProtocolTypeIPv4) {
                Some(SCNetworkProtocolType::IPv4)
            } else if id_is_equal_to(kSCNetworkProtocolTypeIPv6) {
                Some(SCNetworkProtocolType::IPv6)
            } else if id_is_equal_to(kSCNetworkProtocolTypeProxies) {
                Some(SCNetworkProtocolType::Proxies)
            } else if id_is_equal_to(kSCNetworkProtocolTypeSMB) {
                Some(SCNetworkProtocolType::SMB)
            } else {
                None
            }
        }
    }

    /// Returns the string constants used to identify this network protocol type.
    pub fn to_cfstring(&self) -> CFString {
        use system_configuration_sys::network_configuration::*;
        let wrap_const = |const_str| unsafe { CFString::wrap_under_get_rule(const_str) };
        unsafe {
            match self {
                SCNetworkProtocolType::DNS => wrap_const(kSCNetworkProtocolTypeDNS),
                SCNetworkProtocolType::IPv4 => wrap_const(kSCNetworkProtocolTypeIPv4),
                SCNetworkProtocolType::IPv6 => wrap_const(kSCNetworkProtocolTypeIPv6),
                SCNetworkProtocolType::Proxies => wrap_const(kSCNetworkProtocolTypeProxies),
                SCNetworkProtocolType::SMB => wrap_const(kSCNetworkProtocolTypeSMB),
            }
        }
    }
}

core_foundation::declare_TCFType!(
    /// Represents a network service.
    ///
    /// See [`SCNetworkInterfaceRef`] and its [methods] for details.
    ///
    /// [`SCNetworkInterfaceRef`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkserviceref?language=objc
    /// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
    SCNetworkService,
    SCNetworkServiceRef
);

core_foundation::impl_TCFType!(
    SCNetworkService,
    SCNetworkServiceRef,
    SCNetworkServiceGetTypeID
);
core_foundation::impl_CFTypeDescription!(SCNetworkService);

impl SCNetworkService {
    /// Returns an array of all network services
    pub fn get_services(prefs: &SCPreferences) -> CFArray<Self> {
        unsafe {
            let array_ptr = SCNetworkServiceCopyAll(prefs.to_void());
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<Self>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Returns the service with the specified identifier. Or `None` if the service ID does not
    /// exist in the preferences or if an error occurred.
    ///
    /// See [`SCNetworkServiceCopy`] for details.
    ///
    /// [`SCNetworkServiceCopy`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkservicecopy(_:_:)?language=objc
    pub fn find_service<S: Into<CFString>>(prefs: &SCPreferences, service_id: S) -> Option<Self> {
        let cf_service_id = service_id.into();
        unsafe {
            let service_ref = SCNetworkServiceCopy(
                prefs.as_concrete_TypeRef(),
                cf_service_id.as_concrete_TypeRef(),
            );
            if !service_ref.is_null() {
                Some(Self::wrap_under_create_rule(service_ref))
            } else {
                None
            }
        }
    }

    /// Creates a new network service for the specified interface in the configuration. Or `None`
    /// if an error occurred.
    ///
    /// See [`SCNetworkServiceCreate`] for details.
    ///
    /// [`SCNetworkServiceCreate`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkservicecreate(_:_:)?language=objc
    pub fn create(prefs: &SCPreferences, interface: &SCNetworkInterface) -> Option<Self> {
        unsafe {
            let service_ref = SCNetworkServiceCreate(
                prefs.as_concrete_TypeRef(),
                interface.as_concrete_TypeRef(),
            );
            if !service_ref.is_null() {
                Some(Self::wrap_under_create_rule(service_ref))
            } else {
                None
            }
        }
    }

    /// Returns a [`bool`] value indicating whether the specified service is enabled.
    pub fn enabled(&self) -> bool {
        unsafe { SCNetworkServiceGetEnabled(self.0) != 0 }
    }

    /// Returns the service identifier.
    pub fn id(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkServiceGetServiceID(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns the network interface backing this network service, if it has one.
    pub fn network_interface(&self) -> Option<SCNetworkInterface> {
        unsafe {
            let ptr = SCNetworkServiceGetInterface(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(SCNetworkInterface::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns all network protocols associated with the specified service.
    ///
    /// See [`SCNetworkServiceCopyProtocols`] for details.
    ///
    /// [`SCNetworkServiceCopyProtocols`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkservicecopyprotocols(_:)?language=objc
    pub fn network_protocols(&self) -> CFArray<SCNetworkProtocol> {
        unsafe {
            let array_ptr = SCNetworkServiceCopyProtocols(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<SCNetworkProtocol>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Returns the network protocol of the specified type for the specified service. Or `None` if
    /// this protocol has not been added or if an error occurred.
    ///
    /// See [`SCNetworkServiceCopyProtocol`] for details.
    ///
    /// [`SCNetworkServiceCopyProtocol`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkservicecopyprotocol(_:_:)?language=objc
    pub fn find_network_protocol<S: Into<CFString>>(
        &self,
        protocol_type: S,
    ) -> Option<SCNetworkProtocol> {
        let protocol_type_ref = protocol_type.into().as_concrete_TypeRef();
        unsafe {
            let ptr = SCNetworkServiceCopyProtocol(self.0, protocol_type_ref);
            if ptr.is_null() {
                None
            } else {
                Some(SCNetworkProtocol::wrap_under_create_rule(ptr))
            }
        }
    }

    /// Establishes the default configuration for the specified network service. The default
    /// configuration includes the addition of network protocols for the service (with default
    /// configuration options).
    ///
    /// Returns: `true` if the configuration was updated; `false` if an error occurred.
    pub fn establish_default_configuration(&mut self) -> bool {
        (unsafe { SCNetworkServiceEstablishDefaultConfiguration(self.0) }) != 0
    }

    /// Adds the network protocol of the specified type to the specified service. The protocol
    /// configuration is set to default values that are appropriate for the interface associated
    /// with the service.
    ///
    /// Returns: `true` if the protocol was added to the service; `false` if the protocol was
    ///          already present or an error occurred.
    pub fn add_network_protocol<S: Into<CFString>>(&mut self, protocol_type: S) -> bool {
        let protocol_type_ref = protocol_type.into().as_concrete_TypeRef();
        (unsafe { SCNetworkServiceAddProtocolType(self.0, protocol_type_ref) }) != 0
    }

    /// Removes the specified network service from the configuration.
    ///
    /// Returns: `true` if the service was removed; `false` if an error occurred.
    pub fn remove(self) -> bool {
        (unsafe { SCNetworkServiceRemove(self.0) }) != 0
    }

    /// Enables or disables the specified service.
    ///
    /// Returns: `true` if the enabled status was saved; `false` if an error occurred.
    pub fn set_enabled(&mut self, enabled: bool) -> bool {
        (unsafe { SCNetworkServiceSetEnabled(self.0, enabled as Boolean) }) != 0
    }
}

core_foundation::declare_TCFType!(
    /// Represents a complete network configuration for a particular host.
    ///
    /// See [`SCNetworkSet`] for details.
    ///
    /// [`SCNetworkSet`]: https://developer.apple.com/documentation/systemconfiguration/scnetworksetref?language=objc
    SCNetworkSet,
    SCNetworkSetRef
);
core_foundation::impl_TCFType!(SCNetworkSet, SCNetworkSetRef, SCNetworkSetGetTypeID);
core_foundation::impl_CFTypeDescription!(SCNetworkSet);

impl SCNetworkSet {
    /// Returns all available sets for the specified preferences session.
    pub fn get_sets(prefs: &SCPreferences) -> CFArray<Self> {
        unsafe {
            let array_ptr = SCNetworkSetCopyAll(prefs.to_void());
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<Self>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Returns the current set. Or `None` if no current set has been defined.
    pub fn get_current(prefs: &SCPreferences) -> Option<Self> {
        unsafe {
            let set_ref = SCNetworkSetCopyCurrent(prefs.as_concrete_TypeRef());
            if !set_ref.is_null() {
                Some(SCNetworkSet::wrap_under_create_rule(set_ref))
            } else {
                None
            }
        }
    }

    /// Returns the set with the specified identifier. Or `None` if the identifier does not exist
    /// in the preferences or if an error occurred
    ///
    /// See [`SCNetworkSetCopy`] for details.
    ///
    /// [`SCNetworkSetCopy`]: https://developer.apple.com/documentation/systemconfiguration/scnetworksetcopy(_:_:)?language=objc
    pub fn find_set<S: Into<CFString>>(prefs: &SCPreferences, set_id: S) -> Option<Self> {
        let cf_set_id = set_id.into();
        unsafe {
            let set_ref =
                SCNetworkSetCopy(prefs.as_concrete_TypeRef(), cf_set_id.as_concrete_TypeRef());
            if !set_ref.is_null() {
                Some(Self::wrap_under_create_rule(set_ref))
            } else {
                None
            }
        }
    }

    /// Constructs a new set of network services from the preferences.
    pub fn new(prefs: &SCPreferences) -> Self {
        let ptr = unsafe { SCNetworkSetCopyCurrent(prefs.to_void()) };
        unsafe { SCNetworkSet::wrap_under_create_rule(ptr) }
    }

    /// Returns all network services associated with the specified set.
    pub fn services(&self) -> CFArray<SCNetworkService> {
        unsafe {
            let array_ptr = SCNetworkSetCopyServices(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<SCNetworkService>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Returns an list of network service identifiers, ordered by their priority.
    pub fn service_order(&self) -> CFArray<CFString> {
        unsafe {
            let array_ptr = SCNetworkSetGetServiceOrder(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<CFString>::wrap_under_get_rule(array_ptr)
        }
    }

    /// Returns the identifier for the specified set.
    pub fn id(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkSetGetSetID(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns the user-specified name associated with the specified set. Or `None` if it hasn't
    /// been defined.
    pub fn name(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkSetGetName(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns a [`bool`] value indicating whether the specified interface is represented by at
    /// least one network service in the specified set.
    pub fn contains_network_interface(&self, interface: &SCNetworkInterface) -> bool {
        let iface_ref = interface.as_concrete_TypeRef();
        (unsafe { SCNetworkSetContainsInterface(self.0, iface_ref) }) != 0
    }

    /// Adds the specified network service to the specified set.
    ///
    /// Returns: `true` if the service was added to the set; `false` if the service was already
    ///          present or an error occurred.
    pub fn add_service(&mut self, service: &SCNetworkService) -> bool {
        let service_ref = service.as_concrete_TypeRef();
        (unsafe { SCNetworkSetAddService(self.0, service_ref) }) != 0
    }

    /// Removes the specified set from the configuration.
    ///
    /// Returns: `true` if the set was removed; `false` if an error occurred.
    pub fn remove(self) -> bool {
        (unsafe { SCNetworkSetRemove(self.0) }) != 0
    }

    /// Removes the specified network service from the specified set.
    ///
    /// Returns: `true` if the service was removed from the set; `false` if the service was not
    ///          already present or an error occurred.
    pub fn remove_service(&mut self, service: &SCNetworkService) -> bool {
        let service_ref = service.as_concrete_TypeRef();
        (unsafe { SCNetworkSetRemoveService(self.0, service_ref) }) != 0
    }

    /// Specifies the set that should be the current set.
    ///
    /// Returns: `true` if the current set was updated; `false` if an error occurred.
    pub fn set_current(&mut self) -> bool {
        (unsafe { SCNetworkSetSetCurrent(self.0) }) != 0
    }

    /// Stores the user-specified ordering of network services for the specified set.
    ///
    /// Returns: `true` if the new service order was saved; `false` if an error occurred.
    pub fn set_service_order(&mut self, new_order: CFArray<CFString>) -> bool {
        let cf_order_ref = new_order.as_concrete_TypeRef();
        (unsafe { SCNetworkSetSetServiceOrder(self.0, cf_order_ref) }) != 0
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
                    iface.interface_type_string()
                )
            }
        }
    }

    #[test]
    fn test_service_order() {
        let prefs = SCPreferences::default(&CFString::new("test"));
        let services = SCNetworkService::get_services(&prefs);
        let set = SCNetworkSet::new(&prefs);
        let service_order = set.service_order();

        assert!(service_order.iter().all(|service_id| {
            services
                .iter()
                .any(|service| service.id().as_ref() == Some(&*service_id))
        }))
    }

    #[test]
    fn test_empty_array() {
        let empty = create_empty_array::<CFString>();
        let values = empty.get_all_values();
        assert!(values.is_empty())
    }
}
