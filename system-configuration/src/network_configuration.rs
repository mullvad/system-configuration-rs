use std::fmt;
use std::ptr;
use std::mem;
use std::net::IpAddr;

use core_foundation_sys::base::kCFAllocatorDefault;

use core_foundation::base::{CFType, TCFType};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::array::{CFArray};
use core_foundation::dictionary::{CFDictionary};


use system_configuration_sys::*;
use dynamic_store::{SCDynamicStoreBuilder};


#[derive(Debug)]
pub struct SCNetworkInterfaceMTU {
    cur: u32,
    min: u32,
    max: u32,
}

impl SCNetworkInterfaceMTU {
    pub fn cur(&self) -> u32 {
        self.cur
    }

    pub fn min(&self) -> u32 {
        self.min
    }

    pub fn max(&self) -> u32 {
        self.max
    }
}

#[derive(Debug)]
pub struct SCNetworkServiceDNS {
    state_domain_name: Option<String>,
    setup_domain_name: Option<String>,
    state_server_addresses: Option<Vec<IpAddr>>,
    setup_server_addresses: Option<Vec<IpAddr>>,
}

impl SCNetworkServiceDNS {
    pub fn new(domain_name: (Option<String>, Option<String>),
               server_addresses: (Option<Vec<IpAddr>>, Option<Vec<IpAddr>>)) -> SCNetworkServiceDNS {
        
        SCNetworkServiceDNS {
            state_domain_name: domain_name.0,
            setup_domain_name: domain_name.1,
            state_server_addresses: server_addresses.0,
            setup_server_addresses: server_addresses.1,
        }
    }

    pub fn domain_name(&self) -> (Option<String>, Option<String>) {
        (self.state_domain_name.clone(), self.setup_domain_name.clone())
    }

    pub fn server_addresses(&self) -> (Option<Vec<IpAddr>>, Option<Vec<IpAddr>>) {
        (self.state_server_addresses.clone(), self.setup_server_addresses.clone())
    }
}


pub struct SCNetworkGlobal;

impl SCNetworkGlobal {
    pub fn service(&self) -> Option<SCNetworkService> {
        let store = SCDynamicStoreBuilder::new("ng_service").build();
        let key = CFString::from_static_string("State:/Network/Global/IPv4");

        if let Some(value) = store.get(key.clone()) {
            if let Some(dict) = value.downcast_into::<CFDictionary>() {
                if let Some(val) = dict.find2(&CFString::from_static_string("PrimaryService")) {
                    let value = unsafe { CFType::wrap_under_get_rule(val) };
                    if let Some(service_id) = value.downcast::<CFString>() {
                        let service_id = service_id.to_string();

                        for _service in SCNetworkService::list(){
                            if _service.id()  == service_id {
                                return Some(_service);
                            }
                        }
                    }
                }
            }
        }
        return None;
    }

    pub fn interface(&self) -> Option<SCNetworkInterface> {
        let store = SCDynamicStoreBuilder::new("ng_interface").build();
        let key = CFString::from_static_string("State:/Network/Global/IPv4");

        if let Some(value) = store.get(key.clone()) {
            if let Some(dict) = value.downcast_into::<CFDictionary>() {
                if let Some(val) = dict.find2(&CFString::from_static_string("PrimaryInterface")) {
                    let value = unsafe { CFType::wrap_under_get_rule(val) };
                    if let Some(ifname) = value.downcast::<CFString>() {
                        for iface in SCNetworkInterface::list(){
                            let bsd_name = iface.bsd_name();
                            if bsd_name.is_some() && bsd_name.unwrap() == ifname.to_string() {
                                return Some(iface);
                            }
                        }
                    }
                }
            }
        }

        return None;
    }

    pub fn router(&self) -> Option<IpAddr> {
        let store = SCDynamicStoreBuilder::new("ng_interface").build();
        let key = CFString::from_static_string("State:/Network/Global/IPv4");

        if let Some(value) = store.get(key) {
            if let Some(dict) = value.downcast_into::<CFDictionary>() {
                if let Some(val) = dict.find2(&CFString::from_static_string("Router")) {
                    let value = unsafe { CFType::wrap_under_get_rule(val) };
                    if let Some(router_str) = value.downcast::<CFString>() {
                        let router_str = router_str.to_string();
                        match router_str.parse::<IpAddr>() {
                            Ok(router_ip) => {
                                return Some(router_ip);
                            }
                            _ => { }
                        }
                    }
                }
            }
        }

        return None;
    }

