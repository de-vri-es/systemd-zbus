//! # DBus interface proxies for: `org.freedesktop.systemd1.Timer`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `org.freedesktop.systemd1.Timer.xml`.
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
    interface = "org.freedesktop.systemd1.Timer",
    default_service = "org.freedesktop.systemd1"
)]
trait Timer {
    /// AccuracyUSec property
    #[dbus_proxy(property, name = "AccuracyUSec")]
    fn accuracy_usec(&self) -> zbus::Result<u64>;

    /// FixedRandomDelay property
    #[dbus_proxy(property)]
    fn fixed_random_delay(&self) -> zbus::Result<bool>;

    /// LastTriggerUSec property
    #[dbus_proxy(property, name = "LastTriggerUSec")]
    fn last_trigger_usec(&self) -> zbus::Result<u64>;

    /// LastTriggerUSecMonotonic property
    #[dbus_proxy(property, name = "LastTriggerUSecMonotonic")]
    fn last_trigger_usec_monotonic(&self) -> zbus::Result<u64>;

    /// NextElapseUSecMonotonic property
    #[dbus_proxy(property, name = "NextElapseUSecMonotonic")]
    fn next_elapse_usec_monotonic(&self) -> zbus::Result<u64>;

    /// NextElapseUSecRealtime property
    #[dbus_proxy(property, name = "NextElapseUSecRealtime")]
    fn next_elapse_usec_realtime(&self) -> zbus::Result<u64>;

    /// OnClockChange property
    #[dbus_proxy(property)]
    fn on_clock_change(&self) -> zbus::Result<bool>;

    /// OnTimezoneChange property
    #[dbus_proxy(property)]
    fn on_timezone_change(&self) -> zbus::Result<bool>;

    /// Persistent property
    #[dbus_proxy(property)]
    fn persistent(&self) -> zbus::Result<bool>;

    /// RandomizedDelayUSec property
    #[dbus_proxy(property, name = "RandomizedDelayUSec")]
    fn randomized_delay_usec(&self) -> zbus::Result<u64>;

    /// RemainAfterElapse property
    #[dbus_proxy(property)]
    fn remain_after_elapse(&self) -> zbus::Result<bool>;

    /// Result property
    #[dbus_proxy(property)]
    fn result(&self) -> zbus::Result<String>;

    /// TimersCalendar property
    #[dbus_proxy(property)]
    fn timers_calendar(&self) -> zbus::Result<Vec<(String, String, u64)>>;

    /// TimersMonotonic property
    #[dbus_proxy(property)]
    fn timers_monotonic(&self) -> zbus::Result<Vec<(String, u64, u64)>>;

    /// Unit property
    #[dbus_proxy(property)]
    fn unit(&self) -> zbus::Result<String>;

    /// WakeSystem property
    #[dbus_proxy(property)]
    fn wake_system(&self) -> zbus::Result<bool>;
}
