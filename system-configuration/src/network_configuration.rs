// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bindings to [`SCNetworkConfiguration`].
//!
//! See the examples directory for examples how to use this module.
//!
//! [`SCNetworkConfiguration`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration


use core_foundation::array::CFArray;
use core_foundation::base::{CFType, TCFType};
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;

use dynamic_store::SCDynamicStore;
use preferences::SCPreferences;

pub use system_configuration_sys::network_configuration::*;

use std::fmt;
use std::net::IpAddr;

/// MTU
#[derive(Debug)]
pub struct SCNetworkInterfaceMtu {
    /// the current MTU setting for the interface.
    pub current: u32,
    /// the minimum MTU setting for the interface. If None, the minimum setting could not
    /// be determined.
    pub min: Option<u32>,
    /// the maximum MTU setting for the interface. If None, the maximum setting could not
    /// be determined.
    pub max: Option<u32>,
}

/// DNS
#[derive(Debug)]
pub struct SCNetworkServiceDns {
    /// State DNS setting
    pub state: DnsSetting,
    /// Setup DNS setting
    pub setup: DnsSetting,
}

/// DNS Setting
#[derive(Debug)]
pub struct DnsSetting {
    /// Domain Name
    pub domain_name: Option<String>,
    /// DNS Server Addresses
    pub server_addresses: Option<Vec<IpAddr>>,
}


fn global_query(store: &SCDynamicStore, key: &str) -> Option<String> {
    let path = CFString::from_static_string("State:/Network/Global/IPv4");
    if let Some(value) = store.get(path.clone()) {
        if let Some(dict) = value.downcast_into::<CFDictionary>() {
            if let Some(val) = dict.find2(&CFString::new(key)) {
                let value = unsafe { CFType::wrap_under_get_rule(val) };
                if let Some(value) = value.downcast::<CFString>() {
                    return Some(value.to_string());
                }
            }
        }
    }

    return None;
}


/// Returns default route on primary network service.
pub fn global_router(store: &SCDynamicStore) -> Option<IpAddr> {
    if let Some(router_str) = global_query(store, "Router") {
        if let Ok(router_ip) = router_str.parse::<IpAddr>() {
            return Some(router_ip);
        }
    }

    return None;
}


declare_TCFType!{
    /// Network service object.
    SCNetworkService, SCNetworkServiceRef
}

impl_TCFType!(
    SCNetworkService,
    SCNetworkServiceRef,
    SCNetworkServiceGetTypeID
);


impl SCNetworkService {
    /// Returns primary network service
    pub fn global(prefs: &SCPreferences, store: &SCDynamicStore) -> Option<Self> {
        match global_query(store, "PrimaryService") {
            Some(service_id) => SCNetworkService::from_id(prefs, &service_id),
            None => None,
        }
    }

    /// Returns network service for the specified preferences session and service ID.
    pub fn from_id(prefs: &SCPreferences, service_id: &str) -> Option<SCNetworkService> {
        let network_service_ref = unsafe {
            SCNetworkServiceCopy(
                prefs.as_concrete_TypeRef(),
                CFString::new(service_id).as_concrete_TypeRef(),
            )
        };

        if network_service_ref.is_null() {
            None
        } else {
            Some(unsafe { SCNetworkService::wrap_under_get_rule(network_service_ref) })
        }
    }

    /// Returns all available network services for the specified preferences.
    pub fn list(prefs: &SCPreferences) -> Vec<SCNetworkService> {
        let array: CFArray<CFType> = unsafe {
            CFArray::wrap_under_get_rule(SCNetworkServiceCopyAll(prefs.as_concrete_TypeRef()))
        };

        array
            .iter()
            .map(|item| item.downcast::<SCNetworkService>().unwrap())
            .collect::<Vec<SCNetworkService>>()


        // let array: CFArray<SCNetworkService> = unsafe {
        //     CFArray::wrap_under_get_rule(SCNetworkServiceCopyAll(prefs.as_concrete_TypeRef()))
        // };
        
        // array
        //     .iter()
        //     .map(|item| item.as_CFType().downcast::<SCNetworkService>().unwrap())
        //     .collect::<Vec<SCNetworkService>>()
    }

