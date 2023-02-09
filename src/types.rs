use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Type, Value};

use crate::{enum_impl_serde_str, enum_impl_str_conv, impl_value_conversions_as_str};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
pub enum UnitType {
    Service,
    Mount,
    Swap,
    Socket,
    Target,
    Device,
    Automount,
    Timer,
    Path,
    Slice,
    Scope,
}

enum_impl_str_conv!(UnitType, {
    "service": Service,
    "mount": Mount,
    "swap": Swap,
    "socket": Socket,
    "target": Target,
    "device": Device,
    "automount": Automount,
    "timer": Timer,
    "path": Path,
    "slice": Slice,
    "scope": Scope,
});
enum_impl_serde_str!(UnitType);
impl_value_conversions_as_str!(UnitType);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Serialize, Deserialize)]
pub enum UnitFileFlags {
    /// Will enable or disable the unit for runtime only.
    ///
    /// Defined as `#define SD_SYSTEMD_UNIT_RUNTIME  (UINT64_C(1) << 0)`
    Runtime,
    /// Controls whether symlinks pointing to other units shall be replaced if necessary.
    ///
    /// Defined as `#define SD_SYSTEMD_UNIT_FORCE    (UINT64_C(1) << 1)`
    Force,
    /// Will add or remove the symlinks in `/etc/systemd/system.attached` and `/run/systemd/system.attached`.
    ///
    /// Defined as `#define SD_SYSTEMD_UNIT_PORTABLE (UINT64_C(1) << 2)`
    Portable,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum Mode {
    Replace,
    Fail,
    Isolate,
    IgnoreDependencies,
    IgnoreRequirements,
}

enum_impl_str_conv!(Mode, {
    "replace": Replace,
    "fail": Fail,
    "isolate": Isolate,
    "ignore-dependencies": IgnoreDependencies,
    "ignore-requirements": IgnoreRequirements,
});
enum_impl_serde_str!(Mode);
impl_value_conversions_as_str!(Mode);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum LoadState {
    Stub,
    Loaded,
    NotFound,
    BadSetting,
    Error,
    Merged,
    Masked,
}

enum_impl_str_conv!(LoadState, {
    "stub": Stub,
    "loaded": Loaded,
    "not-found": NotFound,
    "bad-setting": BadSetting,
    "error": Error,
    "merged": Merged,
    "masked": Masked,
});
enum_impl_serde_str!(LoadState);
impl_value_conversions_as_str!(LoadState);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum ActiveState {
    Active,
    Reloading,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Maintenance,
}

enum_impl_str_conv!(ActiveState, {
    "active": Active,
    "reloading": Reloading,
    "inactive": Inactive,
    "failed": Failed,
    "activating": Activating,
    "deactivating": Deactivating,
    "maintenance" : Maintenance,
});
enum_impl_serde_str!(ActiveState);
impl_value_conversions_as_str!(ActiveState);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum UnitFileState {
    Enabled,
    EnabledRuntime,
    Linked,
    LinkedRuntime,
    Masked,
    MaskedRuntime,
    Static,
    Disabled,
    Invalid,
}

enum_impl_str_conv!(UnitFileState, {
    "enabled": Enabled,
    "enabled-runtime": EnabledRuntime,
    "linked":  Linked,
    "linked-runtime":  LinkedRuntime,
    "masked":  Masked,
    "masked-runtime":  MaskedRuntime,
    "static": Static,
    "disabled": Disabled,
    "invalid":  Invalid,

});
enum_impl_serde_str!(UnitFileState);
impl_value_conversions_as_str!(UnitFileState);

/// The `SubState` of a unit is specific to the unit type, it is best to run
/// ```ignore
/// systemctl --state=help
/// ```
/// to get a list of substate groups. Many states are in multiple groups.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum SubState {
    Dead,
    Waiting,
    Running,
    Failed,
    Tentative,
    Plugged,
    Mounting,
    MountingDone,
    Mounted,
    Remounting,
    Unmounting,
    RemountingSigterm,
    RemountingSigkill,
    UnmountingSigterm,
    UnmountingSigkill,
    Cleaning,
    Abandoned,
    StopSigterm,
    StopSigkill,
    Condition,
    StartPre,
    Start,
    StartPost,
    Exited,
    Reload,
    Stop,
    StopWatchdog,
    StopPost,
    FinalWatchdog,
    FinalSigterm,
    FinalSigkill,
    AutoRestart,
    Active,
    StartChown,
    Listening,
    StopPre,
    StopPreSigterm,
    StopPreSigkill,
    Activating,
    ActivatingDone,
    Deactivating,
    DeactivatingSigterm,
    DeactivatingSigkill,
    Elapsed,
}

