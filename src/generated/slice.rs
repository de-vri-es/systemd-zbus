//! # DBus interface proxies for: `org.freedesktop.systemd1.Slice`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `org.freedesktop.systemd1.Slice.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Slice",
    default_service = "org.freedesktop.systemd1"
)]
trait Slice {
    /// AttachProcesses method
    fn attach_processes(&self, subcgroup: &str, pids: &[u32]) -> zbus::Result<()>;

    /// GetProcesses method
    fn get_processes(&self) -> zbus::Result<Vec<(String, u32, String)>>;

    /// AllowedCPUs property
    #[dbus_proxy(property, name = "AllowedCPUs")]
    fn allowed_cpus(&self) -> zbus::Result<Vec<u8>>;

    /// AllowedMemoryNodes property
    #[dbus_proxy(property)]
    fn allowed_memory_nodes(&self) -> zbus::Result<Vec<u8>>;

    /// BPFProgram property
    #[dbus_proxy(property, name = "BPFProgram")]
    fn bpf_program(&self) -> zbus::Result<Vec<(String, String)>>;

    /// BlockIOAccounting property
    #[dbus_proxy(property, name = "BlockIOAccounting")]
    fn block_io_accounting(&self) -> zbus::Result<bool>;

    /// BlockIODeviceWeight property
    #[dbus_proxy(property, name = "BlockIODeviceWeight")]
    fn block_io_device_weight(&self) -> zbus::Result<Vec<(String, u64)>>;

    /// BlockIOReadBandwidth property
    #[dbus_proxy(property, name = "BlockIOReadBandwidth")]
    fn block_io_read_bandwidth(&self) -> zbus::Result<Vec<(String, u64)>>;

    /// BlockIOWeight property
    #[dbus_proxy(property, name = "BlockIOWeight")]
    fn block_io_weight(&self) -> zbus::Result<u64>;

    /// BlockIOWriteBandwidth property
    #[dbus_proxy(property, name = "BlockIOWriteBandwidth")]
    fn block_io_write_bandwidth(&self) -> zbus::Result<Vec<(String, u64)>>;

    /// CPUAccounting property
    #[dbus_proxy(property, name = "CPUAccounting")]
    fn cpu_accounting(&self) -> zbus::Result<bool>;

    /// CPUQuotaPerSecUSec property
    #[dbus_proxy(property, name = "CPUQuotaPerSecUSec")]
    fn cpu_quota_per_sec_usec(&self) -> zbus::Result<u64>;

    /// CPUQuotaPeriodUSec property
    #[dbus_proxy(property, name = "CPUQuotaPeriodUSec")]
    fn cpu_quota_period_usec(&self) -> zbus::Result<u64>;

    /// CPUShares property
    #[dbus_proxy(property, name = "CPUShares")]
    fn cpu_shares(&self) -> zbus::Result<u64>;

    /// CPUUsageNSec property
    #[dbus_proxy(property, name = "CPUUsageNSec")]
    fn cpu_usage_nsec(&self) -> zbus::Result<u64>;

    /// CPUWeight property
    #[dbus_proxy(property, name = "CPUWeight")]
    fn cpu_weight(&self) -> zbus::Result<u64>;

    /// ControlGroup property
    #[dbus_proxy(property)]
    fn control_group(&self) -> zbus::Result<String>;

    /// ControlGroupId property
    #[dbus_proxy(property)]
    fn control_group_id(&self) -> zbus::Result<u64>;

    /// DefaultMemoryLow property
    #[dbus_proxy(property)]
    fn default_memory_low(&self) -> zbus::Result<u64>;

    /// DefaultMemoryMin property
    #[dbus_proxy(property)]
    fn default_memory_min(&self) -> zbus::Result<u64>;

    /// Delegate property
    #[dbus_proxy(property)]
    fn delegate(&self) -> zbus::Result<bool>;

    /// DelegateControllers property
    #[dbus_proxy(property)]
    fn delegate_controllers(&self) -> zbus::Result<Vec<String>>;

    /// DeviceAllow property
    #[dbus_proxy(property)]
    fn device_allow(&self) -> zbus::Result<Vec<(String, String)>>;

    /// DevicePolicy property
    #[dbus_proxy(property)]
    fn device_policy(&self) -> zbus::Result<String>;

