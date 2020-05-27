#!/usr/bin/env bash

# Always have the latest version of bindgen and rustfmt installed before using this script.
# This script require GNU sed, and expects it to be available as `gsed`. Adjust SED var if needed.

set -eu

SED=$(which gsed)

export DYLD_LIBRARY_PATH=$(rustc +stable --print sysroot)/lib

SDK_VERSION=`xcodebuild -sdk macosx -version SDKVersion`
SDK_PATH=`xcodebuild -sdk macosx -version Path`
FRAMEWORK_PATH="$SDK_PATH/System/Library/Frameworks/"

PREFERENCES_HEADER_PATH="$FRAMEWORK_PATH/SystemConfiguration.framework/Headers/SCPreferences.h"
DYNAMIC_STORE_HEADER_PATH="$FRAMEWORK_PATH/SystemConfiguration.framework/Headers/SCDynamicStore.h"
DYNAMIC_STORE_COPY_SPECIFIC_HEADER_PATH="$FRAMEWORK_PATH/SystemConfiguration.framework/Headers/SCDynamicStoreCopySpecific.h"
NETWORK_CONFIGURATION_HEADER_PATH="$FRAMEWORK_PATH/SystemConfiguration.framework/Headers/SCNetworkConfiguration.h"
NETWORK_REACHABILITY_HEADER_PATH="$FRAMEWORK_PATH/SystemConfiguration.framework/Headers/SCNetworkReachability.h"
SCHEMA_DEFINITIONS_HEADER_PATH="$FRAMEWORK_PATH/SystemConfiguration.framework/Headers/SCSchemaDefinitions.h"

PREFERENCES_BINDING_PATH="./system-configuration-sys/src/preferences.rs"
DYNAMIC_STORE_BINDING_PATH="./system-configuration-sys/src/dynamic_store.rs"
DYNAMIC_STORE_COPY_SPECIFIC_BINDING_PATH="./system-configuration-sys/src/dynamic_store_copy_specific.rs"
NETWORK_CONFIGURATION_BINDING_PATH="./system-configuration-sys/src/network_configuration.rs"
NETWORK_REACHABILITY_BINDING_PATH="./system-configuration-sys/src/network_reachability.rs"
SCHEMA_DEFINITIONS_BINDING_PATH="./system-configuration-sys/src/schema_definitions.rs"

BINDGEN_VERSION=`bindgen --version`

echo "Using macOS SDK at: $SDK_PATH"
echo "Using $BINDGEN_VERSION"
echo ""

function cleanup_binding() {
    local binding_path="$1"

    # `Option` is in the Rust standard prelude. No need to use full path, it's just verbose.
    $SED -i 's/::core::option::Option/Option/g' "$binding_path"

    # The bindings that need these types will import them directly into scope with `--raw line`
    $SED -i 's/::std::os::raw:://g' "$binding_path"

    # Most low level types should not be `Copy` nor `Clone`. And `Debug` usually don't make much
    # sense, since they are usually just pointers/binary data.
    $SED -i '/#\[derive(Debug, Copy, Clone)\]/d' "$binding_path"

    # Change struct bodies to (c_void);
    #   Search regex: {\n +_unused: \[u8; 0],\n}
    #   Replace string: (c_void);\n
    $SED -i -e '/^pub struct .* {$/ {
        N;N
        s/ {\n *_unused: \[u8; 0\],\n}/(c_void);\n/
    }' "$binding_path"

    # Remove all }\nextern "C" { to condense code a bit
    #   Search regex: }\nextern "C" {
    #   Replace string:
    $SED -i -e '/^extern "C" {$/ {
        :loop
        n
        /^}$/! b loop
        /^}$/ {
            N
            t reset_condition_flags
            :reset_condition_flags
            s/}\nextern "C" {//
            t loop
        }
    }' "$binding_path"

    rustfmt +nightly "$binding_path"
}

BINDGEN_COMMON_ARGUMENTS=(
    --no-doc-comments
    --use-core
    --no-layout-tests
    --raw-line "// Generated using:"
    --raw-line "// $BINDGEN_VERSION"
    --raw-line "// macOS SDK $SDK_VERSION."
    --raw-line ""
)

echo "Generating bindings for $PREFERENCES_HEADER_PATH"
bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --whitelist-function "SCPreferences.*" \
    --blacklist-type "(__)?CF.*" \
    --blacklist-type "Boolean" \
    --blacklist-type "dispatch_queue_[ts]" \
    --blacklist-type "(AuthorizationOpaqueRef|__SCPreferences)" \
    --raw-line "use core::ffi::c_void;" \
    --raw-line "use core_foundation_sys::array::CFArrayRef;" \
    --raw-line "use core_foundation_sys::base::{Boolean, CFIndex, CFAllocatorRef, CFTypeID};" \
    --raw-line "use core_foundation_sys::data::CFDataRef;" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "use core_foundation_sys::propertylist::CFPropertyListRef;" \
    --raw-line "use core_foundation_sys::runloop::CFRunLoopRef;" \
    --raw-line "" \
    --raw-line "use crate::dispatch_queue_t;" \
    --raw-line "" \
    --raw-line "pub type AuthorizationOpaqueRef = c_void;" \
    --raw-line "pub type __SCPreferences = c_void;" \
    -o $PREFERENCES_BINDING_PATH \
    $PREFERENCES_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $PREFERENCES_BINDING_PATH

echo ""
echo ""
echo "Generating bindings for $DYNAMIC_STORE_HEADER_PATH"

bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --whitelist-function "SCDynamicStore.*" \
    --whitelist-var "kSCDynamicStore.*" \
    --blacklist-type "(__)?CF.*" \
    --blacklist-type "Boolean" \
    --blacklist-type "dispatch_queue_[ts]" \
    --raw-line "use core::ffi::c_void;" \
    --raw-line "use core_foundation_sys::array::CFArrayRef;" \
    --raw-line "use core_foundation_sys::base::{Boolean, CFIndex, CFAllocatorRef, CFTypeID};" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "use core_foundation_sys::dictionary::CFDictionaryRef;" \
    --raw-line "use core_foundation_sys::propertylist::CFPropertyListRef;" \
    --raw-line "use core_foundation_sys::runloop::CFRunLoopSourceRef;" \
    --raw-line "" \
    --raw-line "use crate::dispatch_queue_t;" \
    -o $DYNAMIC_STORE_BINDING_PATH \
    $DYNAMIC_STORE_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $DYNAMIC_STORE_BINDING_PATH

echo ""
echo ""
echo "Generating bindings for $DYNAMIC_STORE_COPY_SPECIFIC_HEADER_PATH"

bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --whitelist-function "SCDynamicStoreCopy(ComputerName|ConsoleUser|LocalHostName|Location|Proxies)" \
    --blacklist-type "(__)?CF.*" \
    --blacklist-type "Boolean" \
    --blacklist-type "dispatch_queue_[ts]" \
    --blacklist-type "(__)?SCDynamicStore.*" \
    --raw-line "use core_foundation_sys::string::{CFStringEncoding, CFStringRef};" \
    --raw-line "use core_foundation_sys::dictionary::CFDictionaryRef;" \
    --raw-line "use crate::dynamic_store::SCDynamicStoreRef;" \
    --raw-line "" \
    --raw-line "use libc::c_uint;" \
    -o $DYNAMIC_STORE_COPY_SPECIFIC_BINDING_PATH \
    $DYNAMIC_STORE_COPY_SPECIFIC_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $DYNAMIC_STORE_COPY_SPECIFIC_BINDING_PATH

echo ""
echo ""
echo "Generating bindings for $NETWORK_CONFIGURATION_HEADER_PATH"

bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --whitelist-function "SCNetwork.*" \
    --whitelist-function "SCBondInterface.*" \
    --whitelist-var "kSC(NetworkInterface|BondStatus).*" \
    --blacklist-type "dispatch_queue_[ts]" \
    --blacklist-type "(__)?CF.*" \
    --blacklist-type "__SC.*" \
    --blacklist-type "Boolean" \
    --blacklist-type "(sockaddr|socklen_t|sa_family_t|__darwin_socklen_t|__uint.*_t)" \
    --blacklist-type "(__)?SCPreferences.*" \
    --raw-line "use core::ffi::c_void;" \
    --raw-line "use core_foundation_sys::array::CFArrayRef;" \
    --raw-line "use core_foundation_sys::base::{Boolean, CFIndex, CFAllocatorRef, CFTypeID};" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "use core_foundation_sys::dictionary::CFDictionaryRef;" \
    --raw-line "use core_foundation_sys::runloop::CFRunLoopRef;" \
    --raw-line "" \
    --raw-line "use crate::dispatch_queue_t;" \
    --raw-line "use libc::{c_char, c_int, sockaddr, socklen_t};" \
    --raw-line "use crate::preferences::SCPreferencesRef;" \
    --raw-line "" \
    --raw-line "pub type __SCNetworkConnection = c_void;" \
    --raw-line "pub type __SCNetworkInterface = c_void;" \
    --raw-line "pub type __SCBondStatus = c_void;" \
    --raw-line "pub type __SCNetworkProtocol = c_void;" \
    --raw-line "pub type __SCNetworkService = c_void;" \
    --raw-line "pub type __SCNetworkSet = c_void;" \
    --raw-line "pub type __SCNetworkReachability = c_void;" \
    -o $NETWORK_CONFIGURATION_BINDING_PATH \
    $NETWORK_CONFIGURATION_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $NETWORK_CONFIGURATION_BINDING_PATH

echo ""
echo ""
echo "Generating bindings for $NETWORK_REACHABILITY_HEADER_PATH"

bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --whitelist-var "kSCNetworkReachabilityFlag.*" \
    --whitelist-type "SCNetworkReachability.*" \
    --whitelist-function "SCNetworkReachability.*" \
    --blacklist-type "sockaddr" \
    --blacklist-type "dispatch_queue_[ts]" \
    --blacklist-type "SCNetworkReachability.*" \
    --blacklist-type "(__)?CF.*" \
    --blacklist-type "__SC.*" \
    --blacklist-type "Boolean" \
    --raw-line '#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]' \
    --raw-line "use core_foundation_sys::base::{Boolean, CFAllocatorRef, CFTypeID};" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "use core_foundation_sys::runloop::CFRunLoopRef;" \
    --raw-line "use crate::dispatch_queue_t;" \
    --raw-line "pub use crate::network_configuration::{SCNetworkReachabilityRef, SCNetworkReachabilityContext, SCNetworkReachabilityCallBack, SCNetworkReachabilityFlags};" \
    --raw-line "use libc::{c_char, c_uchar, sockaddr};" \
    -o $NETWORK_REACHABILITY_BINDING_PATH \
    $NETWORK_REACHABILITY_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $NETWORK_REACHABILITY_BINDING_PATH


echo ""
echo ""
echo "Generating bindings for $SCHEMA_DEFINITIONS_HEADER_PATH"

bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --whitelist-var "kSC.*" \
    --blacklist-type "(__)?CF.*" \
    --blacklist-type "dispatch_queue_[ts]" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "" \
    -o $SCHEMA_DEFINITIONS_BINDING_PATH \
    $SCHEMA_DEFINITIONS_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $SCHEMA_DEFINITIONS_BINDING_PATH
