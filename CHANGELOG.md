# Changelog
All changes to the software that can be noticed from the users' perspective should have an entry in
this file.

### Format

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).

Entries should have the imperative form, just like commit messages. Start each entry with words like
add, fix, increase, force etc.. Not added, fixed, increased, forced etc.

Line wrap the file at 100 chars.                                              That is over here -> |

### Categories each change fall into

* **Added**: for new features.
* **Changed**: for changes in existing functionality.
* **Deprecated**: for soon-to-be removed features.
* **Removed**: for now removed features.
* **Fixed**: for any bug fixes.
* **Security**: in case of vulnerabilities.


## [Unreleased]


## [0.5.1] - 2023-05-15
### Added
- Add bindings for `SCNetworkSet` and `SCNetworkService`


## [0.5.0] - 2022-01-03
### Changed
- Upgrade crates to Rust 2021 edition.
- Bump minimum supported Rust version (MSRV) to 1.56.0.
- Upgrade core-foundation to 0.9 and core-foundation-sys to 0.8. This is a breaking
  change since those crates are publicly re-exported from these crates.


## [0.4.1] - 2020-06-04
### Fixed
- Bump the required libc version from 0.2.0 to 0.2.49 to fix the build


## [0.4.0] - 2020-06-04
### Added
- Add bindings for `SCNetworkInterface`.
- Add bindings for `SCNetworkReachability` and related types.

### Changed
- Bump minimum supported Rust version to 1.36
- Update `core-foundation` dependency to 0.7

### Fixed
- Move `SCNetworkReachability` and related types from `network_configuration` into their own module
  in the `system-configuration-sys` crate.


## [0.3.0] - 2019-10-16
### Added
- Re-generate bindings using macOS 10.15 headers.

### Changed
- Use `core::ffi::c_void` instead of `libc::c_void` (new minimum supported Rust version: 1.30).
- Define some ffi types as `struct TheType(c_void)` instead of `type TheType = c_void`.
- Upgrade crates to Rust 2018 edition, increasing minimum supported Rust version to 1.31


## [0.2.0] - 2018-10-23
### Added
- Publicly re-export `libc` and `core_foundation_sys` from `system_configuration_sys`.
- Publicly re-export `core_foundation` from `system_configuration`
- Add low level FFI bindings to SCPreferences and SCNetworkConfiguration.
- Add bare minimal safe high level SCPreferences type.
- Add low level FFI bindings to SCSchemaDefinitions.

### Changed
- Make `system_configuration_sys` a `#[no_std]` crate.
- Publicly re-export `system_configuration_sys` as `system_configuration::sys` instead of
  re-exporting each sys module under their corresponding safe level module.
- Raise minimum Rust version to 1.25 in order to use nested import groups.
- Upgrade `core-foundation` dependency from 0.5 to 0.6.


## [0.1.0] - 2018-02-01
### Added
- Initial release. Supports most SCDynamicStore operations.
