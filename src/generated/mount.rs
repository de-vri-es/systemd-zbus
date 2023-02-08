//! # DBus interface proxies for: `org.freedesktop.systemd1.Mount`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `org.freedesktop.systemd1.Mount.xml`.
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
    interface = "org.freedesktop.systemd1.Mount",
    default_service = "org.freedesktop.systemd1"
)]
trait Mount {
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

    /// AmbientCapabilities property
    #[dbus_proxy(property)]
    fn ambient_capabilities(&self) -> zbus::Result<u64>;

    /// AppArmorProfile property
    #[dbus_proxy(property)]
    fn app_armor_profile(&self) -> zbus::Result<(bool, String)>;

    /// BPFProgram property
    #[dbus_proxy(property, name = "BPFProgram")]
    fn bpf_program(&self) -> zbus::Result<Vec<(String, String)>>;

    /// BindPaths property
    #[dbus_proxy(property)]
    fn bind_paths(&self) -> zbus::Result<Vec<(String, String, bool, u64)>>;

    /// BindReadOnlyPaths property
    #[dbus_proxy(property)]
    fn bind_read_only_paths(&self) -> zbus::Result<Vec<(String, String, bool, u64)>>;

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

    /// CPUAffinity property
    #[dbus_proxy(property, name = "CPUAffinity")]
    fn cpu_affinity(&self) -> zbus::Result<Vec<u8>>;

    /// CPUAffinityFromNUMA property
    #[dbus_proxy(property, name = "CPUAffinityFromNUMA")]
    fn cpu_affinity_from_numa(&self) -> zbus::Result<bool>;

    /// CPUQuotaPerSecUSec property
    #[dbus_proxy(property, name = "CPUQuotaPerSecUSec")]
    fn cpu_quota_per_sec_usec(&self) -> zbus::Result<u64>;

    /// CPUQuotaPeriodUSec property
    #[dbus_proxy(property, name = "CPUQuotaPeriodUSec")]
    fn cpu_quota_period_usec(&self) -> zbus::Result<u64>;

    /// CPUSchedulingPolicy property
    #[dbus_proxy(property, name = "CPUSchedulingPolicy")]
    fn cpu_scheduling_policy(&self) -> zbus::Result<i32>;

    /// CPUSchedulingPriority property
    #[dbus_proxy(property, name = "CPUSchedulingPriority")]
    fn cpu_scheduling_priority(&self) -> zbus::Result<i32>;

    /// CPUSchedulingResetOnFork property
    #[dbus_proxy(property, name = "CPUSchedulingResetOnFork")]
    fn cpu_scheduling_reset_on_fork(&self) -> zbus::Result<bool>;

    /// CPUShares property
    #[dbus_proxy(property, name = "CPUShares")]
    fn cpu_shares(&self) -> zbus::Result<u64>;

    /// CPUUsageNSec property
    #[dbus_proxy(property, name = "CPUUsageNSec")]
    fn cpu_usage_nsec(&self) -> zbus::Result<u64>;

    /// CPUWeight property
    #[dbus_proxy(property, name = "CPUWeight")]
    fn cpu_weight(&self) -> zbus::Result<u64>;

    /// CacheDirectory property
    #[dbus_proxy(property)]
    fn cache_directory(&self) -> zbus::Result<Vec<String>>;

    /// CacheDirectoryMode property
    #[dbus_proxy(property)]
    fn cache_directory_mode(&self) -> zbus::Result<u32>;

    /// CacheDirectorySymlink property
    #[dbus_proxy(property)]
    fn cache_directory_symlink(&self) -> zbus::Result<Vec<(String, String, u64)>>;

    /// CapabilityBoundingSet property
    #[dbus_proxy(property)]
    fn capability_bounding_set(&self) -> zbus::Result<u64>;

    /// ConfigurationDirectory property
    #[dbus_proxy(property)]
    fn configuration_directory(&self) -> zbus::Result<Vec<String>>;

    /// ConfigurationDirectoryMode property
    #[dbus_proxy(property)]
    fn configuration_directory_mode(&self) -> zbus::Result<u32>;

