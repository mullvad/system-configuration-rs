//! Bindings for [`SCNetworkReachability`]
//!
//! [`SCNetworkReachability`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkreachability-g7d

use core_foundation::{
    base::{CFRelease, ToVoid},
    runloop::CFRunLoop,
    string::{CFString, CFStringRef},
};
use system_configuration_sys::{
    libc,
    network_configuration::{
        SCNetworkReachabilityContext, SCNetworkReachabilityCreateWithAddress,
        SCNetworkReachabilityCreateWithAddressPair, SCNetworkReachabilityCreateWithName,
        SCNetworkReachabilityFlags, SCNetworkReachabilityGetFlags, SCNetworkReachabilityRef,
        SCNetworkReachabilityScheduleWithRunLoop, SCNetworkReachabilitySetCallback,
        SCNetworkReachabilityUnscheduleFromRunLoop,
    },
};

use std::{
    ffi::{c_void, CStr},
    net::SocketAddr,
    ptr,
    sync::Arc,
};

/// Failure to determine reachability
#[derive(Debug)]
pub struct ReachabilityError {}

/// Failure to construct a reachability reference from host.
#[derive(Debug)]
pub struct FromHostError {}

/// Failure to schedule a reachability callback on a runloop.
#[derive(Debug)]
pub struct SchedulingError {}

/// Failure to schedule a reachability callback on a runloop.
#[derive(Debug)]
pub struct UnschedulingError {}

/// Failure to set a callback for changes in reachability.
#[derive(Debug)]
pub struct SetCallbackError {}

bitflags::bitflags! {
    /// Rustier interface for [`SCNetworkReachability`].
    ///
    /// [`SCNetworkReachability`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkreachabilityflags
    pub struct ReachabilityFlags: u32 {
        /// The specified node name or address can be reached via a transient connection, such as PPP.
        const TRANSIENT_CONNECTION = 1;
        /// The specified node name or address can be reached using the current network configuration.
        const REACHABLE = 1<<1;
        /// The specified node name or address can be reached using the current network configuration, but a connection must first be established. If this flag is set, the kSCNetworkReachabilityFlagsConnectionOnTraffic flag, kSCNetworkReachabilityFlagsConnectionOnDemand flag, or kSCNetworkReachabilityFlagsIsWWAN flag is also typically set to indicate the type of connection required. If the user must manually make the connection, the kSCNetworkReachabilityFlagsInterventionRequired flag is also set.
        const CONNECTION_REQUIRED = 1<<2;
        /// The specified node name or address can be reached using the current network configuration, but a connection must first be established. Any traffic directed to the specified name or address will initiate the connection.
        const CONNECTION_ON_TRAFFIC = 1<<3;
        /// The specified node name or address can be reached using the current network configuration, but a connection must first be established.
        const INTERVENTION_REQUIRED = 1<<4;
        /// The specified node name or address can be reached using the current network configuration, but a connection must first be established.
        const CONNECTION_ON_DEMAND = 1<<5;
        /// The specified node name or address is one that is associated with a network interface on the current system.
        const IS_LOCAL_ADDRESS = 1<<16;
        /// Network traffic to the specified node name or address will not go through a gateway, but is routed directly to one of the interfaces in the system.
        const IS_DIRECT = 1<<17;
        /// The specified node name or address can be reached via a cellular connection, such as EDGE or GPRS.
        const IS_WWAN = 1<<18;
    }
}

impl ReachabilityFlags {
    fn mut_inner(&mut self) -> &mut u32 {
        &mut self.bits
    }

    fn from_raw(bits: SCNetworkReachabilityFlags) -> Self {
        Self { bits }
    }
}


/// A network address or host for which the connectivity can be determined.
pub struct NetworkReachability {
    inner: Arc<NetworkReachabilityInner>,
}


