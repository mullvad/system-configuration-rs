extern crate system_configuration;
extern crate system_configuration_sys;

extern crate core_foundation;
extern crate core_foundation_sys;

use core_foundation::array::CFArray;
use core_foundation::base::TCFType;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::{CFString, CFStringRef};

use system_configuration::dynamic_store::SCDynamicStore;

fn main() {
    unsafe {
        let store = SCDynamicStore::create("my-test-dyn-store");
        println!("Created dynamic store");

        let ipv4_dict = store
            .get("State:/Network/Global/IPv4")
            .expect("Unable to find global settings");
        println!("Got IPv4 global property list");

        let pri_service_id_ptr = ipv4_dict
            .find2(&CFString::from_static_string("PrimaryService"))
            .expect("No PrimaryService");
        let pri_service_id = CFString::wrap_under_get_rule(pri_service_id_ptr as CFStringRef);

        let pri_service_path =
            CFString::new(&format!("State:/Network/Service/{}/DNS", pri_service_id));
        println!("PrimaryService path: {}", pri_service_path);

        let server_addresses_key = CFString::from_static_string("ServerAddresses");
        let server_addresses_value = CFArray::from_CFTypes(&[
            CFString::from_static_string("8.8.8.8"),
            CFString::from_static_string("8.8.4.4"),
        ]);
        let dns_dictionary =
            CFDictionary::from_CFType_pairs(&[(server_addresses_key, server_addresses_value)]);

        let success = store.set(pri_service_path, &dns_dictionary);
        println!("success? {}", success);
    }
}
