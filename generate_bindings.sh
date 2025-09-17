#!/usr/bin/env bash

# Always have the latest version of bindgen and rustfmt installed before using this script.
# This script require GNU sed, and expects it to be available as `gsed`. Adjust SED var if needed.

set -eu

SED=$(which gsed)

export DYLD_LIBRARY_PATH=$(rustc +stable --print sysroot)/lib

# ---------------- MacOS SDK ----------------
SDK_VERSION=`xcodebuild -sdk macosx -version SDKVersion`
SDK_PATH=`xcodebuild -sdk macosx -version Path`
FRAMEWORK_PATH="${SDK_PATH}/System/Library/Frameworks/"

# ---------------- SystemConfiguration framework headers ----------------
SC_HEADER_PATH="${FRAMEWORK_PATH}/SystemConfiguration.framework/Headers/"

#CAPTIVE_NETWORK_HEADER_PATH="${SC_HEADER_PATH}/CaptiveNetwork.h"
#DHCP_CLIENT_PREFERENCES_HEADER_PATH="${SC_HEADER_PATH}/DHCPClientPreferences.h"
DYNAMIC_STORE_HEADER_PATH="${SC_HEADER_PATH}/SCDynamicStore.h"
#DYNAMIC_STORE_COPY_DHCP_INFO_HEADER_PATH="${SC_HEADER_PATH}/SCDynamicStoreCopyDHCPInfo.h"
DYNAMIC_STORE_COPY_SPECIFIC_HEADER_PATH="${SC_HEADER_PATH}/SCDynamicStoreCopySpecific.h"
#DYNAMIC_STORE_KEY_HEADER_PATH="${SC_HEADER_PATH}/SCDynamicStoreKey.h"
#NETWORK_HEADER_PATH="${SC_HEADER_PATH}/SCNetwork.h"
NETWORK_CONFIGURATION_HEADER_PATH="${SC_HEADER_PATH}/SCNetworkConfiguration.h"
#NETWORK_CONNECTION_HEADER_PATH="${SC_HEADER_PATH}/SCNetworkConnection.h"
NETWORK_REACHABILITY_HEADER_PATH="${SC_HEADER_PATH}/SCNetworkReachability.h"
PREFERENCES_HEADER_PATH="${SC_HEADER_PATH}/SCPreferences.h"
PREFERENCES_PATH_HEADER_PATH="${SC_HEADER_PATH}/SCPreferencesPath.h"
#PREFERENCES_SET_SPECIFIC_HEADER_PATH="${SC_HEADER_PATH}/SCPreferencesSetSpecific.h"
SCHEMA_DEFINITIONS_HEADER_PATH="${SC_HEADER_PATH}/SCSchemaDefinitions.h"
SYSTEM_CONFIGURATION_HEADER_PATH="${SC_HEADER_PATH}/SystemConfiguration.h"

# ---------------- SystemConfiguration framework bindings ----------------
SC_BINDING_PATH="./system-configuration-sys/src/"

#CAPTIVE_NETWORK_BINDING_PATH="${SC_BINDING_PATH}/captive_network.rs"
#DHCP_CLIENT_PREFERENCES_BINDING_PATH="${SC_BINDING_PATH}/dhcp_client_preferences.rs"
DYNAMIC_STORE_BINDING_PATH="${SC_BINDING_PATH}/dynamic_store.rs"
#DYNAMIC_STORE_COPY_DHCP_INFO_BINDING_PATH="${SC_BINDING_PATH}/dynamic_store_copy_dhcp_info.rs"
DYNAMIC_STORE_COPY_SPECIFIC_BINDING_PATH="${SC_BINDING_PATH}/dynamic_store_copy_specific.rs"
#DYNAMIC_STORE_KEY_BINDING_PATH="${SC_BINDING_PATH}/dynamic_store_key.rs"
#NETWORK_BINDING_PATH="${SC_BINDING_PATH}/network.rs"
NETWORK_CONFIGURATION_BINDING_PATH="${SC_BINDING_PATH}/network_configuration.rs"
#NETWORK_CONNECTION_BINDING_PATH="${SC_BINDING_PATH}/network_connection.rs"
NETWORK_REACHABILITY_BINDING_PATH="${SC_BINDING_PATH}/network_reachability.rs"
PREFERENCES_BINDING_PATH="${SC_BINDING_PATH}/preferences.rs"
PREFERENCES_PATH_BINDING_PATH="${SC_BINDING_PATH}/preferences_path.rs"
#PREFERENCES_SET_SPECIFIC_BINDING_PATH="${SC_BINDING_PATH}/preferences_set_specific.rs"
SCHEMA_DEFINITIONS_BINDING_PATH="${SC_BINDING_PATH}/schema_definitions.rs"
SYSTEM_CONFIGURATION_BINDING_PATH="${SC_BINDING_PATH}/system_configuration.rs"

