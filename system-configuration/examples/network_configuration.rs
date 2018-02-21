extern crate system_configuration;

use system_configuration::network_configuration::{
    // SCNetworkInterfaceMTU, SCNetworkServiceDNS,
    SCNetworkGlobal, SCNetworkService, SCNetworkInterface
};


fn main (){
    println!("{:?}", SCNetworkGlobal.service());

    println!("{:?}", SCNetworkGlobal.interface());

    println!("{:?}", SCNetworkGlobal.router());

    for service in SCNetworkService::list_order() {
        println!("{:?}\n\n", service);
    }

    for service in SCNetworkService::list() {
        println!("{:?}\n\n", service);
    }

    for interface in SCNetworkInterface::list() {
        println!("{:?}\n\n", interface);
    }
}