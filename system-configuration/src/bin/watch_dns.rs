extern crate system_configuration;

extern crate core_foundation;

use core_foundation::array::CFArray;
use core_foundation::dictionary::CFDictionary;
use core_foundation::runloop::{CFRunLoop, kCFRunLoopCommonModes};
use core_foundation::string::CFString;

use system_configuration::dynamic_store::{SCDynamicStore, SCDynamicStoreBuilder,
                                          SCDynamicStoreCallBackContext};

use std::env;

#[derive(Debug)]
struct Payload {
    i: u64,
    service_path: CFString,
}

impl Drop for Payload {
    fn drop(&mut self) {
        println!("Payload Drop");
    }
}

fn main() {
    let service_id = env::args()
        .skip(1)
        .next()
        .expect("Give service uuid as first argument");
    let service_path = CFString::from(&format!("State:/Network/Service/{}/DNS", service_id)[..]);
    println!("Watching {}", service_path);

    let watch_keys = CFArray::from_CFTypes(&[service_path.clone()]);
    let watch_patterns: CFArray<CFString> = CFArray::from_CFTypes(&[]);

    let callback_context = SCDynamicStoreCallBackContext {
        callout: my_callback,
        info: Payload {
            i: 0,
            service_path: service_path,
        },
    };

    let store = SCDynamicStoreBuilder::new("my-watch-dns-store")
        .callback_context(callback_context)
        .build();
    println!("Created dynamic store");

    if store.set_notification_keys(&watch_keys, &watch_patterns) {
        println!("Registered for notifications");
    } else {
        panic!("Unable to register notifications");
    }
    let run_loop_source = store.create_run_loop_source();

    let run_loop = CFRunLoop::get_current();
    run_loop.add_source(&run_loop_source, unsafe { kCFRunLoopCommonModes });
    println!("Entering run loop");
    CFRunLoop::run_current();
}

fn my_callback(store: SCDynamicStore, _changed_keys: CFArray<CFString>, payload: &mut Payload) {
    println!("my_callback2 (payload: {:?})", payload);
    payload.i += 1;

    if payload.i > 1 {
        // Only reset DNS on first callback for now. To not get stuck in infinite loop.
        return;
    }

    let server_addresses_key = CFString::from_static_string("ServerAddresses");
    let server_address_1 = CFString::from_static_string("192.168.1.1");
    let server_addresses_value = CFArray::from_CFTypes(&[server_address_1]);

    let dns_dictionary =
        CFDictionary::from_CFType_pairs(&[(server_addresses_key, server_addresses_value)]);

    let success = store.set(payload.service_path.clone(), dns_dictionary);
    println!("callback: {}", success);
}
