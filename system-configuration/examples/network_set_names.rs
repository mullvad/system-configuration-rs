use core_foundation::base::{CFType, TCFType};
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;
use system_configuration::preferences::SCPreferences;
use system_configuration_sys::preferences_path::SCPreferencesPathGetValue;
use system_configuration_sys::schema_definitions::{kSCPrefSets, kSCPropUserDefinedName};

// This example will read the persistent store and print (to stdout) all the names of any network sets.
// This is done with the `preferences_path` API specifically, it is what is being tested for.

fn main() {
    // constants
    let sets_key = unsafe { CFString::wrap_under_get_rule(kSCPrefSets) };
    let user_defined_name_key = unsafe { CFString::wrap_under_get_rule(kSCPropUserDefinedName) };

    // grab IDs
    let prefs = SCPreferences::default(&"my-network-set-test".into());

    // create path that points to stores dictionary
    let sets_path: CFString = (&*format!("/{sets_key}")).into();

    // Grab the dictionary corresponding to that path, and cast all keys to CFString
    let sets_dict = get_path_dictionary(&prefs, &sets_path).unwrap();
    let (keys, _) = sets_dict.get_keys_and_values();
    let keys = keys
        .into_iter()
        .map(|k| unsafe {
            CFType::wrap_under_get_rule(k)
                .downcast_into::<CFString>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    // For every key grab the associated set dictionary, then look up name and print if exits
    println!("Network sets:");
    for k in keys {
        let set_path: CFString = (&*format!("{sets_path}/{k}")).into();
        let set_dict = get_path_dictionary(&prefs, &set_path).unwrap();
        let Some(user_defined_name) = set_dict.find(&user_defined_name_key) else {
            continue;
        };
        let user_defined_name = (&*user_defined_name).downcast::<CFString>().unwrap();
        println!("  -> {} aka {}", k, user_defined_name);
    }
}

/// Returns the dictionary associated with the specified path, or nothing if the path does not exist.
fn get_path_dictionary(
    prefs: &SCPreferences,
    path: &CFString,
) -> Option<CFDictionary<CFString, CFType>> {
    unsafe {
        let dictionary_ref =
            SCPreferencesPathGetValue(prefs.as_concrete_TypeRef(), path.as_concrete_TypeRef());
        if !dictionary_ref.is_null() {
            Some(CFDictionary::wrap_under_get_rule(dictionary_ref))
        } else {
            None
        }
    }
}
