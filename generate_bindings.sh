#!/usr/bin/env bash

# Always have the latest version of bindgen and rustfmt installed before using this script

set -eu

export DYLD_LIBRARY_PATH=$(rustc +stable --print sysroot)/lib

SDK_VERSION=`xcodebuild -sdk macosx -version SDKVersion`
SDK_PATH=`xcodebuild -sdk macosx -version Path`
FRAMEWORK_PATH="$SDK_PATH/System/Library/Frameworks/"

PREFERENCES_HEADER_PATH="$FRAMEWORK_PATH/SystemConfiguration.framework/Headers/SCPreferences.h"
DYNAMIC_STORE_HEADER_PATH="$FRAMEWORK_PATH/SystemConfiguration.framework/Headers/SCDynamicStore.h"
NETWORK_CONFIGURATION_HEADER_PATH="$FRAMEWORK_PATH/SystemConfiguration.framework/Headers/SCNetworkConfiguration.h"

PREFERENCES_BINDING_PATH="./system-configuration-sys/src/preferences.rs"
DYNAMIC_STORE_BINDING_PATH="./system-configuration-sys/src/dynamic_store.rs"
NETWORK_CONFIGURATION_BINDING_PATH="./system-configuration-sys/src/network_configuration.rs"

BINDGEN_VERSION=`bindgen --version`

echo "Using macOS SDK at: $SDK_PATH"
echo "Using $BINDGEN_VERSION"
echo ""

echo "Generating bindings for $PREFERENCES_HEADER_PATH"
bindgen \
    --no-doc-comments \
    --whitelist-function "SCPreferences.*" \
    --blacklist-type "(__)?CF.*" \
    --blacklist-type "Boolean" \
    --blacklist-type "dispatch_queue_[ts]" \
    --blacklist-type "(AuthorizationOpaqueRef|__SCPreferences)" \
    --raw-line "// Generated using:" \
    --raw-line "// $BINDGEN_VERSION" \
    --raw-line "// macOS SDK $SDK_VERSION." \
    --raw-line "" \
    --raw-line "use core_foundation_sys::array::CFArrayRef;" \
    --raw-line "use core_foundation_sys::base::{Boolean, CFIndex, CFAllocatorRef, CFTypeID};" \
    --raw-line "use core_foundation_sys::data::CFDataRef;" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "use core_foundation_sys::propertylist::CFPropertyListRef;" \
    --raw-line "use core_foundation_sys::runloop::CFRunLoopRef;" \
    --raw-line "" \
    --raw-line "use dispatch_queue_t;" \
    --raw-line "use libc::c_void;" \
    --raw-line "" \
    --raw-line "pub type AuthorizationOpaqueRef = c_void;" \
    --raw-line "pub type __SCPreferences = c_void;" \
    -o $PREFERENCES_BINDING_PATH \
    $PREFERENCES_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

rustfmt $PREFERENCES_BINDING_PATH

echo ""
echo ""
echo "Generating bindings for $DYNAMIC_STORE_HEADER_PATH"
sleep 2

bindgen \
    --no-doc-comments \
    --whitelist-function "SCDynamicStore.*" \
    --whitelist-var "kSCDynamicStore.*" \
    --blacklist-type "(__)?CF.*" \
    --blacklist-type "Boolean" \
    --blacklist-type "dispatch_queue_[ts]" \
    --raw-line "// Generated using:" \
    --raw-line "// $BINDGEN_VERSION" \
    --raw-line "// macOS SDK $SDK_VERSION." \
    --raw-line "" \
    --raw-line "use core_foundation_sys::array::CFArrayRef;" \
    --raw-line "use core_foundation_sys::base::{Boolean, CFIndex, CFAllocatorRef, CFTypeID};" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "use core_foundation_sys::dictionary::CFDictionaryRef;" \
    --raw-line "use core_foundation_sys::propertylist::CFPropertyListRef;" \
    --raw-line "use core_foundation_sys::runloop::CFRunLoopSourceRef;" \
    --raw-line "" \
    --raw-line "use dispatch_queue_t;" \
    -o $DYNAMIC_STORE_BINDING_PATH \
    $DYNAMIC_STORE_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

rustfmt $DYNAMIC_STORE_BINDING_PATH

echo ""
echo ""
echo "Generating bindings for $NETWORK_CONFIGURATION_HEADER_PATH"
sleep 2

bindgen \
    --no-doc-comments \
    --whitelist-function "SCNetwork.*" \
    --whitelist-function "SCBondInterface.*" \
    --whitelist-var "kSC(NetworkInterface|BondStatus).*" \
    --blacklist-type "dispatch_queue_[ts]" \
    --blacklist-type "(__)?CF.*" \
    --blacklist-type "__SC.*" \
    --blacklist-type "Boolean" \
    --blacklist-type "(sockaddr|socklen_t|sa_family_t|__darwin_socklen_t|__uint.*_t)" \
    --blacklist-type "(__)?SCPreferences.*" \
    --raw-line "// Generated using:" \
    --raw-line "// $BINDGEN_VERSION" \
    --raw-line "// macOS SDK $SDK_VERSION." \
    --raw-line "" \
    --raw-line "use core_foundation_sys::array::CFArrayRef;" \
    --raw-line "use core_foundation_sys::base::{Boolean, CFIndex, CFAllocatorRef, CFTypeID};" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "use core_foundation_sys::dictionary::CFDictionaryRef;" \
    --raw-line "use core_foundation_sys::runloop::CFRunLoopRef;" \
    --raw-line "" \
    --raw-line "use dispatch_queue_t;" \
    --raw-line "use libc::{c_void, c_char, c_int, sockaddr};" \
    --raw-line "use preferences::SCPreferencesRef;" \
    --raw-line "" \
    --raw-line "pub type __SCNetworkReachability = c_void;" \
    --raw-line "pub type __SCNetworkConnection = c_void;" \
    --raw-line "pub type __SCNetworkInterface = c_void;" \
    --raw-line "pub type __SCBondStatus = c_void;" \
    --raw-line "pub type __SCNetworkProtocol = c_void;" \
    --raw-line "pub type __SCNetworkService = c_void;" \
    --raw-line "pub type __SCNetworkSet = c_void;" \
    -o $NETWORK_CONFIGURATION_BINDING_PATH \
    $NETWORK_CONFIGURATION_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

rustfmt $NETWORK_CONFIGURATION_BINDING_PATH
