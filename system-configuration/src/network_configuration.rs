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
use core_foundation::base::kCFAllocatorDefault;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::{CFString, CFStringRef};

use dynamic_store::{SCDynamicStoreBuilder, SCDynamicStore};
pub use system_configuration_sys::network_configuration::*;
use system_configuration_sys::preferences::SCPreferencesCreate;

use std::{fmt, ptr};
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
    state_domain_name: Option<String>,
    setup_domain_name: Option<String>,
    state_server_addresses: Option<Vec<IpAddr>>,
    setup_server_addresses: Option<Vec<IpAddr>>,
}

impl SCNetworkServiceDns {
    /// DNS Constructor
    pub fn new(
        domain_name: (Option<String>, Option<String>),
        server_addresses: (Option<Vec<IpAddr>>, Option<Vec<IpAddr>>),
    ) -> SCNetworkServiceDns {
        SCNetworkServiceDns {
            state_domain_name: domain_name.0,
            setup_domain_name: domain_name.1,
            state_server_addresses: server_addresses.0,
            setup_server_addresses: server_addresses.1,
        }
    }
    
    /// Returns DomainName (state and setup)
    pub fn domain_name(&self) -> (Option<String>, Option<String>) {
        (
            self.state_domain_name.clone(),
            self.setup_domain_name.clone(),
        )
    }

    /// Returns ServerAddresses (state and setup)
    pub fn server_addresses(&self) -> (Option<Vec<IpAddr>>, Option<Vec<IpAddr>>) {
        (
            self.state_server_addresses.clone(),
            self.setup_server_addresses.clone(),
        )
    }
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
    // let store = SCDynamicStoreBuilder::new(session_name).build();
    if let Some(router_str) = global_query(store, "Router") {
        if let Ok(router_ip) = router_str.parse::<IpAddr>() {
            return Some(router_ip);
        }
    }

    return None;
}

// pub fn netinfo(&self);
// pub fn proxies(&self) ;


declare_TCFType!{
    /// Network service object.
    SCNetworkService, SCNetworkServiceRef
}

impl_TCFType!(SCNetworkService, SCNetworkServiceRef, SCNetworkServiceGetTypeID);



impl SCNetworkService {
    /// Returns primary network service
    pub fn global(store: &SCDynamicStore) -> Option<Self> {
        // let store = SCDynamicStoreBuilder::new(session_name).build();
        if let Some(service_id) = global_query(store, "PrimaryService") {
            for service in SCNetworkService::list() {
                if service.id() == service_id {
                    return Some(service);
                }
            }
        }

        return None;
    }

    /// Returns all available network services for the specified preferences.
    pub fn list() -> Vec<SCNetworkService> {
        let prefs = unsafe {
            SCPreferencesCreate(
                kCFAllocatorDefault,
                CFString::from_static_string("ns_list").as_concrete_TypeRef(),
                ptr::null(),
            )
        };

        let array: CFArray<SCNetworkServiceRef> =
            unsafe { CFArray::wrap_under_get_rule(SCNetworkServiceCopyAll(prefs)) };

        array
            .get_all_values()
            .iter()
            .map(|service_ptr| {
                unsafe {
                    SCNetworkService::wrap_under_get_rule(*service_ptr as _)
                }
            })
            .collect::<Vec<SCNetworkService>>()
    }