impl NetworkReachability {
    /// Construct a NetworkReachability struct with a local and a remote socket address.
    ///
    /// See [``SCNetworkReachabilityCreateWithAddressPair``] for details.
    ///
    /// [``SCNetworkReachabilityCreateWithAddressPair``]: https://developer.apple.com/documentation/systemconfiguration/1514908-scnetworkreachabilitycreatewitha?language=objc
    pub fn from_addr_pair(local: SocketAddr, remote: SocketAddr) -> NetworkReachability {
        let local_ptr = to_c_sockaddr(local);
        let remote_ptr = to_c_sockaddr(remote);

        let ptr = unsafe {
            SCNetworkReachabilityCreateWithAddressPair(std::ptr::null(), local_ptr, remote_ptr)
        };

        unsafe {
            let _ = Box::from_raw(local_ptr);
            let _ = Box::from_raw(remote_ptr);
        }

        Self::from_ptr(ptr)
    }

    /// Construct a Reachability from either a hostname or a network node
    ///
    /// See [`SCNetworkReachabilityCreateWithName`] for details.
    ///
    /// [`SCNetworkReachabilityCreateWithName`]: https://developer.apple.com/documentation/systemconfiguration/1514904-scnetworkreachabilitycreatewithn?language=objc
    pub fn from_host(host: &CStr) -> Result<Self, FromHostError> {
        let ptr = unsafe { SCNetworkReachabilityCreateWithName(ptr::null(), host.as_ptr()) };
        if ptr.is_null() {
            Err(FromHostError {})
        } else {
            Ok(Self::from_ptr(ptr))
        }
    }

    fn from_ptr(ptr: SCNetworkReachabilityRef) -> Self {
        Self {
            inner: Arc::new(NetworkReachabilityInner { ptr }),
        }
    }

    /// Return a flag indicating whether the specified network address is reachable.
    ///
    /// See [`SCNetworkReachabilityGetFlags`] for details.
    ///
    /// [`SCNetworkReachabilityGetFlags`]: https://developer.apple.com/documentation/systemconfiguration/1514924-scnetworkreachabilitygetflags?language=objc
    pub fn reachability(&self) -> Result<ReachabilityFlags, ReachabilityError> {
        let mut flags = ReachabilityFlags::empty();
        if unsafe { SCNetworkReachabilityGetFlags(self.inner.ptr, flags.mut_inner()) } == 0u8 {
            return Err(ReachabilityError {});
        }

        Ok(flags)
    }

    /// Schedule callback with runloop.
    ///
    /// See [`SCNetworkReachabilityScheduleFromRunLoop`] for details.
    ///
    /// [`SCNetworkReachabilityScheduleFromRunLoop`]: https://developer.apple.com/documentation/systemconfiguration/1514894-scnetworkreachabilityschedulewit?language=objc
    pub fn schedule_with_runloop(
        &self,
        run_loop: &CFRunLoop,
        run_loop_mode: CFStringRef,
    ) -> Result<(), SchedulingError> {
        if unsafe {
            SCNetworkReachabilityScheduleWithRunLoop(
                self.inner.ptr,
                run_loop.to_void() as *mut _,
                run_loop_mode,
            )
        } == 0u8
        {
            Err(SchedulingError {})
        } else {
            Ok(())
        }
    }

    /// Unschedule from run loop.
    ///
    /// See [`SCNetworkReachabilityUnscheduleFromRunLoop`] for details.
    ///
    /// [`SCNetworkReachabilityUnscheduleFromRunLoop`]: https://developer.apple.com/documentation/systemconfiguration/1514899-scnetworkreachabilityunschedulef?language=objc
    pub fn unschedule_from_runloop(
        &self,
        run_loop: &CFRunLoop,
        run_loop_mode: CFStringRef,
    ) -> Result<(), UnschedulingError> {
        if unsafe {
            SCNetworkReachabilityUnscheduleFromRunLoop(
                self.inner.ptr,
                run_loop.to_void() as *mut _,
                run_loop_mode,
            )
        } == 0u8
        {
            Err(UnschedulingError {})
        } else {
            Ok(())
        }
    }

