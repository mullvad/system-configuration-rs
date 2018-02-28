extern crate system_configuration;

use system_configuration::dynamic_store::SCDynamicStoreBuilder;
use system_configuration::network_configuration::{SCNetworkService};

use std::net::{IpAddr, Ipv4Addr};

// This example will change the DNS settings on the primary
// network interface to 8.8.8.8 and 8.8.4.4

// Usage:

// $ cargo build --example set_dns
// $ sudo ../target/debug/examples/set_dns

fn main() {
    let addrs = vec![
        IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
        IpAddr::V4(Ipv4Addr::new(8, 8, 4, 4)),
    ];

    let store = SCDynamicStoreBuilder::new("session_name").build();

    let global_service = SCNetworkService::global(&store).expect("No PrimaryService active");
    let global_interface = global_service.interface().expect("No PrimaryInterface active");

    println!("Global Service:");
    println!("\tid: {:?}", global_service.id());
    println!("\tname: {:?}", global_service.name());
    println!("\tenabled: {:?}", global_service.enabled());
    println!("\tdns: {:?}", global_service.dns(&store));
    println!("\tinterface: {:?}", global_interface.name().unwrap());

    println!(
        "Set dns to {:?} on {:?} service ...",
        addrs,
        global_service.name()
    );
    

    println!("Success: {:?}", global_service.set_dns_server_addresses(&store, Some(addrs) ));

    // Check
    // networksetup -getdnsservers "Wi-Fi"
    // scutil --dns
    // dig
    println!("{:?}", global_service.dns(&store));

    println!(
        "\n\nUse Command `networksetup -setdnsservers \"{}\" \"Empty\"` to restore DNS setting. ",
        global_service.name()
    );
}