    /// Returns the user-specified ordering of network services within the specified set.
    pub fn list_order() -> Vec<SCNetworkService> {
        let prefs = unsafe {
            SCPreferencesCreate(
                kCFAllocatorDefault,
                CFString::from_static_string("ns_list_order").as_concrete_TypeRef(),
                ptr::null(),
            )
        };

        let netset = unsafe { SCNetworkSetCopyCurrent(prefs) };

        let array: CFArray<SCNetworkServiceRef> =
            unsafe { CFArray::wrap_under_get_rule(SCNetworkSetGetServiceOrder(netset)) };

        let mut services = Vec::new();

        for id in array.get_all_values().iter() {
            let id_ptr: CFStringRef = *id as _;
            let service_ptr: SCNetworkServiceRef = unsafe { SCNetworkServiceCopy(prefs, id_ptr) };
            services.push(unsafe { SCNetworkService::wrap_under_get_rule(service_ptr) });
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
    pub fn dns(&self) -> SCNetworkServiceDns {
        let store = SCDynamicStoreBuilder::new("ns_dns").build();

        let query = |path: String| -> (Option<String>, Option<Vec<IpAddr>>) {
            let mut _domain_name: Option<String> = None;
            let mut _server_addresses: Option<Vec<IpAddr>> = None;

            if let Some(value) = store.get(CFString::new(&path)) {
                if let Some(dict) = value.downcast_into::<CFDictionary>() {
                    if let Some(domain_name) =
                        dict.find2(&CFString::from_static_string("DomainName"))
                    {
                        let domain_name = unsafe { CFType::wrap_under_get_rule(domain_name) };
                        if let Some(domain_name) = domain_name.downcast::<CFString>() {
                            _domain_name = Some(domain_name.to_string());
                        }
                    }

                    if let Some(addrs) =
                        dict.find2(&CFString::from_static_string("ServerAddresses"))
                    {
                        let addrs = unsafe { CFType::wrap_under_get_rule(addrs) };
                        if let Some(addrs) = addrs.downcast::<CFArray<CFString>>() {
                            let mut temp = Vec::new();
                            for addr in addrs.iter() {
                                if let Ok(ip_addr) = addr.to_string().parse::<IpAddr>() {
                                    temp.push(ip_addr);
                                }
                            }

                            if temp.len() > 0 {
                                _server_addresses = Some(temp);
                            }
                        }
                    }
                }
            }

            return (_domain_name, _server_addresses);
        };

        let (state_domain_name, state_server_addresses) =
            query(format!("State:/Network/Service/{}/DNS", self.id()));
        let (setup_domain_name, setup_server_addresses) =
            query(format!("Setup:/Network/Service/{}/DNS", self.id()));

        SCNetworkServiceDns {
            state_domain_name: state_domain_name,
            state_server_addresses: state_server_addresses,
            setup_domain_name: setup_domain_name,
            setup_server_addresses: setup_server_addresses,
        }
    }

    /// Setting DNS on this network service
    pub fn set_dns(&self, dns: SCNetworkServiceDns) -> bool {
        let store = SCDynamicStoreBuilder::new("ns_dns_set").build();

        if dns.setup_server_addresses.is_some() {
            let key = CFString::from_static_string("ServerAddresses");
            let addrs: Vec<CFString> = dns.setup_server_addresses
                .unwrap()
                .iter()
                .map(|s| CFString::new(&format!("{}", s)))
                .collect();
            let value = CFArray::from_CFTypes(&addrs);
            let dictionary = CFDictionary::from_CFType_pairs(&[(key, value)]);

            let path = CFString::new(&format!("Setup:/Network/Service/{}/DNS", self.id()));

            if !store.set(path, dictionary) {
                return false;
            }
        }

        if dns.setup_domain_name.is_some() {
            let key = CFString::from_static_string("DomainName");
            let value = CFString::new(dns.setup_domain_name.unwrap().as_str());
            let dictionary = CFDictionary::from_CFType_pairs(&[(key, value)]);

            let path = CFString::new(&format!("Setup:/Network/Service/{}/DNS", self.id()));

            if !store.set(path, dictionary) {
                // FIXME: should rollback ?
                return false;
            }
        }

        return true;
    }

    /// Returns the network interface associated with this network service.
    pub fn interface(&self) -> Option<SCNetworkInterface> {
        let interface_ptr = unsafe { SCNetworkServiceGetInterface(self.0) };
        if interface_ptr.is_null() {
            None
        } else {
            Some(unsafe { SCNetworkInterface::wrap_under_get_rule(interface_ptr) })
        }
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
            "SCNetworkService{{ id: {:?}, name: {:?}, enabled: {}, interface: {:?}, dns: {:?} }}",
            self.id(),
            self.name(),
            self.enabled(),
            self.interface(),
            self.dns()
        )
    }
}


declare_TCFType!{
    /// Network interface object.
    SCNetworkInterface, SCNetworkInterfaceRef
}

impl_TCFType!(SCNetworkInterface, SCNetworkInterfaceRef, SCNetworkInterfaceGetTypeID);


impl SCNetworkInterface {
    /// Returns primary network interface
    pub fn global(store: &SCDynamicStore) -> Option<Self> {
        if let Some(ifname) = global_query(store, "PrimaryInterface") {
            for iface in SCNetworkInterface::list() {
                if let Some(bsd_name) = iface.bsd_name() {
                    if bsd_name == ifname {
                        return Some(iface);
                    }
                }
            }
        }

        return None;
    }

    /// Returns all network-capable interfaces on the system.
    pub fn list() -> Vec<SCNetworkInterface> {
        let array: CFArray<SCNetworkInterfaceRef> =
            unsafe { CFArray::wrap_under_get_rule(SCNetworkInterfaceCopyAll()) };

        array
            .get_all_values()
            .iter()
            .map(|interface_ptr| {
                unsafe {
                    SCNetworkInterface::wrap_under_get_rule(*interface_ptr as _)
                }
            })
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
    pub fn bsd_name(&self) -> Option<String> {
        unsafe {
            let str_ptr = SCNetworkInterfaceGetBSDName(self.0);
            if str_ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(str_ptr).to_string())
            }
        }
    }

    /// Returns the BSD interface or device name
    pub fn name(&self) -> Option<String> {
        self.bsd_name()
    }

    /// Returns the network interface type
    pub fn type_(&self) -> Option<String> {
        unsafe {
            let str_ptr = SCNetworkInterfaceGetInterfaceType(self.0);
            if str_ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(str_ptr).to_string())
            }
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
            "SCNetworkInterface{{ mtu: {}, bsd_name: {:?}, type: {:?}, hwaddr: {:?} }}",
            mtu_fmt,
            self.bsd_name(),
            self.type_(),
            self.hwaddr()
        )
    }
}
