#!/usr/bin/env bash

# TODO: right now this script does NOT work, it keeps giving errors to the effect of `expected: ","` so it is some
#       weird preprocessor macro error, not sure why how who what ;(((((
#       as of now I handroll the private bindings, for the bits that I need

# Always have the latest version of bindgen and rustfmt installed before using this script.
# This script require GNU sed, and expects it to be available as `gsed`. Adjust SED var if needed.

set -eu

SED=$(which gsed)

export DYLD_LIBRARY_PATH=$(rustc +stable --print sysroot)/lib

SCRIPT_ROOT_PATH="$(pwd)/"

# ---------------- MacOS SDK ----------------
SDK_VERSION=`xcodebuild -sdk macosx -version SDKVersion`
SDK_PATH=`xcodebuild -sdk macosx -version Path`
FRAMEWORK_PATH="${SDK_PATH}/System/Library/Frameworks/"
SYSTEM_CONFIGURATION_PATH="${FRAMEWORK_PATH}/SystemConfiguration.framework/"
SC_PRIVATE_HEADER_PATH="${SYSTEM_CONFIGURATION_PATH}/PrivateHeaders/"

# ---------------- MacOS vendored sourcecode ----------------
git submodule update --init --recursive
MACOS_VENDORED_PATH="${SCRIPT_ROOT_PATH}/system-configuration-sys/apple-open-source/"
function select_macos_vendored_version() {
    # makes sure to select the right version of vendored code
    local version="$1"
    cd "$MACOS_VENDORED_PATH"
    git checkout "$version"
    cd "$SCRIPT_ROOT_PATH"
}
function configure_macos_private_staging_headers() {
    local system_configuration_src="$1"
    mkdir -p "$SC_PRIVATE_HEADER_PATH"

    # copy over the appropriate headers (only the strictly needed ones)
    cp "$system_configuration_src/SCValidation.h"                      "$SC_PRIVATE_HEADER_PATH"
    cp "$system_configuration_src/SCNetworkConfigurationPrivate.h"     "$SC_PRIVATE_HEADER_PATH"
}

# ---------------- SystemConfiguration framework headers ----------------
select_macos_vendored_version "$SDK_VERSION"
configure_macos_private_staging_headers "${MACOS_VENDORED_PATH}/configd/SystemConfiguration.fproj/"
echo "got past here.."
SC_HEADER_PATH="$SC_PRIVATE_HEADER_PATH"

#DYNAMIC_STORE_PRIVATE_HEADER_PATH="${SC_HEADER_PATH}/SCDynamicStorePrivate.h"
#DYNAMIC_STORE_COPY_SPECIFIC_PRIVATE_HEADER_PATH="${SC_HEADER_PATH}/SCDynamicStoreCopySpecificPrivate.h"
#DYNAMIC_STORE_SET_SPECIFIC_PRIVATE_HEADER_PATH="${SC_HEADER_PATH}/SCDynamicStoreSetSpecificPrivate.h"
NETWORK_CONFIGURATION_PRIVATE_HEADER_PATH="${SC_HEADER_PATH}/SCNetworkConfigurationPrivate.h"
#NETWORK_CONNECTION_PRIVATE_HEADER_PATH="${SC_HEADER_PATH}/SCNetworkConnectionPrivate.h"
#PREFERENCES_PRIVATE_HEADER_PATH="${SC_HEADER_PATH}/SCPreferencesPrivate.h"
#PREFERENCES_GET_SPECIFIC_PRIVATE_HEADER_PATH="${SC_HEADER_PATH}/SCPreferencesGetSpecificPrivate.h"
#PREFERENCES_SET_SPECIFIC_PRIVATE_HEADER_PATH="${SC_HEADER_PATH}/SCPreferencesSetSpecificPrivate.h"
#SCHEMA_DEFINITIONS_PRIVATE_HEADER_PATH="${SC_HEADER_PATH}/SCSchemaDefinitionsPrivate.h"

# ---------------- SystemConfiguration framework bindings ----------------
SC_BINDING_PATH="${SCRIPT_ROOT_PATH}/system-configuration-sys/src/private/"

#DYNAMIC_STORE_PRIVATE_BINDING_PATH="${SC_BINDING_PATH}/dynamic_store_private.rs"
#DYNAMIC_STORE_COPY_SPECIFIC_PRIVATE_BINDING_PATH="${SC_BINDING_PATH}/dynamic_store_copy_specific_private.rs"
#DYNAMIC_STORE_SET_SPECIFIC_PRIVATE_BINDING_PATH="${SC_BINDING_PATH}/dynamic_store_set_specific_private.rs"
NETWORK_CONFIGURATION_PRIVATE_BINDING_PATH="${SC_BINDING_PATH}/network_configuration_private.rs"
#NETWORK_CONNECTION_PRIVATE_BINDING_PATH="${SC_BINDING_PATH}/network_connection_private.rs"
#PREFERENCES_PRIVATE_BINDING_PATH="${SC_BINDING_PATH}/preferences_private.rs"
#PREFERENCES_GET_SPECIFIC_PRIVATE_BINDING_PATH="${SC_BINDING_PATH}/preferences_get_specific_private.rs"
#PREFERENCES_SET_SPECIFIC_PRIVATE_BINDING_PATH="${SC_BINDING_PATH}/preferences_set_specific_private.rs"
#SCHEMA_DEFINITIONS_PRIVATE_BINDING_PATH="${SC_BINDING_PATH}/schema_definitions_private.rs"

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

# ---------------- Bindgen: SCNetworkConfigurationPrivate.h => network_configuration_private.rs ----------------
#clang -E -H $NETWORK_CONFIGURATION_PRIVATE_HEADER_PATH \
#    -I $SDK_PATH/usr/include \
#    -F $FRAMEWORK_PATH >/dev/null

echo "Generating bindings for $NETWORK_CONFIGURATION_PRIVATE_HEADER_PATH"
bindgen \
    "${BINDGEN_COMMON_ARGUMENTS[@]}" \
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
    -o $NETWORK_CONFIGURATION_PRIVATE_BINDING_PATH \
    $NETWORK_CONFIGURATION_PRIVATE_HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

cleanup_binding $NETWORK_CONFIGURATION_PRIVATE_BINDING_PATH