    /// Returns the user-specified ordering of network services within the specified
    /// preferences.
    pub fn list_order(prefs: &SCPreferences) -> Vec<SCNetworkService> {
        let netset = unsafe { SCNetworkSetCopyCurrent(prefs.as_concrete_TypeRef()) };

        let array: CFArray<CFType> =
            unsafe { CFArray::wrap_under_get_rule(SCNetworkSetGetServiceOrder(netset)) };

        let mut services = Vec::new();

        for item in array.iter() {
            if let Some(id) = item.downcast::<CFString>() {
                if let Some(serv) = SCNetworkService::from_id(prefs, id.to_string().as_str()) {
                    services.push(serv);
                }
            }
        }

        services
    }

    /// Returns the identifier for this network service.
    pub fn id(&self) -> String {
        unsafe { CFString::wrap_under_get_rule(SCNetworkServiceGetServiceID(self.0)) }.to_string()
    }

    /// Returns the user-specified name associated with this network service.
    pub fn name(&self) -> String {
        unsafe { CFString::wrap_under_get_rule(SCNetworkServiceGetName(self.0)) }.to_string()
    }

    /// Returns this network service is enabled or disabled.
    pub fn enabled(&self) -> bool {
        let ret = unsafe { SCNetworkServiceGetEnabled(self.0) };
        ret == 1
    }

    /// Returns the DNS infomation on this network service
    pub fn dns(&self, store: &SCDynamicStore) -> SCNetworkServiceDns {
        let query = |path: String| -> DnsSetting {
            let mut dns_domain_name: Option<String> = None;
            let mut dns_server_addresses: Option<Vec<IpAddr>> = None;

            if let Some(value) = store.get(CFString::new(&path)) {
                if let Some(dict) = value.downcast_into::<CFDictionary>() {
                    if let Some(domain_name) =
                        dict.find2(&CFString::from_static_string("DomainName"))
                    {
                        let domain_name = unsafe { CFType::wrap_under_get_rule(domain_name) };
                        if let Some(domain_name) = domain_name.downcast::<CFString>() {
                            dns_domain_name = Some(domain_name.to_string());
                        }
                    }

                    if let Some(addrs) =
                        dict.find2(&CFString::from_static_string("ServerAddresses"))
                    {
                        let addrs = unsafe { CFType::wrap_under_get_rule(addrs) };
                        if let Some(addrs) = addrs.downcast::<CFArray<CFType>>() {
                            let mut temp = Vec::new();
                            for addr in addrs.iter() {
                                if let Some(addr) = addr.downcast::<CFString>() {
                                    if let Ok(ip_addr) = addr.to_string().parse::<IpAddr>() {
                                        temp.push(ip_addr);
                                    }
                                }
                            }

                            if temp.len() > 0 {
                                dns_server_addresses = Some(temp);
                            }
                        }
                    }
                }
            }

            DnsSetting {
                domain_name: dns_domain_name,
                server_addresses: dns_server_addresses,
            }
        };

        let state_dns_setting = query(format!("State:/Network/Service/{}/DNS", self.id()));
        let setup_dns_setting = query(format!("Setup:/Network/Service/{}/DNS", self.id()));

        SCNetworkServiceDns {
            state: state_dns_setting,
            setup: setup_dns_setting,
        }
    }

    /// Setting DNS Domain Name on this network service
    pub fn set_dns_domain_name(&self, store: &SCDynamicStore, domain_name: Option<String>) -> bool {
        let key = CFString::from_static_string("DomainName");
        let value = CFString::new(domain_name.unwrap_or("Empty".to_string()).as_str());
        let dictionary = CFDictionary::from_CFType_pairs(&[(key, value)]);

        let path = CFString::new(&format!("Setup:/Network/Service/{}/DNS", self.id()));

        store.set(path, dictionary)
    }