# ---------------- Bindgen-related definitions ----------------
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

# ---------------- Bindgen: SCDynamicStore.h => dynamic_store.rs ----------------
echo "Generating bindings for $DYNAMIC_STORE_HEADER_PATH"
bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --allowlist-function "SCDynamicStore.*" \
    --allowlist-var "kSCDynamicStore.*" \
    --blocklist-type "(__)?CF.*" \
    --blocklist-type "Boolean" \
    --blocklist-type "dispatch_queue_[ts]" \
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

# ---------------- Bindgen: SCDynamicStoreCopySpecific.h => dynamic_store_copy_specific.rs ----------------
echo "Generating bindings for $DYNAMIC_STORE_COPY_SPECIFIC_HEADER_PATH"
bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --allowlist-function "SCDynamicStoreCopy(ComputerName|ConsoleUser|LocalHostName|Location|Proxies)" \
    --blocklist-type "(__)?CF.*" \
    --blocklist-type "Boolean" \
    --blocklist-type "dispatch_queue_[ts]" \
    --blocklist-type "(__)?SCDynamicStore.*" \
    --raw-line "use core_foundation_sys::string::{CFStringEncoding, CFStringRef};" \
    --raw-line "use core_foundation_sys::dictionary::CFDictionaryRef;" \
    --raw-line "use crate::dynamic_store::SCDynamicStoreRef;" \
    -o $DYNAMIC_STORE_COPY_SPECIFIC_BINDING_PATH \
    $DYNAMIC_STORE_COPY_SPECIFIC_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $DYNAMIC_STORE_COPY_SPECIFIC_BINDING_PATH

echo ""
echo ""

# ---------------- Bindgen: SCNetworkConfiguration.h => network_configuration.rs ----------------
echo "Generating bindings for $NETWORK_CONFIGURATION_HEADER_PATH"
bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --allowlist-function "SCNetwork.*" \
    --allowlist-function "SCBondInterface.*" \
    --allowlist-var "kSC(NetworkInterface|BondStatus).*" \
    --blocklist-type "SCNetworkReachability.*" \
    --blocklist-function "SCNetworkReachability.*" \
    --blocklist-type "dispatch_queue_[ts]" \
    --blocklist-type "(__)?CF.*" \
    --blocklist-type "__SC.*" \
    --blocklist-type "Boolean" \
    --blocklist-type "(sockaddr|socklen_t|sa_family_t|__darwin_socklen_t|__uint.*_t)" \
    --blocklist-type "(__)?SCPreferences.*" \
    --raw-line "use core::ffi::c_void;" \
    --raw-line "use core_foundation_sys::array::CFArrayRef;" \
    --raw-line "use core_foundation_sys::base::{Boolean, CFIndex, CFAllocatorRef, CFTypeID};" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "use core_foundation_sys::dictionary::CFDictionaryRef;" \
    --raw-line "use core_foundation_sys::runloop::CFRunLoopRef;" \
    --raw-line "" \
    --raw-line "use crate::dispatch_queue_t;" \
    --raw-line "use libc::{sockaddr, socklen_t};" \
    --raw-line "use crate::preferences::SCPreferencesRef;" \
    --raw-line "" \
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

cleanup_binding $NETWORK_CONFIGURATION_BINDING_PATH

echo ""
echo ""