    // pub fn netinfo(&self);
    // pub fn proxies(&self) ;
}


pub struct SCNetworkService(pub SCNetworkServiceRef);

impl SCNetworkService {
    pub fn list() -> Vec<SCNetworkService> {
        let prefs = unsafe {
            SCPreferencesCreate(kCFAllocatorDefault,
                                CFString::from_static_string("ns_list").as_concrete_TypeRef(),
                                ptr::null())
        };

        let array: CFArray<SCNetworkServiceRef> = unsafe { 
            CFArray::wrap_under_get_rule(SCNetworkServiceCopyAll(prefs)) 
        };

        array.get_all_values()
              .iter()
              .map(|service_ptr| SCNetworkService( unsafe { mem::transmute(*service_ptr) } ))
              .collect::<Vec<SCNetworkService>>()
    }
    
    pub fn list_order() -> Vec<SCNetworkService> {
        let prefs = unsafe {
            SCPreferencesCreate(kCFAllocatorDefault, 
                                CFString::from_static_string("ns_list_order").as_concrete_TypeRef(),
                                ptr::null())
        };

        let netset = unsafe { SCNetworkSetCopyCurrent(prefs) };

        let array: CFArray<SCNetworkServiceRef> = unsafe {
            CFArray::wrap_under_get_rule( SCNetworkSetGetServiceOrder(netset) )
        };

        let mut services = Vec::new();

        for id in array.get_all_values().iter() {
            let id_ptr: CFStringRef = unsafe { mem::transmute(*id) };
            let service_ptr: SCNetworkServiceRef = unsafe { SCNetworkServiceCopy(prefs, id_ptr) };
            services.push(SCNetworkService(service_ptr));
        }

        services
    }

    pub fn id(&self) -> String {
        unsafe { CFString::wrap_under_get_rule( SCNetworkServiceGetServiceID( self.0 ) ) }.to_string()
    }

    pub fn name(&self) -> String {
        unsafe { CFString::wrap_under_get_rule( SCNetworkServiceGetName( self.0 ) ) }.to_string()
    }

    pub fn enabled(&self) -> bool {
        let ret = unsafe { SCNetworkServiceGetEnabled( self.0 ) };
        ret == 1
    }

    pub fn dns(&self) -> SCNetworkServiceDNS {
        let store = SCDynamicStoreBuilder::new("ns_dns").build();

        let mut state_domain_name: Option<String> = None;
        let mut state_server_addresses: Option<Vec<IpAddr>> = None;
        let mut setup_domain_name: Option<String> = None;
        let mut setup_server_addresses: Option<Vec<IpAddr>> = None;

        if let Some(value) = store.get(CFString::new(&format!("State:/Network/Service/{}/DNS", self.id()))) {
            if let Some(dict) = value.downcast_into::<CFDictionary>() {
                if let Some(domain_name) = dict.find2(&CFString::from_static_string("DomainName")) {
                    let domain_name = unsafe { CFType::wrap_under_get_rule(domain_name) };
                    if let Some(domain_name) = domain_name.downcast::<CFString>() {
                        state_domain_name = Some(domain_name.to_string());
                    }
                }

                if let Some(addrs) = dict.find2(&CFString::from_static_string("ServerAddresses")) {
                    let addrs = unsafe { CFType::wrap_under_get_rule(addrs) };
                    if let Some(addrs) = addrs.downcast::<CFArray<CFString>>() {
                        let mut temp = Vec::new();
                        for addr in addrs.iter() {
                            if let Ok(ip_addr) = addr.to_string().parse::<IpAddr>() {
                                temp.push(ip_addr);
                            }
                        }

                        if temp.len() > 0 {
                            state_server_addresses = Some(temp);
                        }
                    }
                }
            }
        }

        if let Some(value) = store.get(CFString::new(&format!("Setup:/Network/Service/{}/DNS", self.id()))) {
            if let Some(dict) = value.downcast_into::<CFDictionary>() {
                if let Some(domain_name) = dict.find2(&CFString::from_static_string("DomainName")) {
                    let domain_name = unsafe { CFType::wrap_under_get_rule(domain_name) };
                    if let Some(domain_name) = domain_name.downcast::<CFString>() {
                        setup_domain_name = Some(domain_name.to_string());
                    }
                }

                if let Some(addrs) = dict.find2(&CFString::from_static_string("ServerAddresses")) {
                    let addrs = unsafe { CFType::wrap_under_get_rule(addrs) };
                    if let Some(addrs) = addrs.downcast::<CFArray<CFString>>() {
                        let mut temp = Vec::new();
                        for addr in addrs.iter() {
                            if let Ok(ip_addr) = addr.to_string().parse::<IpAddr>() {
                                temp.push(ip_addr);
                            }
                        }

                        if temp.len() > 0 {
                            setup_server_addresses = Some(temp);
                        }
                    }
                }
            }
        }

        SCNetworkServiceDNS {
            state_domain_name: state_domain_name,
            state_server_addresses: state_server_addresses,
            setup_domain_name: setup_domain_name,
            setup_server_addresses: setup_server_addresses,
        }
    }