    /// ControlGroup property
    #[dbus_proxy(property)]
    fn control_group(&self) -> zbus::Result<String>;

    /// ControlGroupId property
    #[dbus_proxy(property)]
    fn control_group_id(&self) -> zbus::Result<u64>;

    /// ControlPID property
    #[dbus_proxy(property, name = "ControlPID")]
    fn control_pid(&self) -> zbus::Result<u32>;

    /// CoredumpFilter property
    #[dbus_proxy(property)]
    fn coredump_filter(&self) -> zbus::Result<u64>;

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

    /// DirectoryMode property
    #[dbus_proxy(property)]
    fn directory_mode(&self) -> zbus::Result<u32>;

    /// DisableControllers property
    #[dbus_proxy(property)]
    fn disable_controllers(&self) -> zbus::Result<Vec<String>>;

    /// DynamicUser property
    #[dbus_proxy(property)]
    fn dynamic_user(&self) -> zbus::Result<bool>;

    /// EffectiveCPUs property
    #[dbus_proxy(property, name = "EffectiveCPUs")]
    fn effective_cpus(&self) -> zbus::Result<Vec<u8>>;

    /// EffectiveMemoryNodes property
    #[dbus_proxy(property)]
    fn effective_memory_nodes(&self) -> zbus::Result<Vec<u8>>;

    /// Environment property
    #[dbus_proxy(property)]
    fn environment(&self) -> zbus::Result<Vec<String>>;

    /// EnvironmentFiles property
    #[dbus_proxy(property)]
    fn environment_files(&self) -> zbus::Result<Vec<(String, bool)>>;

