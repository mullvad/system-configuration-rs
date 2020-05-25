// automatically generated by rust-bindgen

// Generated using:
// bindgen 0.54.0
// macOS SDK 10.15.4.

use core_foundation_sys::string::CFStringRef;


extern "C" {
    pub static kSCResvLink: CFStringRef;

    pub static kSCResvInactive: CFStringRef;

    pub static kSCPropInterfaceName: CFStringRef;

    pub static kSCPropMACAddress: CFStringRef;

    pub static kSCPropUserDefinedName: CFStringRef;

    pub static kSCPropVersion: CFStringRef;

    pub static kSCPrefCurrentSet: CFStringRef;

    pub static kSCPrefNetworkServices: CFStringRef;

    pub static kSCPrefSets: CFStringRef;

    pub static kSCPrefSystem: CFStringRef;

    pub static kSCCompNetwork: CFStringRef;

    pub static kSCCompService: CFStringRef;

    pub static kSCCompGlobal: CFStringRef;

    pub static kSCCompHostNames: CFStringRef;

    pub static kSCCompInterface: CFStringRef;

    pub static kSCCompSystem: CFStringRef;

    pub static kSCCompUsers: CFStringRef;

    pub static kSCCompAnyRegex: CFStringRef;

    pub static kSCEntNetAirPort: CFStringRef;

    pub static kSCEntNetDHCP: CFStringRef;

    pub static kSCEntNetDNS: CFStringRef;

    pub static kSCEntNetEthernet: CFStringRef;

    pub static kSCEntNetFireWire: CFStringRef;

    pub static kSCEntNetInterface: CFStringRef;

    pub static kSCEntNetIPSec: CFStringRef;

    pub static kSCEntNetIPv4: CFStringRef;

    pub static kSCEntNetIPv6: CFStringRef;

    pub static kSCEntNetL2TP: CFStringRef;

    pub static kSCEntNetLink: CFStringRef;

    pub static kSCEntNetModem: CFStringRef;

    pub static kSCEntNetPPP: CFStringRef;

    pub static kSCEntNetPPPoE: CFStringRef;

    pub static kSCEntNetPPPSerial: CFStringRef;

    pub static kSCEntNetPPTP: CFStringRef;

    pub static kSCEntNetProxies: CFStringRef;

    pub static kSCEntNetSMB: CFStringRef;

    pub static kSCEntNet6to4: CFStringRef;

    pub static kSCPropNetOverridePrimary: CFStringRef;

    pub static kSCPropNetServiceOrder: CFStringRef;

    pub static kSCPropNetPPPOverridePrimary: CFStringRef;

    pub static kSCPropNetInterfaces: CFStringRef;

    pub static kSCPropNetLocalHostName: CFStringRef;

    pub static kSCPropNetAirPortAllowNetCreation: CFStringRef;

    pub static kSCPropNetAirPortAuthPassword: CFStringRef;

    pub static kSCPropNetAirPortAuthPasswordEncryption: CFStringRef;

    pub static kSCPropNetAirPortJoinMode: CFStringRef;

    pub static kSCPropNetAirPortPowerEnabled: CFStringRef;

    pub static kSCPropNetAirPortPreferredNetwork: CFStringRef;

    pub static kSCPropNetAirPortSavePasswords: CFStringRef;

    pub static kSCValNetAirPortJoinModeAutomatic: CFStringRef;

    pub static kSCValNetAirPortJoinModePreferred: CFStringRef;

    pub static kSCValNetAirPortJoinModeRanked: CFStringRef;

    pub static kSCValNetAirPortJoinModeRecent: CFStringRef;

    pub static kSCValNetAirPortJoinModeStrongest: CFStringRef;

    pub static kSCValNetAirPortAuthPasswordEncryptionKeychain: CFStringRef;

    pub static kSCPropNetDNSDomainName: CFStringRef;

    pub static kSCPropNetDNSOptions: CFStringRef;

    pub static kSCPropNetDNSSearchDomains: CFStringRef;

    pub static kSCPropNetDNSSearchOrder: CFStringRef;

    pub static kSCPropNetDNSServerAddresses: CFStringRef;

    pub static kSCPropNetDNSServerPort: CFStringRef;

    pub static kSCPropNetDNSServerTimeout: CFStringRef;

    pub static kSCPropNetDNSSortList: CFStringRef;

    pub static kSCPropNetDNSSupplementalMatchDomains: CFStringRef;

    pub static kSCPropNetDNSSupplementalMatchOrders: CFStringRef;

    pub static kSCPropNetEthernetMediaSubType: CFStringRef;

    pub static kSCPropNetEthernetMediaOptions: CFStringRef;

    pub static kSCPropNetEthernetMTU: CFStringRef;

    pub static kSCPropNetInterfaceDeviceName: CFStringRef;

    pub static kSCPropNetInterfaceHardware: CFStringRef;

    pub static kSCPropNetInterfaceType: CFStringRef;

    pub static kSCPropNetInterfaceSubType: CFStringRef;

    pub static kSCPropNetInterfaceSupportsModemOnHold: CFStringRef;

    pub static kSCValNetInterfaceTypeEthernet: CFStringRef;

    pub static kSCValNetInterfaceTypeFireWire: CFStringRef;

    pub static kSCValNetInterfaceTypePPP: CFStringRef;

    pub static kSCValNetInterfaceType6to4: CFStringRef;

    pub static kSCValNetInterfaceTypeIPSec: CFStringRef;

    pub static kSCValNetInterfaceSubTypePPPoE: CFStringRef;

    pub static kSCValNetInterfaceSubTypePPPSerial: CFStringRef;

    pub static kSCValNetInterfaceSubTypePPTP: CFStringRef;

    pub static kSCValNetInterfaceSubTypeL2TP: CFStringRef;

    pub static kSCPropNetIPSecAuthenticationMethod: CFStringRef;

    pub static kSCPropNetIPSecLocalCertificate: CFStringRef;

    pub static kSCPropNetIPSecLocalIdentifier: CFStringRef;

    pub static kSCPropNetIPSecLocalIdentifierType: CFStringRef;

    pub static kSCPropNetIPSecSharedSecret: CFStringRef;

    pub static kSCPropNetIPSecSharedSecretEncryption: CFStringRef;

    pub static kSCPropNetIPSecConnectTime: CFStringRef;

    pub static kSCPropNetIPSecRemoteAddress: CFStringRef;

    pub static kSCPropNetIPSecStatus: CFStringRef;

    pub static kSCPropNetIPSecXAuthEnabled: CFStringRef;

    pub static kSCPropNetIPSecXAuthName: CFStringRef;

    pub static kSCPropNetIPSecXAuthPassword: CFStringRef;

    pub static kSCPropNetIPSecXAuthPasswordEncryption: CFStringRef;

    pub static kSCValNetIPSecAuthenticationMethodSharedSecret: CFStringRef;

    pub static kSCValNetIPSecAuthenticationMethodCertificate: CFStringRef;

    pub static kSCValNetIPSecAuthenticationMethodHybrid: CFStringRef;

    pub static kSCValNetIPSecLocalIdentifierTypeKeyID: CFStringRef;

    pub static kSCValNetIPSecSharedSecretEncryptionKeychain: CFStringRef;

    pub static kSCValNetIPSecXAuthPasswordEncryptionKeychain: CFStringRef;

    pub static kSCValNetIPSecXAuthPasswordEncryptionPrompt: CFStringRef;

    pub static kSCPropNetIPv4Addresses: CFStringRef;

    pub static kSCPropNetIPv4ConfigMethod: CFStringRef;

    pub static kSCPropNetIPv4DHCPClientID: CFStringRef;

    pub static kSCPropNetIPv4Router: CFStringRef;

    pub static kSCPropNetIPv4SubnetMasks: CFStringRef;

    pub static kSCPropNetIPv4DestAddresses: CFStringRef;

    pub static kSCPropNetIPv4BroadcastAddresses: CFStringRef;

    pub static kSCValNetIPv4ConfigMethodAutomatic: CFStringRef;

    pub static kSCValNetIPv4ConfigMethodBOOTP: CFStringRef;

    pub static kSCValNetIPv4ConfigMethodDHCP: CFStringRef;

    pub static kSCValNetIPv4ConfigMethodINFORM: CFStringRef;

    pub static kSCValNetIPv4ConfigMethodLinkLocal: CFStringRef;

    pub static kSCValNetIPv4ConfigMethodManual: CFStringRef;

    pub static kSCValNetIPv4ConfigMethodPPP: CFStringRef;

    pub static kSCPropNetIPv6Addresses: CFStringRef;

    pub static kSCPropNetIPv6ConfigMethod: CFStringRef;

    pub static kSCPropNetIPv6DestAddresses: CFStringRef;

    pub static kSCPropNetIPv6Flags: CFStringRef;

    pub static kSCPropNetIPv6PrefixLength: CFStringRef;

    pub static kSCPropNetIPv6Router: CFStringRef;

    pub static kSCValNetIPv6ConfigMethodAutomatic: CFStringRef;

    pub static kSCValNetIPv6ConfigMethodLinkLocal: CFStringRef;

    pub static kSCValNetIPv6ConfigMethodManual: CFStringRef;

    pub static kSCValNetIPv6ConfigMethodRouterAdvertisement: CFStringRef;

    pub static kSCValNetIPv6ConfigMethod6to4: CFStringRef;

    pub static kSCPropNet6to4Relay: CFStringRef;

    pub static kSCPropNetLinkActive: CFStringRef;

    pub static kSCPropNetLinkDetaching: CFStringRef;

    pub static kSCPropNetModemAccessPointName: CFStringRef;

    pub static kSCPropNetModemConnectionPersonality: CFStringRef;

    pub static kSCPropNetModemConnectionScript: CFStringRef;

    pub static kSCPropNetModemConnectSpeed: CFStringRef;

    pub static kSCPropNetModemDataCompression: CFStringRef;

    pub static kSCPropNetModemDeviceContextID: CFStringRef;

    pub static kSCPropNetModemDeviceModel: CFStringRef;

    pub static kSCPropNetModemDeviceVendor: CFStringRef;

    pub static kSCPropNetModemDialMode: CFStringRef;

    pub static kSCPropNetModemErrorCorrection: CFStringRef;

    pub static kSCPropNetModemHoldCallWaitingAudibleAlert: CFStringRef;

    pub static kSCPropNetModemHoldDisconnectOnAnswer: CFStringRef;

    pub static kSCPropNetModemHoldEnabled: CFStringRef;

    pub static kSCPropNetModemHoldReminder: CFStringRef;

    pub static kSCPropNetModemHoldReminderTime: CFStringRef;

    pub static kSCPropNetModemNote: CFStringRef;

    pub static kSCPropNetModemPulseDial: CFStringRef;

    pub static kSCPropNetModemSpeaker: CFStringRef;

    pub static kSCPropNetModemSpeed: CFStringRef;

    pub static kSCValNetModemDialModeIgnoreDialTone: CFStringRef;

    pub static kSCValNetModemDialModeManual: CFStringRef;

    pub static kSCValNetModemDialModeWaitForDialTone: CFStringRef;

    pub static kSCPropNetPPPACSPEnabled: CFStringRef;

    pub static kSCPropNetPPPConnectTime: CFStringRef;

    pub static kSCPropNetPPPDeviceLastCause: CFStringRef;

    pub static kSCPropNetPPPDialOnDemand: CFStringRef;

    pub static kSCPropNetPPPDisconnectOnFastUserSwitch: CFStringRef;

    pub static kSCPropNetPPPDisconnectOnIdle: CFStringRef;

    pub static kSCPropNetPPPDisconnectOnIdleTimer: CFStringRef;

    pub static kSCPropNetPPPDisconnectOnLogout: CFStringRef;

    pub static kSCPropNetPPPDisconnectOnSleep: CFStringRef;

    pub static kSCPropNetPPPDisconnectTime: CFStringRef;

    pub static kSCPropNetPPPIdleReminder: CFStringRef;

    pub static kSCPropNetPPPIdleReminderTimer: CFStringRef;

    pub static kSCPropNetPPPLastCause: CFStringRef;

    pub static kSCPropNetPPPLogfile: CFStringRef;

    pub static kSCPropNetPPPPlugins: CFStringRef;

    pub static kSCPropNetPPPRetryConnectTime: CFStringRef;

    pub static kSCPropNetPPPSessionTimer: CFStringRef;

    pub static kSCPropNetPPPStatus: CFStringRef;

    pub static kSCPropNetPPPUseSessionTimer: CFStringRef;

    pub static kSCPropNetPPPVerboseLogging: CFStringRef;

    pub static kSCPropNetPPPAuthEAPPlugins: CFStringRef;

    pub static kSCPropNetPPPAuthName: CFStringRef;

    pub static kSCPropNetPPPAuthPassword: CFStringRef;

    pub static kSCPropNetPPPAuthPasswordEncryption: CFStringRef;

    pub static kSCPropNetPPPAuthPrompt: CFStringRef;

    pub static kSCPropNetPPPAuthProtocol: CFStringRef;

    pub static kSCValNetPPPAuthPasswordEncryptionKeychain: CFStringRef;

    pub static kSCValNetPPPAuthPasswordEncryptionToken: CFStringRef;

    pub static kSCValNetPPPAuthPromptBefore: CFStringRef;

    pub static kSCValNetPPPAuthPromptAfter: CFStringRef;

    pub static kSCValNetPPPAuthProtocolCHAP: CFStringRef;

    pub static kSCValNetPPPAuthProtocolEAP: CFStringRef;

    pub static kSCValNetPPPAuthProtocolMSCHAP1: CFStringRef;

    pub static kSCValNetPPPAuthProtocolMSCHAP2: CFStringRef;

    pub static kSCValNetPPPAuthProtocolPAP: CFStringRef;

    pub static kSCPropNetPPPCommAlternateRemoteAddress: CFStringRef;

    pub static kSCPropNetPPPCommConnectDelay: CFStringRef;

    pub static kSCPropNetPPPCommDisplayTerminalWindow: CFStringRef;

    pub static kSCPropNetPPPCommRedialCount: CFStringRef;

    pub static kSCPropNetPPPCommRedialEnabled: CFStringRef;

    pub static kSCPropNetPPPCommRedialInterval: CFStringRef;

    pub static kSCPropNetPPPCommRemoteAddress: CFStringRef;

    pub static kSCPropNetPPPCommTerminalScript: CFStringRef;

    pub static kSCPropNetPPPCommUseTerminalScript: CFStringRef;

    pub static kSCPropNetPPPCCPEnabled: CFStringRef;

    pub static kSCPropNetPPPCCPMPPE40Enabled: CFStringRef;

    pub static kSCPropNetPPPCCPMPPE128Enabled: CFStringRef;

    pub static kSCPropNetPPPIPCPCompressionVJ: CFStringRef;

    pub static kSCPropNetPPPIPCPUsePeerDNS: CFStringRef;

    pub static kSCPropNetPPPLCPEchoEnabled: CFStringRef;

    pub static kSCPropNetPPPLCPEchoFailure: CFStringRef;

    pub static kSCPropNetPPPLCPEchoInterval: CFStringRef;

    pub static kSCPropNetPPPLCPCompressionACField: CFStringRef;

    pub static kSCPropNetPPPLCPCompressionPField: CFStringRef;

    pub static kSCPropNetPPPLCPMRU: CFStringRef;

    pub static kSCPropNetPPPLCPMTU: CFStringRef;

    pub static kSCPropNetPPPLCPReceiveACCM: CFStringRef;

    pub static kSCPropNetPPPLCPTransmitACCM: CFStringRef;

    pub static kSCPropNetL2TPIPSecSharedSecret: CFStringRef;

    pub static kSCPropNetL2TPIPSecSharedSecretEncryption: CFStringRef;

    pub static kSCPropNetL2TPTransport: CFStringRef;

    pub static kSCValNetL2TPIPSecSharedSecretEncryptionKeychain: CFStringRef;

    pub static kSCValNetL2TPTransportIP: CFStringRef;

    pub static kSCValNetL2TPTransportIPSec: CFStringRef;

    pub static kSCPropNetProxiesExceptionsList: CFStringRef;

    pub static kSCPropNetProxiesExcludeSimpleHostnames: CFStringRef;

    pub static kSCPropNetProxiesFTPEnable: CFStringRef;

    pub static kSCPropNetProxiesFTPPassive: CFStringRef;

    pub static kSCPropNetProxiesFTPPort: CFStringRef;

    pub static kSCPropNetProxiesFTPProxy: CFStringRef;

    pub static kSCPropNetProxiesGopherEnable: CFStringRef;

    pub static kSCPropNetProxiesGopherPort: CFStringRef;

    pub static kSCPropNetProxiesGopherProxy: CFStringRef;

    pub static kSCPropNetProxiesHTTPEnable: CFStringRef;

    pub static kSCPropNetProxiesHTTPPort: CFStringRef;

    pub static kSCPropNetProxiesHTTPProxy: CFStringRef;

    pub static kSCPropNetProxiesHTTPSEnable: CFStringRef;

    pub static kSCPropNetProxiesHTTPSPort: CFStringRef;

    pub static kSCPropNetProxiesHTTPSProxy: CFStringRef;

    pub static kSCPropNetProxiesRTSPEnable: CFStringRef;

    pub static kSCPropNetProxiesRTSPPort: CFStringRef;

    pub static kSCPropNetProxiesRTSPProxy: CFStringRef;

    pub static kSCPropNetProxiesSOCKSEnable: CFStringRef;

    pub static kSCPropNetProxiesSOCKSPort: CFStringRef;

    pub static kSCPropNetProxiesSOCKSProxy: CFStringRef;

    pub static kSCPropNetProxiesProxyAutoConfigEnable: CFStringRef;

    pub static kSCPropNetProxiesProxyAutoConfigJavaScript: CFStringRef;

    pub static kSCPropNetProxiesProxyAutoConfigURLString: CFStringRef;

    pub static kSCPropNetProxiesProxyAutoDiscoveryEnable: CFStringRef;

    pub static kSCPropNetSMBNetBIOSName: CFStringRef;

    pub static kSCPropNetSMBNetBIOSNodeType: CFStringRef;

    pub static kSCPropNetSMBNetBIOSScope: CFStringRef;

    pub static kSCPropNetSMBWINSAddresses: CFStringRef;

    pub static kSCPropNetSMBWorkgroup: CFStringRef;

    pub static kSCValNetSMBNetBIOSNodeTypeBroadcast: CFStringRef;

    pub static kSCValNetSMBNetBIOSNodeTypePeer: CFStringRef;

    pub static kSCValNetSMBNetBIOSNodeTypeMixed: CFStringRef;

    pub static kSCValNetSMBNetBIOSNodeTypeHybrid: CFStringRef;

    pub static kSCEntUsersConsoleUser: CFStringRef;

    pub static kSCPropSystemComputerName: CFStringRef;

    pub static kSCPropSystemComputerNameEncoding: CFStringRef;

    pub static kSCDynamicStoreDomainFile: CFStringRef;

    pub static kSCDynamicStoreDomainPlugin: CFStringRef;

    pub static kSCDynamicStoreDomainSetup: CFStringRef;

    pub static kSCDynamicStoreDomainState: CFStringRef;

    pub static kSCDynamicStoreDomainPrefs: CFStringRef;

    pub static kSCDynamicStorePropSetupCurrentSet: CFStringRef;

    pub static kSCDynamicStorePropSetupLastUpdated: CFStringRef;

    pub static kSCDynamicStorePropNetInterfaces: CFStringRef;

    pub static kSCDynamicStorePropNetPrimaryInterface: CFStringRef;

    pub static kSCDynamicStorePropNetPrimaryService: CFStringRef;

    pub static kSCDynamicStorePropNetServiceIDs: CFStringRef;

    pub static kSCPropUsersConsoleUserName: CFStringRef;

    pub static kSCPropUsersConsoleUserUID: CFStringRef;

    pub static kSCPropUsersConsoleUserGID: CFStringRef;
}
