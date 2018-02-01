extern crate system_configuration;

extern crate core_foundation;

use core_foundation::array::CFArray;
use core_foundation::base::{CFType, TCFType};
use core_foundation::dictionary::CFDictionary;
use core_foundation::propertylist::CFPropertyList;
use core_foundation::runloop::{CFRunLoop, kCFRunLoopCommonModes};
use core_foundation::string::CFString;

use system_configuration::dynamic_store::{SCDynamicStore, SCDynamicStoreBuilder,
                                          SCDynamicStoreCallBackContext};

// This example will watch the dynamic store for changes to any DNS setting. As soon as a change
// is detected, it will be printed to stdout.

fn main() {
    let callback_context = SCDynamicStoreCallBackContext {
        callout: my_callback,
        info: Context { call_count: 0 },
    };

    let store = SCDynamicStoreBuilder::new("my-watch-dns-store")
        .callback_context(callback_context)
        .build();

    let watch_keys: CFArray<CFString> = CFArray::from_CFTypes(&[]);
    let watch_patterns =
        CFArray::from_CFTypes(&[CFString::from("(State|Setup):/Network/Service/.*/DNS")]);

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

/// This struct acts as a user provided context/payload to each notification callback.
/// Here one can store any type of data or state needed in the callback function.
#[derive(Debug)]
struct Context {
    call_count: u64,
}

fn my_callback(store: SCDynamicStore, changed_keys: CFArray<CFString>, context: &mut Context) {
    context.call_count += 1;
    println!("Callback call count: {}", context.call_count);

    for key in changed_keys.iter() {
        if let Some(addresses) = get_dns(&store, key.clone()) {
            let addresses = addresses.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            println!("{} changed DNS to {:?}", *key, addresses);
        } else {
            println!("{} removed DNS", *key);
        }
    }
}

fn get_dns(store: &SCDynamicStore, path: CFString) -> Option<CFArray<CFString>> {
    let dictionary = store
        .get(path)
        .and_then(CFPropertyList::downcast_into::<CFDictionary>);
    if let Some(dictionary) = dictionary {
        dictionary
            .find2(&CFString::from_static_string("ServerAddresses"))
            .map(|ptr| unsafe { CFType::wrap_under_get_rule(ptr) })
            .and_then(CFType::downcast_into::<CFArray<CFString>>)
    } else {
        None
    }
}