    /// ExecMount property
    #[dbus_proxy(property)]
    fn exec_mount(
        &self,
    ) -> zbus::Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32)>>;

    /// ExecPaths property
    #[dbus_proxy(property)]
    fn exec_paths(&self) -> zbus::Result<Vec<String>>;

    /// ExecRemount property
    #[dbus_proxy(property)]
    fn exec_remount(
        &self,
    ) -> zbus::Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32)>>;

    /// ExecSearchPath property
    #[dbus_proxy(property)]
    fn exec_search_path(&self) -> zbus::Result<Vec<String>>;

    /// ExecUnmount property
    #[dbus_proxy(property)]
    fn exec_unmount(
        &self,
    ) -> zbus::Result<Vec<(String, Vec<String>, bool, u64, u64, u64, u64, u32, i32, i32)>>;

    /// ExtensionDirectories property
    #[dbus_proxy(property)]
    fn extension_directories(&self) -> zbus::Result<Vec<String>>;

    /// ExtensionImages property
    #[dbus_proxy(property)]
    fn extension_images(&self) -> zbus::Result<Vec<(String, bool, Vec<(String, String)>)>>;

    /// FinalKillSignal property
    #[dbus_proxy(property)]
    fn final_kill_signal(&self) -> zbus::Result<i32>;

    /// ForceUnmount property
    #[dbus_proxy(property)]
    fn force_unmount(&self) -> zbus::Result<bool>;

    /// GID property
    #[dbus_proxy(property, name = "GID")]
    fn gid(&self) -> zbus::Result<u32>;

    /// Group property
    #[dbus_proxy(property)]
    fn group(&self) -> zbus::Result<String>;

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

    /// IOSchedulingClass property
    #[dbus_proxy(property, name = "IOSchedulingClass")]
    fn io_scheduling_class(&self) -> zbus::Result<i32>;

    /// IOSchedulingPriority property
    #[dbus_proxy(property, name = "IOSchedulingPriority")]
    fn io_scheduling_priority(&self) -> zbus::Result<i32>;

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

    /// IPCNamespacePath property
    #[dbus_proxy(property, name = "IPCNamespacePath")]
    fn ipc_namespace_path(&self) -> zbus::Result<String>;

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
    fn ipingress_bytes(&self) -> zbus::Result<u64>;

    /// IPIngressFilterPath property
    #[dbus_proxy(property, name = "IPIngressFilterPath")]
    fn ip_ingress_filter_path(&self) -> zbus::Result<Vec<String>>;

    /// IPIngressPackets property
    #[dbus_proxy(property, name = "IPIngressPackets")]
    fn ip_ingress_packets(&self) -> zbus::Result<u64>;

    /// IgnoreSIGPIPE property
    #[dbus_proxy(property, name = "IgnoreSIGPIPE")]
    fn ignore_sigpipe(&self) -> zbus::Result<bool>;

    /// InaccessiblePaths property
    #[dbus_proxy(property)]
    fn inaccessible_paths(&self) -> zbus::Result<Vec<String>>;

    /// KeyringMode property
    #[dbus_proxy(property)]
    fn keyring_mode(&self) -> zbus::Result<String>;

    /// KillMode property
    #[dbus_proxy(property)]
    fn kill_mode(&self) -> zbus::Result<String>;

    /// KillSignal property
    #[dbus_proxy(property)]
    fn kill_signal(&self) -> zbus::Result<i32>;

    /// LazyUnmount property
    #[dbus_proxy(property)]
    fn lazy_unmount(&self) -> zbus::Result<bool>;

    /// LimitAS property
    #[dbus_proxy(property, name = "LimitAS")]
    fn limit_as(&self) -> zbus::Result<u64>;

    /// LimitASSoft property
    #[dbus_proxy(property, name = "LimitASSoft")]
    fn limit_as_soft(&self) -> zbus::Result<u64>;

    /// LimitCORE property
    #[dbus_proxy(property, name = "LimitCORE")]
    fn limit_core(&self) -> zbus::Result<u64>;

    /// LimitCORESoft property
    #[dbus_proxy(property, name = "LimitCORESoft")]
    fn limit_core_soft(&self) -> zbus::Result<u64>;

    /// LimitCPU property
    #[dbus_proxy(property, name = "LimitCPU")]
    fn limit_cpu(&self) -> zbus::Result<u64>;

    /// LimitCPUSoft property
    #[dbus_proxy(property, name = "LimitCPUSoft")]
    fn limit_cpu_soft(&self) -> zbus::Result<u64>;

    /// LimitDATA property
    #[dbus_proxy(property, name = "LimitDATA")]
    fn limit_data(&self) -> zbus::Result<u64>;

    /// LimitDATASoft property
    #[dbus_proxy(property, name = "LimitDATASoft")]
    fn limit_data_soft(&self) -> zbus::Result<u64>;

    /// LimitFSIZE property
    #[dbus_proxy(property, name = "LimitFSIZE")]
    fn limit_fsize(&self) -> zbus::Result<u64>;

    /// LimitFSIZESoft property
    #[dbus_proxy(property, name = "LimitFSIZESoft")]
    fn limit_fsize_soft(&self) -> zbus::Result<u64>;

    /// LimitLOCKS property
    #[dbus_proxy(property, name = "LimitLOCKS")]
    fn limit_locks(&self) -> zbus::Result<u64>;

    /// LimitLOCKSSoft property
    #[dbus_proxy(property, name = "LimitLOCKSSoft")]
    fn limit_locks_soft(&self) -> zbus::Result<u64>;

    /// LimitMEMLOCK property
    #[dbus_proxy(property, name = "LimitMEMLOCK")]
    fn limit_memlock(&self) -> zbus::Result<u64>;

    /// LimitMEMLOCKSoft property
    #[dbus_proxy(property, name = "LimitMEMLOCKSoft")]
    fn limit_memlock_soft(&self) -> zbus::Result<u64>;

    /// LimitMSGQUEUE property
    #[dbus_proxy(property, name = "LimitMSGQUEUE")]
    fn limit_msgqueue(&self) -> zbus::Result<u64>;

    /// LimitMSGQUEUESoft property
    #[dbus_proxy(property, name = "LimitMSGQUEUESoft")]
    fn limit_msgqueue_soft(&self) -> zbus::Result<u64>;

    /// LimitNICE property
    #[dbus_proxy(property, name = "LimitNICE")]
    fn limit_nice(&self) -> zbus::Result<u64>;

    /// LimitNICESoft property
    #[dbus_proxy(property, name = "LimitNICESoft")]
    fn limit_nice_soft(&self) -> zbus::Result<u64>;

    /// LimitNOFILE property
    #[dbus_proxy(property, name = "LimitNOFILE")]
    fn limit_nofile(&self) -> zbus::Result<u64>;

    /// LimitNOFILESoft property
    #[dbus_proxy(property, name = "LimitNOFILESoft")]
    fn limit_nofile_soft(&self) -> zbus::Result<u64>;

    /// LimitNPROC property
    #[dbus_proxy(property, name = "LimitNPROC")]
    fn limit_nproc(&self) -> zbus::Result<u64>;

    /// LimitNPROCSoft property
    #[dbus_proxy(property, name = "LimitNPROCSoft")]
    fn limit_nproc_soft(&self) -> zbus::Result<u64>;

    /// LimitRSS property
    #[dbus_proxy(property, name = "LimitRSS")]
    fn limit_rss(&self) -> zbus::Result<u64>;

    /// LimitRSSSoft property
    #[dbus_proxy(property, name = "LimitRSSSoft")]
    fn limit_rss_soft(&self) -> zbus::Result<u64>;

    /// LimitRTPRIO property
    #[dbus_proxy(property, name = "LimitRTPRIO")]
    fn limit_rtprio(&self) -> zbus::Result<u64>;

    /// LimitRTPRIOSoft property
    #[dbus_proxy(property, name = "LimitRTPRIOSoft")]
    fn limit_rtprio_soft(&self) -> zbus::Result<u64>;

    /// LimitRTTIME property
    #[dbus_proxy(property, name = "LimitRTTIME")]
    fn limit_rttime(&self) -> zbus::Result<u64>;

    /// LimitRTTIMESoft property
    #[dbus_proxy(property, name = "LimitRTTIMESoft")]
    fn limit_rttime_soft(&self) -> zbus::Result<u64>;

    /// LimitSIGPENDING property
    #[dbus_proxy(property, name = "LimitSIGPENDING")]
    fn limit_sigpending(&self) -> zbus::Result<u64>;

    /// LimitSIGPENDINGSoft property
    #[dbus_proxy(property, name = "LimitSIGPENDINGSoft")]
    fn limit_sigpending_soft(&self) -> zbus::Result<u64>;

    /// LimitSTACK property
    #[dbus_proxy(property, name = "LimitSTACK")]
    fn limit_stack(&self) -> zbus::Result<u64>;

    /// LimitSTACKSoft property
    #[dbus_proxy(property, name = "LimitSTACKSoft")]
    fn limit_stack_soft(&self) -> zbus::Result<u64>;

    /// LoadCredential property
    #[dbus_proxy(property)]
    fn load_credential(&self) -> zbus::Result<Vec<(String, String)>>;

    /// LoadCredentialEncrypted property
    #[dbus_proxy(property)]
    fn load_credential_encrypted(&self) -> zbus::Result<Vec<(String, String)>>;

    /// LockPersonality property
    #[dbus_proxy(property)]
    fn lock_personality(&self) -> zbus::Result<bool>;

    /// LogExtraFields property
    #[dbus_proxy(property)]
    fn log_extra_fields(&self) -> zbus::Result<Vec<Vec<u8>>>;

    /// LogLevelMax property
    #[dbus_proxy(property)]
    fn log_level_max(&self) -> zbus::Result<i32>;

    /// LogNamespace property
    #[dbus_proxy(property)]
    fn log_namespace(&self) -> zbus::Result<String>;

    /// LogRateLimitBurst property
    #[dbus_proxy(property)]
    fn log_rate_limit_burst(&self) -> zbus::Result<u32>;

    /// LogRateLimitIntervalUSec property
    #[dbus_proxy(property, name = "LogRateLimitIntervalUSec")]
    fn log_rate_limit_interval_usec(&self) -> zbus::Result<u64>;

    /// LogsDirectory property
    #[dbus_proxy(property)]
    fn logs_directory(&self) -> zbus::Result<Vec<String>>;

    /// LogsDirectoryMode property
    #[dbus_proxy(property)]
    fn logs_directory_mode(&self) -> zbus::Result<u32>;

    /// LogsDirectorySymlink property
    #[dbus_proxy(property)]
    fn logs_directory_symlink(&self) -> zbus::Result<Vec<(String, String, u64)>>;

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

    /// MemoryDenyWriteExecute property
    #[dbus_proxy(property)]
    fn memory_deny_write_execute(&self) -> zbus::Result<bool>;

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

    /// MountAPIVFS property
    #[dbus_proxy(property, name = "MountAPIVFS")]
    fn mount_apivfs(&self) -> zbus::Result<bool>;

    /// MountFlags property
    #[dbus_proxy(property)]
    fn mount_flags(&self) -> zbus::Result<u64>;

    /// MountImages property
    #[dbus_proxy(property)]
    fn mount_images(&self) -> zbus::Result<Vec<(String, String, bool, Vec<(String, String)>)>>;

    /// NUMAMask property
    #[dbus_proxy(property, name = "NUMAMask")]
    fn numa_mask(&self) -> zbus::Result<Vec<u8>>;

    /// NUMAPolicy property
    #[dbus_proxy(property, name = "NUMAPolicy")]
    fn numa_policy(&self) -> zbus::Result<i32>;

    /// NetworkNamespacePath property
    #[dbus_proxy(property)]
    fn network_namespace_path(&self) -> zbus::Result<String>;

    /// Nice property
    #[dbus_proxy(property)]
    fn nice(&self) -> zbus::Result<i32>;

    /// NoExecPaths property
    #[dbus_proxy(property)]
    fn no_exec_paths(&self) -> zbus::Result<Vec<String>>;

    /// NoNewPrivileges property
    #[dbus_proxy(property)]
    fn no_new_privileges(&self) -> zbus::Result<bool>;

    /// NonBlocking property
    #[dbus_proxy(property)]
    fn non_blocking(&self) -> zbus::Result<bool>;

    /// OOMScoreAdjust property
    #[dbus_proxy(property, name = "OOMScoreAdjust")]
    fn oom_score_adjust(&self) -> zbus::Result<i32>;

    /// Options property
    #[dbus_proxy(property)]
    fn options(&self) -> zbus::Result<String>;

    /// PAMName property
    #[dbus_proxy(property, name = "PAMName")]
    fn pam_name(&self) -> zbus::Result<String>;

    /// PassEnvironment property
    #[dbus_proxy(property)]
    fn pass_environment(&self) -> zbus::Result<Vec<String>>;

    /// Personality property
    #[dbus_proxy(property)]
    fn personality(&self) -> zbus::Result<String>;

    /// PrivateDevices property
    #[dbus_proxy(property)]
    fn private_devices(&self) -> zbus::Result<bool>;

    /// PrivateIPC property
    #[dbus_proxy(property, name = "PrivateIPC")]
    fn private_ipc(&self) -> zbus::Result<bool>;

    /// PrivateMounts property
    #[dbus_proxy(property)]
    fn private_mounts(&self) -> zbus::Result<bool>;

    /// PrivateNetwork property
    #[dbus_proxy(property)]
    fn private_network(&self) -> zbus::Result<bool>;

    /// PrivateTmp property
    #[dbus_proxy(property)]
    fn private_tmp(&self) -> zbus::Result<bool>;

    /// PrivateUsers property
    #[dbus_proxy(property)]
    fn private_users(&self) -> zbus::Result<bool>;

    /// ProcSubset property
    #[dbus_proxy(property)]
    fn proc_subset(&self) -> zbus::Result<String>;

    /// ProtectClock property
    #[dbus_proxy(property)]
    fn protect_clock(&self) -> zbus::Result<bool>;

    /// ProtectControlGroups property
    #[dbus_proxy(property)]
    fn protect_control_groups(&self) -> zbus::Result<bool>;

    /// ProtectHome property
    #[dbus_proxy(property)]
    fn protect_home(&self) -> zbus::Result<String>;

    /// ProtectHostname property
    #[dbus_proxy(property)]
    fn protect_hostname(&self) -> zbus::Result<bool>;

    /// ProtectKernelLogs property
    #[dbus_proxy(property)]
    fn protect_kernel_logs(&self) -> zbus::Result<bool>;

    /// ProtectKernelModules property
    #[dbus_proxy(property)]
    fn protect_kernel_modules(&self) -> zbus::Result<bool>;

    /// ProtectKernelTunables property
    #[dbus_proxy(property)]
    fn protect_kernel_tunables(&self) -> zbus::Result<bool>;

    /// ProtectProc property
    #[dbus_proxy(property)]
    fn protect_proc(&self) -> zbus::Result<String>;

    /// ProtectSystem property
    #[dbus_proxy(property)]
    fn protect_system(&self) -> zbus::Result<String>;

    /// ReadOnlyPaths property
    #[dbus_proxy(property)]
    fn read_only_paths(&self) -> zbus::Result<Vec<String>>;

    /// ReadWriteOnly property
    #[dbus_proxy(property)]
    fn read_write_only(&self) -> zbus::Result<bool>;

    /// ReadWritePaths property
    #[dbus_proxy(property)]
    fn read_write_paths(&self) -> zbus::Result<Vec<String>>;

    /// RemoveIPC property
    #[dbus_proxy(property, name = "RemoveIPC")]
    fn remove_ipc(&self) -> zbus::Result<bool>;

    /// RestartKillSignal property
    #[dbus_proxy(property)]
    fn restart_kill_signal(&self) -> zbus::Result<i32>;

    /// RestrictAddressFamilies property
    #[dbus_proxy(property)]
    fn restrict_address_families(&self) -> zbus::Result<(bool, Vec<String>)>;

    /// RestrictFileSystems property
    #[dbus_proxy(property)]
    fn restrict_file_systems(&self) -> zbus::Result<(bool, Vec<String>)>;

    /// RestrictNamespaces property
    #[dbus_proxy(property)]
    fn restrict_namespaces(&self) -> zbus::Result<u64>;

    /// RestrictNetworkInterfaces property
    #[dbus_proxy(property)]
    fn restrict_network_interfaces(&self) -> zbus::Result<(bool, Vec<String>)>;

    /// RestrictRealtime property
    #[dbus_proxy(property)]
    fn restrict_realtime(&self) -> zbus::Result<bool>;

    /// RestrictSUIDSGID property
    #[dbus_proxy(property, name = "RestrictSUIDSGID")]
    fn restrict_suidsgid(&self) -> zbus::Result<bool>;

    /// Result property
    #[dbus_proxy(property)]
    fn result(&self) -> zbus::Result<String>;

    /// RootDirectory property
    #[dbus_proxy(property)]
    fn root_directory(&self) -> zbus::Result<String>;

    /// RootHash property
    #[dbus_proxy(property)]
    fn root_hash(&self) -> zbus::Result<Vec<u8>>;

    /// RootHashPath property
    #[dbus_proxy(property)]
    fn root_hash_path(&self) -> zbus::Result<String>;

    /// RootHashSignature property
    #[dbus_proxy(property)]
    fn root_hash_signature(&self) -> zbus::Result<Vec<u8>>;

    /// RootHashSignaturePath property
    #[dbus_proxy(property)]
    fn root_hash_signature_path(&self) -> zbus::Result<String>;

    /// RootImage property
    #[dbus_proxy(property)]
    fn root_image(&self) -> zbus::Result<String>;

    /// RootImageOptions property
    #[dbus_proxy(property)]
    fn root_image_options(&self) -> zbus::Result<Vec<(String, String)>>;

    /// RootVerity property
    #[dbus_proxy(property)]
    fn root_verity(&self) -> zbus::Result<String>;

    /// RuntimeDirectory property
    #[dbus_proxy(property)]
    fn runtime_directory(&self) -> zbus::Result<Vec<String>>;

    /// RuntimeDirectoryMode property
    #[dbus_proxy(property)]
    fn runtime_directory_mode(&self) -> zbus::Result<u32>;

    /// RuntimeDirectoryPreserve property
    #[dbus_proxy(property)]
    fn runtime_directory_preserve(&self) -> zbus::Result<String>;

    /// RuntimeDirectorySymlink property
    #[dbus_proxy(property)]
    fn runtime_directory_symlink(&self) -> zbus::Result<Vec<(String, String, u64)>>;

    /// SELinuxContext property
    #[dbus_proxy(property, name = "SELinuxContext")]
    fn selinux_context(&self) -> zbus::Result<(bool, String)>;

    /// SameProcessGroup property
    #[dbus_proxy(property)]
    fn same_process_group(&self) -> zbus::Result<bool>;

    /// SecureBits property
    #[dbus_proxy(property)]
    fn secure_bits(&self) -> zbus::Result<i32>;

    /// SendSIGHUP property
    #[dbus_proxy(property, name = "SendSIGHUP")]
    fn send_sighup(&self) -> zbus::Result<bool>;

    /// SendSIGKILL property
    #[dbus_proxy(property, name = "SendSIGKILL")]
    fn send_sigkill(&self) -> zbus::Result<bool>;

    /// SetCredential property
    #[dbus_proxy(property)]
    fn set_credential(&self) -> zbus::Result<Vec<(String, Vec<u8>)>>;

    /// SetCredentialEncrypted property
    #[dbus_proxy(property)]
    fn set_credential_encrypted(&self) -> zbus::Result<Vec<(String, Vec<u8>)>>;

    /// Slice property
    #[dbus_proxy(property)]
    fn slice(&self) -> zbus::Result<String>;

    /// SloppyOptions property
    #[dbus_proxy(property)]
    fn sloppy_options(&self) -> zbus::Result<bool>;

    /// SmackProcessLabel property
    #[dbus_proxy(property)]
    fn smack_process_label(&self) -> zbus::Result<(bool, String)>;

    /// SocketBindAllow property
    #[dbus_proxy(property)]
    fn socket_bind_allow(&self) -> zbus::Result<Vec<(i32, i32, u16, u16)>>;

    /// SocketBindDeny property
    #[dbus_proxy(property)]
    fn socket_bind_deny(&self) -> zbus::Result<Vec<(i32, i32, u16, u16)>>;

    /// StandardError property
    #[dbus_proxy(property)]
    fn standard_error(&self) -> zbus::Result<String>;

    /// StandardErrorFileDescriptorName property
    #[dbus_proxy(property)]
    fn standard_error_file_descriptor_name(&self) -> zbus::Result<String>;

    /// StandardInput property
    #[dbus_proxy(property)]
    fn standard_input(&self) -> zbus::Result<String>;

    /// StandardInputData property
    #[dbus_proxy(property)]
    fn standard_input_data(&self) -> zbus::Result<Vec<u8>>;

    /// StandardInputFileDescriptorName property
    #[dbus_proxy(property)]
    fn standard_input_file_descriptor_name(&self) -> zbus::Result<String>;

    /// StandardOutput property
    #[dbus_proxy(property)]
    fn standard_output(&self) -> zbus::Result<String>;

    /// StandardOutputFileDescriptorName property
    #[dbus_proxy(property)]
    fn standard_output_file_descriptor_name(&self) -> zbus::Result<String>;

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

    /// StateDirectory property
    #[dbus_proxy(property)]
    fn state_directory(&self) -> zbus::Result<Vec<String>>;

    /// StateDirectoryMode property
    #[dbus_proxy(property)]
    fn state_directory_mode(&self) -> zbus::Result<u32>;

    /// StateDirectorySymlink property
    #[dbus_proxy(property)]
    fn state_directory_symlink(&self) -> zbus::Result<Vec<(String, String, u64)>>;

    /// SupplementaryGroups property
    #[dbus_proxy(property)]
    fn supplementary_groups(&self) -> zbus::Result<Vec<String>>;

    /// SyslogFacility property
    #[dbus_proxy(property)]
    fn syslog_facility(&self) -> zbus::Result<i32>;

    /// SyslogIdentifier property
    #[dbus_proxy(property)]
    fn syslog_identifier(&self) -> zbus::Result<String>;

    /// SyslogLevel property
    #[dbus_proxy(property)]
    fn syslog_level(&self) -> zbus::Result<i32>;

    /// SyslogLevelPrefix property
    #[dbus_proxy(property)]
    fn syslog_level_prefix(&self) -> zbus::Result<bool>;

    /// SyslogPriority property
    #[dbus_proxy(property)]
    fn syslog_priority(&self) -> zbus::Result<i32>;

    /// SystemCallArchitectures property
    #[dbus_proxy(property)]
    fn system_call_architectures(&self) -> zbus::Result<Vec<String>>;

    /// SystemCallErrorNumber property
    #[dbus_proxy(property)]
    fn system_call_error_number(&self) -> zbus::Result<i32>;

    /// SystemCallFilter property
    #[dbus_proxy(property)]
    fn system_call_filter(&self) -> zbus::Result<(bool, Vec<String>)>;

    /// SystemCallLog property
    #[dbus_proxy(property)]
    fn system_call_log(&self) -> zbus::Result<(bool, Vec<String>)>;

    /// TTYColumns property
    #[dbus_proxy(property, name = "TTYColumns")]
    fn tty_columns(&self) -> zbus::Result<u16>;

    /// TTYPath property
    #[dbus_proxy(property, name = "TTYPath")]
    fn tty_path(&self) -> zbus::Result<String>;

    /// TTYReset property
    #[dbus_proxy(property, name = "TTYReset")]
    fn tty_reset(&self) -> zbus::Result<bool>;

    /// TTYRows property
    #[dbus_proxy(property, name = "TTYRows")]
    fn tty_rows(&self) -> zbus::Result<u16>;

    /// TTYVHangup property
    #[dbus_proxy(property, name = "TTYVHangup")]
    fn tty_vhangup(&self) -> zbus::Result<bool>;

    /// TTYVTDisallocate property
    #[dbus_proxy(property, name = "TTYVTDisallocate")]
    fn tty_vt_disallocate(&self) -> zbus::Result<bool>;

    /// TasksAccounting property
    #[dbus_proxy(property)]
    fn tasks_accounting(&self) -> zbus::Result<bool>;

    /// TasksCurrent property
    #[dbus_proxy(property)]
    fn tasks_current(&self) -> zbus::Result<u64>;

    /// TasksMax property
    #[dbus_proxy(property)]
    fn tasks_max(&self) -> zbus::Result<u64>;

    /// TemporaryFileSystem property
    #[dbus_proxy(property)]
    fn temporary_file_system(&self) -> zbus::Result<Vec<(String, String)>>;

    /// TimeoutCleanUSec property
    #[dbus_proxy(property, name = "TimeoutCleanUSec")]
    fn timeout_clean_usec(&self) -> zbus::Result<u64>;

    /// TimeoutUSec property
    #[dbus_proxy(property, name = "TimeoutUSec")]
    fn timeout_usec(&self) -> zbus::Result<u64>;

    /// TimerSlackNSec property
    #[dbus_proxy(property, name = "TimerSlackNSec")]
    fn timer_slack_nsec(&self) -> zbus::Result<u64>;

    /// Type property
    #[dbus_proxy(property)]
    fn kind(&self) -> zbus::Result<String>;

    /// UID property
    #[dbus_proxy(property, name = "UID")]
    fn uid(&self) -> zbus::Result<u32>;

    /// UMask property
    #[dbus_proxy(property, name = "UMask")]
    fn umask(&self) -> zbus::Result<u32>;

    /// UnsetEnvironment property
    #[dbus_proxy(property)]
    fn unset_environment(&self) -> zbus::Result<Vec<String>>;

    /// User property
    #[dbus_proxy(property)]
    fn user(&self) -> zbus::Result<String>;

    /// UtmpIdentifier property
    #[dbus_proxy(property)]
    fn utmp_identifier(&self) -> zbus::Result<String>;

    /// UtmpMode property
    #[dbus_proxy(property)]
    fn utmp_mode(&self) -> zbus::Result<String>;

    /// WatchdogSignal property
    #[dbus_proxy(property)]
    fn watchdog_signal(&self) -> zbus::Result<i32>;

    /// What property
    #[dbus_proxy(property)]
    fn what(&self) -> zbus::Result<String>;

    /// Where property
    #[dbus_proxy(property)]
    fn where_path(&self) -> zbus::Result<String>;

    /// WorkingDirectory property
    #[dbus_proxy(property)]
    fn working_directory(&self) -> zbus::Result<String>;
}
