use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedObjectPath, Type};

use crate::{enum_impl_serde_str, enum_impl_str_conv, impl_try_from_owned_as_str};

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
impl_try_from_owned_as_str!(UnitType);

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
impl_try_from_owned_as_str!(Mode);

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
impl_try_from_owned_as_str!(LoadState);

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
impl_try_from_owned_as_str!(ActiveState);

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
impl_try_from_owned_as_str!(UnitFileState);

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
impl_try_from_owned_as_str!(SubState);

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
impl_try_from_owned_as_str!(ChangeType);

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct Change {
    change_type: ChangeType,
    /// Filename of the symlink
    symlink_file: String,
    /// Destination of the symlink
    symlink_dest: String,
}
