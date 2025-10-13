use objc2_core_foundation::{kCFRunLoopCommonModes, CFRetained, CFRunLoop, CFType};
use system_configuration::{
    core_foundation::{CFArray, CFDictionary, CFString},
    dynamic_store::{SCDynamicStore, SCDynamicStoreBuilder, SCDynamicStoreCallBackContext},
    sys::kSCPropNetDNSServerAddresses,
};

// This example will watch the dynamic store for changes to any DNS setting. As soon as a change
// is detected, it will be printed to stdout.

fn main() {
    let callback_context = SCDynamicStoreCallBackContext {
        callout: my_callback,
        info: Context { call_count: 0 },
    };

    let store = SCDynamicStoreBuilder::new("my-watch-dns-store")
        .callback_context(callback_context)
        .build()
        .expect("Unable to create DynamicStore");

    let watch_keys = CFArray::<CFString>::from_objects(&[]);
    let watch_patterns = CFArray::from_retained_objects(&[CFString::from_str(
        "(State|Setup):/Network/Service/.*/DNS",
    )]);

    if store.set_notification_keys(&watch_keys, &watch_patterns) {
        println!("Registered for notifications");
    } else {
        panic!("Unable to register notifications");
    }

    let run_loop_source = store
        .create_run_loop_source()
        .expect("Unable to create run loop source");
    let run_loop = CFRunLoop::current().unwrap();
    run_loop.add_source(Some(&run_loop_source), unsafe { kCFRunLoopCommonModes });

    println!("Entering run loop");
    CFRunLoop::run();
}

/// This struct acts as a user provided context/payload to each notification callback.
/// Here one can store any type of data or state needed in the callback function.
#[derive(Debug)]
struct Context {
    call_count: u64,
}

#[allow(clippy::needless_pass_by_value)]
fn my_callback(store: &SCDynamicStore, changed_keys: &CFArray<CFString>, context: &mut Context) {
    context.call_count += 1;
    println!("Callback call count: {}", context.call_count);

    for key in changed_keys.iter() {
        if let Some(addresses) = get_dns(store, &key) {
            println!("{} changed DNS to {:?}", *key, addresses);
        } else {
            println!("{} removed DNS", *key);
        }
    }
}

fn get_dns(store: &SCDynamicStore, path: &CFString) -> Option<Vec<String>> {
    let dns_settings = store
        .get(path.to_string())
        .and_then(|ty| CFRetained::downcast::<CFDictionary>(ty).ok())?;
    let dns_settings = unsafe { dns_settings.cast_unchecked::<CFString, CFType>() };
    let address_array = dns_settings
        .get(unsafe { kSCPropNetDNSServerAddresses })
        .and_then(|ty| CFRetained::downcast::<CFArray>(ty).ok())?;
    let address_array = unsafe { address_array.cast_unchecked::<CFType>() };
    let mut result = Vec::with_capacity(address_array.len() as usize);
    for address in address_array {
        let address = address.downcast::<CFString>().ok()?;
        result.push(address.to_string())
    }
    Some(result)
}
