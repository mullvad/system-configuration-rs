// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate bindgen;

use std::env;
use std::path::Path;
use std::process::Command;
use std::str;

fn main() {
    if std::env::var("TARGET").unwrap().contains("-apple") {
        println!("cargo:rustc-link-lib=framework=SystemConfiguration");
    }

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR missing from environment");
    let sdk_path = get_macos_sdk_path();
    let framework_path = format!("{}/System/Library/Frameworks/", sdk_path);
    let sc_header_dir = format!("{}/SystemConfiguration.framework/Headers/", framework_path);

    let sc_header_path = format!("{}/SCDynamicStore.h", sc_header_dir);
    let _ = bindgen::builder()
        .header(sc_header_path)
        .clang_arg(format!("-I{}/usr/include", sdk_path))
        .clang_arg(format!("-F{}", framework_path))
        .whitelist_function("SCDynamicStore.*")
        .whitelist_var("kSCDynamicStore.*")
        .blacklist_type("(__)?CF.*")
        .blacklist_type("Boolean")
        .raw_line("use core_foundation_sys::array::CFArrayRef;")
        .raw_line("use core_foundation_sys::base::{Boolean, CFIndex, CFAllocatorRef, CFTypeID};")
        .raw_line("use core_foundation_sys::string::CFStringRef;")
        .raw_line("use core_foundation_sys::dictionary::CFDictionaryRef;")
        .raw_line("use core_foundation_sys::propertylist::CFPropertyListRef;")
        .raw_line("use core_foundation_sys::runloop::CFRunLoopSourceRef;")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(Path::new(&out_dir).join("SCDynamicStore.rs"))
        .expect("Unable to write SCDynamicStore.rs");
}

fn get_macos_sdk_path() -> String {
    let output = Command::new("xcodebuild")
        .args(&["-sdk", "macosx", "Path", "-version"])
        .output()
        .expect("Unable to get macOS SDK path with \"xcodebuild\"");
    let stdout = str::from_utf8(&output.stdout).expect("xcodebuild did not print valid utf-8");
    stdout.trim().to_owned()
}