    /// Sets callback that is run whenever network connectivity changes. For the callback to be
    /// invoked, the `NetworkReachability` has to be registered on a run loop. Calling this
    /// function multiple times will clear the subsequently set callback. Returns false if setting
    /// a callback failed.
    ///
    /// See [`SCNetworkReachabilityContext`] for details.
    ///
    /// [`SCNetworkReachabilityContext`]: https://developer.apple.com/documentation/systemconfiguration/1514903-scnetworkreachabilitysetcallback?language=objc
    pub fn set_callback<F: FnMut(ReachabilityFlags)>(
        &mut self,
        callback: F,
    ) -> Result<(), SetCallbackError> {
        let callback = Box::new(NetworkReachabilityCallbackContext::new(
            self.inner.clone(),
            callback,
        ));

        let mut callback_context = SCNetworkReachabilityContext {
            version: 0,
            info: Box::into_raw(callback) as *mut _,
            retain: None,
            release: Some(Self::release_context::<F>),
            copyDescription: Some(Self::copy_ctx_description),
        };

        if unsafe {
            SCNetworkReachabilitySetCallback(
                self.inner.ptr,
                Some(Self::callback::<F>),
                &mut callback_context,
            )
        } == 0u8
        {
            Err(SetCallbackError {})
        } else {
            Ok(())
        }
    }

    extern "C" fn callback<T: FnMut(ReachabilityFlags)>(
        _target: SCNetworkReachabilityRef,
        flags: SCNetworkReachabilityFlags,
        context: *mut c_void,
    ) {
        let mut context: Box<NetworkReachabilityCallbackContext<T>> =
            unsafe { Box::from_raw(context as *mut _) };
        (context.callback)(ReachabilityFlags::from_raw(flags));
        std::mem::forget(context);
    }

    extern "C" fn copy_ctx_description(_ctx: *const c_void) -> CFStringRef {
        let description = CFString::new("NetworkRechability's callback context");
        let ptr = description.to_void().cast();
        std::mem::forget(description);
        ptr as *const _
    }

    extern "C" fn release_context<T: FnMut(ReachabilityFlags)>(ctx: *const c_void) {
        unsafe {
            let _: Box<NetworkReachabilityCallbackContext<T>> = Box::from_raw(ctx as *mut _);
        }
    }
}

impl From<SocketAddr> for NetworkReachability {
    fn from(addr: SocketAddr) -> Self {
        let sockaddr_ptr = to_c_sockaddr(addr);

        let ptr = unsafe { SCNetworkReachabilityCreateWithAddress(std::ptr::null(), sockaddr_ptr) };
        unsafe {
            let _ = Box::from_raw(sockaddr_ptr);
        }

        NetworkReachability::from_ptr(ptr)
    }
}


struct NetworkReachabilityInner {
    ptr: SCNetworkReachabilityRef,
}

impl Drop for NetworkReachabilityInner {
    fn drop(&mut self) {
        unsafe { CFRelease(self.ptr) }
    }
}

struct NetworkReachabilityCallbackContext<T: FnMut(ReachabilityFlags)> {
    _host: Arc<NetworkReachabilityInner>,
    callback: T,
}

impl<T: FnMut(ReachabilityFlags)> NetworkReachabilityCallbackContext<T> {
    fn new(host: Arc<NetworkReachabilityInner>, callback: T) -> Self {
        Self {
            _host: host,
            callback,
        }
    }
}

