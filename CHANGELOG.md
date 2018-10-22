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


## [0.1.0] - 2018-02-01
### Added
- Initial release. Supports most SCDynamicStore operations.