enum_impl_str_conv!(SubState, {
    "dead": Dead,
    "waiting": Waiting,
    "running": Running,
    "failed": Failed,
    "tentative": Tentative,
    "plugged": Plugged,
    "mounting": Mounting,
    "mounting-done": MountingDone,
    "mounted": Mounted,
    "remounting": Remounting,
    "unmounting": Unmounting,
    "remounting-sigterm": RemountingSigterm,
    "remounting-sigkill": RemountingSigkill,
    "unmounting-sigterm": UnmountingSigterm,
    "unmounting-sigkill": UnmountingSigkill,
    "cleaning": Cleaning,
    "adandoned": Abandoned,
    "stop-sigterm": StopSigterm,
    "stop-sigkill": StopSigkill,
    "condition": Condition,
    "start-pre": StartPre,
    "start": Start,
    "start-post": StartPost,
    "exited": Exited,
    "reload": Reload,
    "stop": Stop,
    "stop-watchdog": StopWatchdog,
    "stop-post": StopPost,
    "final-watchdog": FinalWatchdog,
    "final-sigterm": FinalSigterm,
    "final-sigkill": FinalSigkill,
    "auto-restart": AutoRestart,
    "active": Active,
    "start-chown": StartChown,
    "listening": Listening,
    "stop-pre": StopPre,
    "stop-pre-sigterm": StopPreSigterm,
    "stop-pre-sigkill": StopPreSigkill,
    "activating": Activating,
    "activatin-done": ActivatingDone,
    "deactivating": Deactivating,
    "deactivating-sigterm": DeactivatingSigterm,
    "deactivating-sigkill": DeactivatingSigkill,
    "elapsed": Elapsed,
});
enum_impl_serde_str!(SubState);
impl_value_conversions_as_str!(SubState);

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct Unit {
    /// The primary name
    pub name: String,
    /// The human readable description string
    pub description: String,
    /// The load state (i.e. whether the unit file has been loaded successfully)
    pub load: LoadState,
    /// The active state (i.e. whether the unit is currently started or not)
    pub active: ActiveState,
    /// The sub state (a more fine-grained version of the active state that is specific to the unit type, which the active state is not)
    pub sub_state: SubState,
    /// A unit that is being followed in its state by this unit, if there is any, otherwise the empty string.
    pub followed_unit: String,
    /// The unit object path
    pub path: OwnedObjectPath,
    /// If there is a job queued for the job unit, the numeric job id, 0 otherwise
    pub queued_job: u32,
    /// The job type as string
    pub job_type: String,
    /// The job object path
    pub job_path: OwnedObjectPath,
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct Job {
    /// The numeric job id
    pub job_id: u32,
    /// The primary unit name for this job
    pub unit_name: String,
    /// The job type
    pub job_type: String,
    /// The job state
    pub job_state: String,
    /// The job object path
    pub job_path: OwnedObjectPath,
    /// The job unit path
    pub unit_path: OwnedObjectPath,
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct EnquedJob {
    /// The numeric job id
    pub job_id: u32,
    /// The job object path
    pub job_path: OwnedObjectPath,
    /// The primary unit name for this job
    pub unit_name: String,
    /// The job unit path
    pub unit_path: OwnedObjectPath,
    /// The job type
    pub job_type: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct EnqueJob {
    pub job: EnquedJob,
    pub affected_jobs: Vec<EnquedJob>,
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Value, OwnedValue)]
pub struct KeyValue<K, V>
where
    K: Type,
    V: Type,
{
    /// The key.
    pub key: K,

    /// The value.
    pub value: V,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum ChangeType {
    Symlink,
    Unlink,
}

enum_impl_str_conv!(ChangeType, {
    "symlink": Symlink,
    "unlink": Unlink,
});
enum_impl_serde_str!(ChangeType);
impl_value_conversions_as_str!(ChangeType);

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct Change {
    change_type: ChangeType,
    /// Filename of the symlink
    symlink_file: String,
    /// Destination of the symlink
    symlink_dest: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum PathWatchCondition {
    Exists,
    ExistsGlob,
    Changed,
    Modified,
    DirectoryNotEmpty,
}
enum_impl_str_conv!(PathWatchCondition, {
    "PathExists": Exists,
    "PathExistsGlob": ExistsGlob,
    "PathChanged": Changed,
    "PathModified": Modified,
    "PathDirectoryNotEmpty": DirectoryNotEmpty,
});
enum_impl_serde_str!(PathWatchCondition);
impl_value_conversions_as_str!(PathWatchCondition);

/// A path watch specification for a Path unit.
#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Value, OwnedValue)]
pub struct PathWatch {
    /// The condition to watch for.
    pub condition: PathWatchCondition,

    /// The path to watch.
    ///
    /// If the condition is [`ExistsGlob`][PathWatchCondition::ExistsGlob],
    /// the path is a glob pattern.
    pub path: String,
}

/// An exec command, augmented with runtime data.
#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Value, OwnedValue)]
pub struct Exec {
    /// The binary to run.
    pub binary_path: String,

    /// The arguments to the binary, including argv[0].
    pub arguments: Vec<String>,

    /// If true, it is considered a failure if the process exits uncleanly.
    pub is_failure: bool,

    /// The last start time of the process in microseconds on the realtime clock, or 0 if it was never started yet.
    pub last_start_realtime_us: u64,

    /// The last start time of the process in microseconds on the monotonic clock, or 0 if it was never started yet.
    pub last_start_monotonic_us: u64,

    /// The last finish time of the process in microseconds on the realtime clock, or 0 if it never finished yet.
    pub last_finish_realtime_us: u64,

    /// The last finish time of the process in microseconds on the monotonic clock, or 0 if it never finished yet.
    pub last_finish_monotonic_us: u64,

    /// The PID of the process, or 0 if it was never started yet.
    pub pid: u32,

    /// The last exit code of the process.
    pub last_exit_code: i32,

    /// The last status of the process.
    pub last_status: i32,
}

/// A realtime calendar specification.
#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Value, OwnedValue)]
pub struct TimerCalendar {
    /// The timer base.
    pub base: TimerCalendarBase,

    /// The calendar specification string.
    pub calendar_spec: String,

    /// The next elapsation point on the realtime clock.
    pub next_elapse_realtime_usec: u64,
}

/// A calender timer base.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum TimerCalendarBase {
    OnCalendar,
}
enum_impl_str_conv!(TimerCalendarBase, {
    "OnCalendar": OnCalendar,
});
enum_impl_serde_str!(TimerCalendarBase);
impl_value_conversions_as_str!(TimerCalendarBase);

/// A realtime calendar specification.
#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Value, OwnedValue)]
pub struct TimerMonotonic {
    /// The timer base.
    pub base: TimerMonotonicBase,

    /// The timer offset relative to the timer base.
    pub offset_usec: u64,

    /// The next elapsation point on the realtime clock.
    pub next_elapse_realtime_usec: u64,
}

/// A monotonic timer base.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum TimerMonotonicBase {
    OnActiveUSec,
    OnBootUSec,
    OnStartupUSec,
    OnUnitActiveUSec,
    OnUnitInactiveUSec,
}
enum_impl_str_conv!(TimerMonotonicBase, {
    "OnActiveUSec": OnActiveUSec,
    "OnBootUSec": OnBootUSec,
    "OnStartupUSec": OnStartupUSec,
    "OnUnitActiveUSec": OnUnitActiveUSec,
    "OnUnitInactiveUSec": OnUnitInactiveUSec,
});
enum_impl_serde_str!(TimerMonotonicBase);
impl_value_conversions_as_str!(TimerMonotonicBase);

/// The bind mount is recursive.
pub const BIND_MOUNT_RECURSIVE: u64 = 0x4000;

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Value, OwnedValue)]
pub struct BindMount {
    /// The source folder of the bind mount.
    pub source: String,

    /// The destination folder of the bind mount.
    pub destination: String,

    /// Ignore the bind mount if the source folder does not exist.
    pub ignore_non_existing: bool,

    /// Additional options for the bind mount as bitmask.
    ///
    /// Currently only the [`BIND_MOUNT_RECURSIVE`] option exists.
    pub options: u64,
}

/// A process spawned by systemd for a unit.
#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Value, OwnedValue)]
pub struct Process {
    /// The cgroup controller of the process.
    pub cgroup_controller: String,

    /// The PID of the process.
    pub pid: u32,

    /// The command line of the process.
    pub command_line: String,
}

/// A symbolic link created for a runtime,state, cache or log directort directory.
#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Value, OwnedValue)]
pub struct DirectorySymlink {
    /// The path of the symbolic link target.
    pub target_path: String,

    /// The path of the symbolic link.
    pub symlink_path: String,

    /// Flags, currently unused.
    pub flags: u64,
}