# ---------------- Bindgen: SCNetworkReachability.h => network_reachability.rs ----------------
echo "Generating bindings for $NETWORK_REACHABILITY_HEADER_PATH"
bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --allowlist-function "SCNetworkReachability.*" \
    --allowlist-type "SCNetworkReachability.*" \
    --allowlist-var "kSCNetworkReachability.*" \
    --blocklist-type "sockaddr" \
    --blocklist-type "dispatch_queue_[ts]" \
    --blocklist-type "(__)?CF.*" \
    --blocklist-type "__SC.*" \
    --blocklist-type "Boolean" \
    --blocklist-type "dispatch_.*" \
    --blocklist-type "(sockaddr|socklen_t|sa_family_t|__darwin_socklen_t|__uint.*_t)" \
    --raw-line '#![allow(clippy::unreadable_literal)]' \
    --raw-line "use core_foundation_sys::base::{Boolean, CFAllocatorRef, CFTypeID, CFIndex};" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "use core_foundation_sys::runloop::CFRunLoopRef;" \
    --raw-line "use libc::sockaddr;" \
    --raw-line "use crate::dispatch_queue_t;" \
    --raw-line "pub type __SCNetworkReachability = ::core::ffi::c_void;" \
    -o $NETWORK_REACHABILITY_BINDING_PATH \
    $NETWORK_REACHABILITY_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $NETWORK_REACHABILITY_BINDING_PATH

echo ""
echo ""

# ---------------- Bindgen: SCPreferences.h => preferences.rs ----------------
echo "Generating bindings for $PREFERENCES_HEADER_PATH"
bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --allowlist-function "SCPreferences.*" \
    --blocklist-type "(__)?CF.*" \
    --blocklist-type "Boolean" \
    --blocklist-type "dispatch_queue_[ts]" \
    --blocklist-type "(AuthorizationOpaqueRef|__SCPreferences)" \
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

# ---------------- Bindgen: SCPreferencesPath.h => preferences_path.rs ----------------
echo "Generating bindings for $PREFERENCES_PATH_HEADER_PATH"
bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --allowlist-function "SCPreferencesPath.*" \
    --blocklist-type "(__)?CF.*" \
    --blocklist-type "Boolean" \
    --blocklist-type "(__SCPreferences|SCPreferencesRef)" \
    --raw-line "use core_foundation_sys::dictionary::CFDictionaryRef;" \
    --raw-line "use core_foundation_sys::base::Boolean;" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "" \
    --raw-line "use crate::preferences::SCPreferencesRef;" \
    -o $PREFERENCES_PATH_BINDING_PATH \
    $PREFERENCES_PATH_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $PREFERENCES_PATH_BINDING_PATH

echo ""
echo ""

# ---------------- Bindgen: SCSchemaDefinitions.h => schema_definitions.rs ----------------
echo "Generating bindings for $SCHEMA_DEFINITIONS_HEADER_PATH"
bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --allowlist-var "kSC.*" \
    --blocklist-type "(__)?CF.*" \
    --blocklist-type "dispatch_queue_[ts]" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "" \
    -o $SCHEMA_DEFINITIONS_BINDING_PATH \
    $SCHEMA_DEFINITIONS_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $SCHEMA_DEFINITIONS_BINDING_PATH

echo ""
echo ""

# ---------------- Bindgen: SystemConfiguration.h => system_configuration.rs ----------------
echo "Generating bindings for $SYSTEM_CONFIGURATION_HEADER_PATH"
bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
    --allowlist-var "k(CFErrorDomainSystemConfiguration|SCStatus.*)" \
    --allowlist-function "SC(CopyLastError|Error|ErrorString)" \
    --blocklist-type "(__)?CF.*" \
    --raw-line "use core_foundation_sys::error::CFErrorRef;" \
    --raw-line "use core_foundation_sys::string::CFStringRef;" \
    --raw-line "" \
    -o $SYSTEM_CONFIGURATION_BINDING_PATH \
    $SYSTEM_CONFIGURATION_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $SYSTEM_CONFIGURATION_BINDING_PATH