    pub fn set_dns(&self, dns: SCNetworkServiceDNS) -> bool {
        let store = SCDynamicStoreBuilder::new("ns_dns_set").build();

        if dns.setup_server_addresses.is_some() {
            let key = CFString::from_static_string("ServerAddresses");
            let addrs: Vec<CFString> = dns.setup_server_addresses.unwrap()
                                                    .iter()
                                                    .map(|s| CFString::new(&format!("{}", s)) )
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

    pub fn interface(&self) -> Option<SCNetworkInterface> {
        let interface_ptr = unsafe { SCNetworkServiceGetInterface( self.0 ) };
        if interface_ptr.is_null() {
            None
        } else {
            Some(SCNetworkInterface( interface_ptr ))
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
        write!(f, "SCNetworkService{{ id: {:?}, name: {:?}, enabled: {}, interface: {:?}, dns: {:?} }}",
                self.id(),
                self.name(),
                self.enabled(),
                self.interface(),
                self.dns())
    }
}


pub struct SCNetworkInterface(pub SCNetworkInterfaceRef);

impl SCNetworkInterface {
    pub fn list() -> Vec<SCNetworkInterface> {
        let array: CFArray<SCNetworkInterfaceRef> = unsafe {
            CFArray::wrap_under_get_rule(SCNetworkInterfaceCopyAll())
        };

        array.get_all_values()
              .iter()
              .map(|interface_ptr| SCNetworkInterface(unsafe { mem::transmute(*interface_ptr) }) )
              .collect::<Vec<SCNetworkInterface>>()
    }

    pub fn mtu(&self) -> Option<SCNetworkInterfaceMTU> {
        let mut current = 0i32;
        let mut min = 0i32;
        let mut max = 0i32;

        let ret_code = unsafe { SCNetworkInterfaceCopyMTU(self.0, &mut current, &mut min, &mut max) };
        if ret_code == 0 {
            None
        } else {
            Some(SCNetworkInterfaceMTU {
                cur: current as u32,
                min: min as u32,
                max: max as u32,
            })
        }
    }

    pub fn bsd_name(&self) -> Option<String> {
        unsafe {
            let str_ptr = SCNetworkInterfaceGetBSDName(self.0);
            if str_ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule( str_ptr ).to_string())
            }
        }
    }

    pub fn name(&self) -> Option<String> {
        self.bsd_name()
    }

    pub fn type_(&self) -> Option<String> {
        unsafe { 
            let str_ptr = SCNetworkInterfaceGetInterfaceType(self.0);
            if str_ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule( str_ptr ).to_string())
            }
        }
    }

    pub fn hwaddr(&self) -> Option<String> {
        unsafe { 
            let str_ptr = SCNetworkInterfaceGetHardwareAddressString(self.0);
            if str_ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule( str_ptr ).to_string())
            }
        }
    }

    pub fn config(&self) -> Option<CFDictionary> {
        unsafe {
            let config_ptr = SCNetworkInterfaceGetConfiguration(self.0);
            if config_ptr.is_null() {
                None
            } else {
                Some(CFDictionary::wrap_under_get_rule( config_ptr ))
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
            format!("{{ cur: {}, min: {}, max: {} }}", mtu.cur(), mtu.min(), mtu.max())
        };

        write!(f, "SCNetworkInterface{{ mtu: {}, bsd_name: {:?}, type: {:?}, hwaddr: {:?} }}", 
                    mtu_fmt,
                    self.bsd_name(),
                    self.type_(),
                    self.hwaddr())
    }
}