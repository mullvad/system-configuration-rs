#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate core_foundation_sys;

pub mod dynamic_store {
    include!(concat!(env!("OUT_DIR"), "/SCDynamicStore.rs"));
}