    /// Setting DNS Server Addresses on this network service
    pub fn set_dns_server_addresses(
        &self,
        store: &SCDynamicStore,
        server_addrs: Option<Vec<IpAddr>>,
    ) -> bool {
        let key = CFString::from_static_string("ServerAddresses");
        let addrs: Vec<CFString> = match server_addrs {
            Some(addrs) => addrs
                .iter()
                .map(|s| CFString::new(&format!("{}", s)))
                .collect(),
            None => vec![CFString::new("Empty")],
        };
        let value = CFArray::from_CFTypes(&addrs);
        let dictionary = CFDictionary::from_CFType_pairs(&[(key, value)]);

        let path = CFString::new(&format!("Setup:/Network/Service/{}/DNS", self.id()));

        store.set(path, dictionary)
    }

    /// Returns the network interface associated with this network service.
    pub fn interface(&self) -> Option<SCNetworkInterface> {
        let interface_ref =
            unsafe { CFType::wrap_under_get_rule(SCNetworkServiceGetInterface(self.0)) };
        interface_ref.downcast::<SCNetworkInterface>()
    }
}

impl fmt::Display for SCNetworkService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for SCNetworkService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SCNetworkService{{ id: {:?}, name: {:?}, enabled: {}, interface: {:?} }}",
            self.id(),
            self.name(),
            self.enabled(),
            self.interface(),
        )
    }
}


declare_TCFType!{
    /// Network interface object.
    SCNetworkInterface, SCNetworkInterfaceRef
}

impl_TCFType!(
    SCNetworkInterface,
    SCNetworkInterfaceRef,
    SCNetworkInterfaceGetTypeID
);


impl SCNetworkInterface {
    /// Returns all network-capable interfaces on the system.
    pub fn list() -> Vec<SCNetworkInterface> {
        let array: CFArray<CFType> =
            unsafe { CFArray::wrap_under_get_rule(SCNetworkInterfaceCopyAll()) };

        array
            .iter()
            .map(|item| item.downcast::<SCNetworkInterface>().unwrap())
            .collect::<Vec<SCNetworkInterface>>()
    }

    /// Returns the current MTU setting and the range of allowable values
    pub fn mtu(&self) -> Option<SCNetworkInterfaceMtu> {
        let mut current = 0i32;
        let mut min = 0i32;
        let mut max = 0i32;

        let ret_code =
            unsafe { SCNetworkInterfaceCopyMTU(self.0, &mut current, &mut min, &mut max) };
        if ret_code == 0 {
            None
        } else {
            Some(SCNetworkInterfaceMtu {
                current: current as u32,
                min: if min < 0 { None } else { Some(min as u32) },
                max: if max < 0 { None } else { Some(max as u32) },
            })
        }
    }

    /// Returns the BSD interface or device name
    pub fn name(&self) -> Option<String> {
        unsafe {
            let str_ptr = SCNetworkInterfaceGetBSDName(self.0);
            if str_ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(str_ptr).to_string())
            }
        }
    }

    /// Returns the network interface type
    pub fn interface_type(&self) -> String {
        unsafe {
            CFString::wrap_under_get_rule(SCNetworkInterfaceGetInterfaceType(self.0)).to_string()
        }
    }

    /// Returns a displayable link layer address
    pub fn hwaddr(&self) -> Option<String> {
        unsafe {
            let str_ptr = SCNetworkInterfaceGetHardwareAddressString(self.0);
            if str_ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(str_ptr).to_string())
            }
        }
    }

    /// Returns the configuration settings associated with this network interface
    pub fn config(&self) -> Option<CFDictionary> {
        unsafe {
            let config_ptr = SCNetworkInterfaceGetConfiguration(self.0);
            if config_ptr.is_null() {
                None
            } else {
                Some(CFDictionary::wrap_under_get_rule(config_ptr))
            }
        }
    }
}

impl fmt::Display for SCNetworkInterface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for SCNetworkInterface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mtu = self.mtu();
        let mtu_fmt = if mtu.is_none() {
            format!("None")
        } else {
            let mtu = mtu.unwrap();
            format!(
                "{{ current: {}, min: {:?}, max: {:?} }}",
                mtu.current, mtu.min, mtu.max
            )
        };

        write!(
            f,
            "SCNetworkInterface{{ mtu: {}, name: {:?}, type: {:?}, hwaddr: {:?} }}",
            mtu_fmt,
            self.name(),
            self.interface_type(),
            self.hwaddr()
        )
    }
}
