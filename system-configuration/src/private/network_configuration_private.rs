use std::os;
use core_foundation::{
    array::CFArray,
    base::{Boolean, CFRetain, CFType, CFTypeID, CFTypeRef, TCFType, TCFTypeRef, ToVoid},
    dictionary::CFDictionary,
    string::CFString,
};
use sys::{
    network_configuration::SCNetworkInterfaceGetTypeID,
    private::network_configuration_private::{
        SCBridgeInterfaceCopyAll, SCBridgeInterfaceCopyAvailableMemberInterfaces, SCBridgeInterfaceCreate,
        SCBridgeInterfaceGetAllowConfiguredMembers, SCBridgeInterfaceGetMemberInterfaces, SCBridgeInterfaceGetOptions,
        SCBridgeInterfaceRef, SCBridgeInterfaceRemove, SCBridgeInterfaceSetAllowConfiguredMembers,
        SCBridgeInterfaceSetMemberInterfaces, SCBridgeInterfaceSetOptions
    },
};

use crate::helpers::create_empty_array;
use crate::network_configuration::{SCNetworkInterface, SCNetworkInterfaceSubClass, SCNetworkInterfaceType};
use crate::preferences::SCPreferences;

core_foundation::declare_TCFType! {
    /// Represents a bridge interface, which is a subclass of
    /// [`SCNetworkInterface`](crate::network_configuration::SCNetworkInterface).
    SCBridgeInterface, SCBridgeInterfaceRef
}
core_foundation::impl_CFTypeDescription!(SCBridgeInterface);

// default implementation copied verbatim from `core_foundation::impl_TCFType!(...)` expansion.
//
// only difference is the lack of `ConcreteCFType` implementation, to prevent `CFType::downcast`
// from being implemented, as that would be unsound behavior.
//
// also implements `SCNetworkInterfaceSubClass` to allow up/downcasting to/from `SCNetworkInterface`
const _: () = {
    impl TCFType for SCBridgeInterface {
        type Ref = SCBridgeInterfaceRef;

        #[inline]
        fn as_concrete_TypeRef(&self) -> SCBridgeInterfaceRef {
            self.0
        }

        #[inline]
        unsafe fn wrap_under_create_rule(reference: SCBridgeInterfaceRef) -> Self {
            assert!(!reference.is_null(), "Attempted to create a NULL object.");
            SCBridgeInterface(reference)
        }

        #[inline]
        fn type_id() -> CFTypeID {
            unsafe {
                SCNetworkInterfaceGetTypeID()
            }
        }

        #[inline]
        fn as_CFTypeRef(&self) -> CFTypeRef {
            self.as_concrete_TypeRef() as CFTypeRef
        }

        #[inline]
        unsafe fn wrap_under_get_rule(reference: SCBridgeInterfaceRef) -> Self {
            assert!(!reference.is_null(), "Attempted to create a NULL object.");
            let reference = CFRetain(reference) as SCBridgeInterfaceRef;
            TCFType::wrap_under_create_rule(reference)
        }
    }
    impl Clone for SCBridgeInterface {
        #[inline]
        fn clone(&self) -> SCBridgeInterface {
            unsafe {
                SCBridgeInterface::wrap_under_get_rule(self.0)
            }
        }
    }
    impl PartialEq for SCBridgeInterface {
        #[inline]
        fn eq(&self, other: &SCBridgeInterface) -> bool {
            self.as_CFType().eq(&other.as_CFType())
        }
    }
    impl Eq for SCBridgeInterface {}
    unsafe impl<'a> ToVoid<SCBridgeInterface> for &'a SCBridgeInterface {
        fn to_void(&self) -> *const os::raw::c_void {
            self.as_concrete_TypeRef().as_void_ptr()
        }
    }
    unsafe impl ToVoid<SCBridgeInterface> for SCBridgeInterface {
        fn to_void(&self) -> *const os::raw::c_void {
            self.as_concrete_TypeRef().as_void_ptr()
        }
    }
    unsafe impl ToVoid<SCBridgeInterface> for SCBridgeInterfaceRef {
        fn to_void(&self) -> *const os::raw::c_void {
            self.as_void_ptr()
        }
    }
    unsafe impl SCNetworkInterfaceSubClass for SCBridgeInterface {
        const INTERFACE_TYPE: SCNetworkInterfaceType = SCNetworkInterfaceType::Bridge;
    }
};

