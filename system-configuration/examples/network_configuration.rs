extern crate system_configuration;

use system_configuration::network_configuration::{SCNetworkGlobal, SCNetworkInterface,
                                                  SCNetworkService};

// This example will output network-global-service, network-global-interface, network-global-router,
// network-service-order-list, network-services and network-interfaces to stdout.

fn main() {
    println!("Global Service:\n{:?}\n", SCNetworkGlobal.service());

    println!("Global Interface:\n{:?}\n", SCNetworkGlobal.interface());

    println!("Global Service Router:\n{:?}\n", SCNetworkGlobal.router());

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
