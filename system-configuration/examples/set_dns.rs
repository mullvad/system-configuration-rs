use objc2_core_foundation::{CFRetained, CFType};
use system_configuration::{
    core_foundation::{CFArray, CFDictionary, CFString},
    dynamic_store::{SCDynamicStore, SCDynamicStoreBuilder},
    sys::{kSCDynamicStorePropNetPrimaryService, kSCPropNetDNSServerAddresses},
};

// This example will change the DNS settings on the primary
// network interface to 8.8.8.8 and 8.8.4.4

fn main() {
    let store = SCDynamicStoreBuilder::new("my-test-dyn-store")
        .build()
        .expect("Unable to create DynamicStore");
    let primary_service_uuid = get_primary_service_uuid(&store).expect("No PrimaryService active");
    println!("PrimaryService UUID: {}", primary_service_uuid);

    let primary_service_path = CFString::from_str(&format!(
        "State:/Network/Service/{}/DNS",
        primary_service_uuid
    ));
    println!("PrimaryService path: {}", primary_service_path);

    let dns_dictionary = create_dns_dictionary(&[
        &*CFString::from_static_str("8.8.8.8"),
        &*CFString::from_static_str("8.8.4.4"),
    ]);

    let success = store.set(&primary_service_path, &dns_dictionary);
    println!("success? {}", success);
}

fn get_primary_service_uuid(store: &SCDynamicStore) -> Option<CFRetained<CFString>> {
    let dictionary = store
        .get("State:/Network/Global/IPv4")
        .and_then(|ty| CFRetained::downcast::<CFDictionary>(ty).ok())?;
    let dictionary = unsafe { dictionary.cast_unchecked::<CFString, CFType>() };
    dictionary
        .get(unsafe { kSCDynamicStorePropNetPrimaryService })
        .and_then(|ty| CFRetained::downcast::<CFString>(ty).ok())
}

fn create_dns_dictionary(
    addresses: &[&CFString],
) -> CFRetained<CFDictionary<CFString, CFArray<CFString>>> {
    let key = unsafe { kSCPropNetDNSServerAddresses };
    let value = CFArray::from_objects(addresses);
    CFDictionary::from_slices(&[key], &[&*value])
}
