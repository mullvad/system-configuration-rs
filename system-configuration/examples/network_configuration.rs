extern crate core_foundation;
extern crate system_configuration;

use core_foundation::base::kCFAllocatorDefault;

use system_configuration::dynamic_store::SCDynamicStoreBuilder;
use system_configuration::network_configuration::{global_router, SCNetworkInterface,
                                                  SCNetworkService};
use system_configuration::preferences::SCPreferences;

// This example will output network-global-service, network-global-interface, network-global-router,
// network-service-order-list, network-services and network-interfaces to stdout.

fn main() {
    let session_name = "session_name";
    let prefs = SCPreferences::new(unsafe { kCFAllocatorDefault }, session_name, None);
    let store = SCDynamicStoreBuilder::new(session_name).build();

    let service = SCNetworkService::global(&prefs, &store).unwrap();
    println!("Global Service:\n{:?}\n", service);
    println!("Global Interface:\n{:?}\n", service.interface());
    println!("Global Service Router:\n{:?}\n", global_router(&store));

    println!("\n-listnetworkserviceorder:");
    for service in SCNetworkService::list_order(&prefs) {
        println!("{:?}", service);
    }

    println!("\n-listallnetworkservices:");
    for service in SCNetworkService::list(&prefs) {
        println!("{:?}", service);
    }

    println!("\n-listallnetworkinterface:");
    for interface in SCNetworkInterface::list() {
        println!("{:?}", interface);
    }
}