    /// DisableControllers property
    #[dbus_proxy(property)]
    fn disable_controllers(&self) -> zbus::Result<Vec<String>>;

    /// EffectiveCPUs property
    #[dbus_proxy(property, name = "EffectiveCPUs")]
    fn effective_cpus(&self) -> zbus::Result<Vec<u8>>;

    /// EffectiveMemoryNodes property
    #[dbus_proxy(property)]
    fn effective_memory_nodes(&self) -> zbus::Result<Vec<u8>>;

    /// IOAccounting property
    #[dbus_proxy(property, name = "IOAccounting")]
    fn io_accounting(&self) -> zbus::Result<bool>;

    /// IODeviceLatencyTargetUSec property
    #[dbus_proxy(property, name = "IODeviceLatencyTargetUSec")]
    fn io_device_latency_target_usec(&self) -> zbus::Result<Vec<(String, u64)>>;

    /// IODeviceWeight property
    #[dbus_proxy(property, name = "IODeviceWeight")]
    fn io_device_weight(&self) -> zbus::Result<Vec<(String, u64)>>;

    /// IOReadBandwidthMax property
    #[dbus_proxy(property, name = "IOReadBandwidthMax")]
    fn io_read_bandwidth_max(&self) -> zbus::Result<Vec<(String, u64)>>;

    /// IOReadBytes property
    #[dbus_proxy(property, name = "IOReadBytes")]
    fn io_read_bytes(&self) -> zbus::Result<u64>;

    /// IOReadIOPSMax property
    #[dbus_proxy(property, name = "IOReadIOPSMax")]
    fn io_read_iops_max(&self) -> zbus::Result<Vec<(String, u64)>>;

    /// IOReadOperations property
    #[dbus_proxy(property, name = "IOReadOperations")]
    fn io_read_operations(&self) -> zbus::Result<u64>;

    /// IOWeight property
    #[dbus_proxy(property, name = "IOWeight")]
    fn io_weight(&self) -> zbus::Result<u64>;

    /// IOWriteBandwidthMax property
    #[dbus_proxy(property, name = "IOWriteBandwidthMax")]
    fn io_write_bandwidth_max(&self) -> zbus::Result<Vec<(String, u64)>>;

    /// IOWriteBytes property
    #[dbus_proxy(property, name = "IOWriteBytes")]
    fn io_write_bytes(&self) -> zbus::Result<u64>;

    /// IOWriteIOPSMax property
    #[dbus_proxy(property, name = "IOWriteIOPSMax")]
    fn io_write_iops_max(&self) -> zbus::Result<Vec<(String, u64)>>;

    /// IOWriteOperations property
    #[dbus_proxy(property, name = "IOWriteOperations")]
    fn io_write_operations(&self) -> zbus::Result<u64>;

    /// IPAccounting property
    #[dbus_proxy(property, name = "IPAccounting")]
    fn ip_accounting(&self) -> zbus::Result<bool>;

    /// IPAddressAllow property
    #[dbus_proxy(property, name = "IPAddressAllow")]
    fn ip_address_allow(&self) -> zbus::Result<Vec<(i32, Vec<u8>, u32)>>;

    /// IPAddressDeny property
    #[dbus_proxy(property, name = "IPAddressDeny")]
    fn ip_address_deny(&self) -> zbus::Result<Vec<(i32, Vec<u8>, u32)>>;

    /// IPEgressBytes property
    #[dbus_proxy(property, name = "IPEgressBytes")]
    fn ip_egress_bytes(&self) -> zbus::Result<u64>;

    /// IPEgressFilterPath property
    #[dbus_proxy(property, name = "IPEgressFilterPath")]
    fn ip_egress_filter_path(&self) -> zbus::Result<Vec<String>>;

    /// IPEgressPackets property
    #[dbus_proxy(property, name = "IPEgressPackets")]
    fn ip_egress_packets(&self) -> zbus::Result<u64>;

    /// IPIngressBytes property
    #[dbus_proxy(property, name = "IPIngressBytes")]
    fn ip_ingress_bytes(&self) -> zbus::Result<u64>;

    /// IPIngressFilterPath property
    #[dbus_proxy(property, name = "IPIngressFilterPath")]
    fn ip_ingress_filter_path(&self) -> zbus::Result<Vec<String>>;