impl SCBridgeInterface {
    /// Retrieve all network capable devices on the system that can be added to a bridge interface.
    pub fn get_available_member_interfaces(prefs: &SCPreferences) -> CFArray<SCNetworkInterface> {
        unsafe {
            let array_ptr = SCBridgeInterfaceCopyAvailableMemberInterfaces(prefs.as_concrete_TypeRef());
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<SCNetworkInterface>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Retrieve all current bridge interfaces.
    pub fn get_interfaces(prefs: &SCPreferences) -> CFArray<Self> {
        unsafe {
            let array_ptr = SCBridgeInterfaceCopyAll(prefs.as_concrete_TypeRef());
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<Self>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Creates a new SCBridgeInterface interface. Or `None` if an error occurred.
    pub fn create(prefs: &SCPreferences) -> Option<Self> {
        unsafe {
            let bridge_ref = SCBridgeInterfaceCreate(prefs.as_concrete_TypeRef());
            if !bridge_ref.is_null() {
                Some(Self::wrap_under_create_rule(bridge_ref))
            } else {
                None
            }
        }
    }

    /// Returns a [`bool`] value indicating whether the bridge interface allows members with
    /// configured services.
    pub fn configured_members_allowed(&self) -> bool {
        unsafe { SCBridgeInterfaceGetAllowConfiguredMembers(self.0) != 0 }
    }

    /// Returns the member interfaces for the specified bridge interface.
    pub fn member_interfaces(&self) -> CFArray<SCNetworkInterface> {
        unsafe {
            let array_ptr = SCBridgeInterfaceGetMemberInterfaces(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<SCNetworkInterface>::wrap_under_get_rule(array_ptr)
        }
    }

    /// Returns the configuration settings associated with the bridge interface. Or `None` if no
    /// changes to the default configuration have been saved.
    pub fn options(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let dictionary_ref = SCBridgeInterfaceGetOptions(self.as_concrete_TypeRef());
            if !dictionary_ref.is_null() {
                Some(CFDictionary::wrap_under_get_rule(dictionary_ref))
            } else {
                None
            }
        }
    }

    /// Removes the SCBridgeInterface from the configuration.
    ///
    /// Returns: `true` if the interface was removed; `false` if an error was encountered.
    pub fn remove(self) -> bool {
        (unsafe { SCBridgeInterfaceRemove(self.0) }) != 0
    }

    /// Allow adding member interfaces to the bridge that have configured services.
    ///
    /// Returns: `true` if the change was successful; `false` otherwise.
    pub fn set_configured_members_allowed(&mut self, enable: bool) -> bool {
        (unsafe { SCBridgeInterfaceSetAllowConfiguredMembers(self.0, enable as Boolean) }) != 0
    }

    /// Sets the member interfaces for the specified bridge interface.
    ///
    /// Returns: `true` if the configuration was stored; `false` if an error was encountered.
    pub fn set_member_interfaces(&mut self, members: &CFArray<SCNetworkInterface>) -> bool {
        (unsafe { SCBridgeInterfaceSetMemberInterfaces(self.0, members.as_concrete_TypeRef()) }) != 0
    }

    /// Sets the configuration settings for the specified bridge interface.
    ///
    /// Returns: `true` if the configuration was stored; `false` if an error occurred.
    pub fn set_options(&mut self, new_options: &CFDictionary<CFString, CFType>) -> bool {
        (unsafe { SCBridgeInterfaceSetOptions(self.0, new_options.as_concrete_TypeRef()) }) != 0
    }
}
