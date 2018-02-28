extern crate system_configuration;

use system_configuration::dynamic_store::SCDynamicStoreBuilder;
use system_configuration::network_configuration::{global_router, SCNetworkInterface,
                                                  SCNetworkService};

// This example will output network-global-service, network-global-interface, network-global-router,
// network-service-order-list, network-services and network-interfaces to stdout.

fn main() {
    let store = SCDynamicStoreBuilder::new("session_name").build();

    let service = SCNetworkService::global(&store).unwrap();
    println!("Global Service:\n{:?}\n", service);
    println!("Global Interface:\n{:?}\n", service.interface());
    println!("Global Service Router:\n{:?}\n", global_router(&store));

    println!("\n-listnetworkserviceorder:");
    for service in SCNetworkService::list_order() {
        println!("{:?}", service);
    }

    println!("\n-listallnetworkservices:");
    for service in SCNetworkService::list() {
        println!("{:?}", service);
    }

    println!("\n-listallnetworkinterface:");
    for interface in SCNetworkInterface::list() {
        println!("{:?}", interface);
    }
}