    /// IPIngressPackets property
    #[dbus_proxy(property, name = "IPIngressPackets")]
    fn ip_ingress_packets(&self) -> zbus::Result<u64>;

    /// ManagedOOMMemoryPressure property
    #[dbus_proxy(property, name = "ManagedOOMMemoryPressure")]
    fn managed_oom_memory_pressure(&self) -> zbus::Result<String>;

    /// ManagedOOMMemoryPressureLimit property
    #[dbus_proxy(property, name = "ManagedOOMMemoryPressureLimit")]
    fn managed_oom_memory_pressure_limit(&self) -> zbus::Result<u32>;

    /// ManagedOOMPreference property
    #[dbus_proxy(property, name = "ManagedOOMPreference")]
    fn managed_oom_preference(&self) -> zbus::Result<String>;

    /// ManagedOOMSwap property
    #[dbus_proxy(property, name = "ManagedOOMSwap")]
    fn managed_oom_swap(&self) -> zbus::Result<String>;

    /// MemoryAccounting property
    #[dbus_proxy(property)]
    fn memory_accounting(&self) -> zbus::Result<bool>;

    /// MemoryAvailable property
    #[dbus_proxy(property)]
    fn memory_available(&self) -> zbus::Result<u64>;

    /// MemoryCurrent property
    #[dbus_proxy(property)]
    fn memory_current(&self) -> zbus::Result<u64>;

    /// MemoryHigh property
    #[dbus_proxy(property)]
    fn memory_high(&self) -> zbus::Result<u64>;

    /// MemoryLimit property
    #[dbus_proxy(property)]
    fn memory_limit(&self) -> zbus::Result<u64>;

    /// MemoryLow property
    #[dbus_proxy(property)]
    fn memory_low(&self) -> zbus::Result<u64>;

    /// MemoryMax property
    #[dbus_proxy(property)]
    fn memory_max(&self) -> zbus::Result<u64>;

    /// MemoryMin property
    #[dbus_proxy(property)]
    fn memory_min(&self) -> zbus::Result<u64>;

    /// MemorySwapMax property
    #[dbus_proxy(property)]
    fn memory_swap_max(&self) -> zbus::Result<u64>;

    /// RestrictNetworkInterfaces property
    #[dbus_proxy(property)]
    fn restrict_network_interfaces(&self) -> zbus::Result<(bool, Vec<String>)>;

    /// Slice property
    #[dbus_proxy(property)]
    fn slice(&self) -> zbus::Result<String>;

    /// SocketBindAllow property
    #[dbus_proxy(property)]
    fn socket_bind_allow(&self) -> zbus::Result<Vec<(i32, i32, u16, u16)>>;

    /// SocketBindDeny property
    #[dbus_proxy(property)]
    fn socket_bind_deny(&self) -> zbus::Result<Vec<(i32, i32, u16, u16)>>;

    /// StartupAllowedCPUs property
    #[dbus_proxy(property, name = "StartupAllowedCPUs")]
    fn startup_allowed_cpus(&self) -> zbus::Result<Vec<u8>>;

    /// StartupAllowedMemoryNodes property
    #[dbus_proxy(property)]
    fn startup_allowed_memory_nodes(&self) -> zbus::Result<Vec<u8>>;

    /// StartupBlockIOWeight property
    #[dbus_proxy(property, name = "StartupBlockIOWeight")]
    fn startup_block_io_weight(&self) -> zbus::Result<u64>;

    /// StartupCPUShares property
    #[dbus_proxy(property, name = "StartupCPUShares")]
    fn startup_cpu_shares(&self) -> zbus::Result<u64>;

    /// StartupCPUWeight property
    #[dbus_proxy(property, name = "StartupCPUWeight")]
    fn startup_cpu_weight(&self) -> zbus::Result<u64>;

    /// StartupIOWeight property
    #[dbus_proxy(property, name = "StartupIOWeight")]
    fn startup_io_weight(&self) -> zbus::Result<u64>;

    /// TasksAccounting property
    #[dbus_proxy(property)]
    fn tasks_accounting(&self) -> zbus::Result<bool>;

    /// TasksCurrent property
    #[dbus_proxy(property)]
    fn tasks_current(&self) -> zbus::Result<u64>;

    /// TasksMax property
    #[dbus_proxy(property)]
    fn tasks_max(&self) -> zbus::Result<u64>;
}
