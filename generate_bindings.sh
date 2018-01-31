#!/usr/bin/env bash

# Always have the latest version of bindgen and rustfmt installed before using this script

export DYLD_LIBRARY_PATH=$(rustc +stable --print sysroot)/lib

SDK_VERSION=`xcodebuild -sdk macosx -version SDKVersion`
SDK_PATH=`xcodebuild -sdk macosx -version Path`
FRAMEWORK_PATH="$SDK_PATH/System/Library/Frameworks/"
HEADER_PATH="$FRAMEWORK_PATH/SystemConfiguration.framework/Headers/SCDynamicStore.h"

BINDING_PATH="./system-configuration-sys/src/dynamic_store.rs"

BINDGEN_VERSION=`bindgen --version`

echo "Using macOS SDK at: $SDK_PATH"
echo "Using $BINDGEN_VERSION"
echo ""

echo "Generating bindings for $HEADER_PATH"
bindgen \
    --whitelist-function "SCDynamicStore.*" \
    --whitelist-var "kSCDynamicStore.*" \
    --blacklist-type "(__)?CF.*" \
    --blacklist-type "Boolean" \
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
    -o $BINDING_PATH \
    $HEADER_PATH -- \
    -I$SDK_PATH/usr/include \
    -F$FRAMEWORK_PATH

rustfmt $BINDING_PATH