/// Allocates a libc::sockaddr compatible struct and fills it with either a libc::sockaddr_in or a
/// libc::sockaddr_in6, depending on the passed in standard library SocketAddr. The returned
/// pointer should be freed with `Box::from_raw`.
fn to_c_sockaddr(addr: SocketAddr) -> *mut libc::sockaddr {
    match addr {
        SocketAddr::V4(addr) => {
            let ptr = Box::new(libc::sockaddr_in {
                sin_len: std::mem::size_of::<libc::sockaddr_in>() as u8,
                sin_family: libc::AF_INET as u8,
                sin_port: addr.port(),
                sin_addr: libc::in_addr {
                    s_addr: u32::from(*addr.ip()),
                },
                sin_zero: [0i8; 8],
            });
            Box::into_raw(ptr).cast()
        }
        SocketAddr::V6(addr) => {
            let ptr = Box::new(libc::sockaddr_in6 {
                sin6_len: std::mem::size_of::<libc::sockaddr_in6>() as u8,
                sin6_family: libc::AF_INET6 as u8,
                sin6_port: addr.port(),
                sin6_flowinfo: 0,
                sin6_addr: libc::in6_addr {
                    s6_addr: addr.ip().octets(),
                },
                sin6_scope_id: 0,
            });

            Box::into_raw(ptr).cast()
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
    use std::ffi::CString;

    #[test]
    fn test_network_reachability_from_addr() {
        let sockaddrs = vec![
            "0.0.0.0:0".parse::<SocketAddr>().unwrap(),
            "[::0]:0".parse::<SocketAddr>().unwrap(),
        ];

        for addr in sockaddrs {
            let mut reachability = NetworkReachability::from(addr);
            if reachability.inner.ptr.is_null() {
                panic!(
                    "Failed to construct a NetworkReachability struct with {}",
                    addr
                );
            }
            reachability.set_callback(|_| {}).unwrap();
            reachability
                .schedule_with_runloop(&CFRunLoop::get_current(), unsafe { kCFRunLoopCommonModes })
                .unwrap();
            reachability
                .unschedule_from_runloop(&CFRunLoop::get_current(), unsafe {
                    kCFRunLoopCommonModes
                })
                .unwrap();
        }
    }


    #[test]
    fn test_sockaddr_pair_reachability() {
        let pairs = vec![
            ("0.0.0.0:0", "[::0]:0"),
            ("[::0]:0", "0.0.0.0:0"),
            ("[::0]:0", "[::0]:0"),
            ("0.0.0.0:0", "0.0.0.0:0"),
        ]
        .into_iter()
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()));

        for (local, remote) in pairs {
            let mut reachability = NetworkReachability::from_addr_pair(local, remote);
            if reachability.inner.ptr.is_null() {
                panic!(
                    "Failed to construct a NetworkReachability struct with address pair {} - {}",
                    local, remote
                );
            }
            reachability.set_callback(|_| {}).unwrap();
            reachability
                .schedule_with_runloop(&CFRunLoop::get_current(), unsafe { kCFRunLoopCommonModes })
                .unwrap();
            reachability
                .unschedule_from_runloop(&CFRunLoop::get_current(), unsafe {
                    kCFRunLoopCommonModes
                })
                .unwrap();
        }
    }

    #[test]
    fn test_reachability_ref_from_host() {
        let valid_inputs = vec!["example.com", "host-in-local-network", "en0"];

        let get_cstring = |input: &str| CString::new(input).unwrap();

        for input in valid_inputs.into_iter().map(get_cstring) {
            match NetworkReachability::from_host(&input) {
                Ok(mut reachability) => {
                    reachability.set_callback(|_| {}).unwrap();
                    reachability
                        .schedule_with_runloop(&CFRunLoop::get_current(), unsafe {
                            kCFRunLoopCommonModes
                        })
                        .unwrap();
                    reachability
                        .unschedule_from_runloop(&CFRunLoop::get_current(), unsafe {
                            kCFRunLoopCommonModes
                        })
                        .unwrap();
                }
                Err(_) => {
                    panic!(
                        "Failed to construct a NetworkReachability from {}",
                        input.to_string_lossy()
                    );
                }
            }
        }

        // Can only testify that an empty string is invalid, everything else seems to work
        if let Ok(_) = NetworkReachability::from_host(&get_cstring("")) {
            panic!("Constructed valid NetworkReachability from empty string");
        }
    }


    unsafe impl Send for NetworkReachability {}

    #[test]
    fn assert_infallibility_of_setting_a_callback() {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut reachability =
                NetworkReachability::from("0.0.0.0:0".parse::<SocketAddr>().unwrap());
            reachability.set_callback(|_| {}).unwrap();
            reachability
                .schedule_with_runloop(&CFRunLoop::get_current(), unsafe { kCFRunLoopCommonModes })
                .unwrap();
            reachability.set_callback(|_| {}).unwrap();
            let _ = tx.send(reachability);
            CFRunLoop::run_current();
        });
        let mut reachability = rx.recv().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        reachability.set_callback(|_| {}).unwrap();
    }
}
