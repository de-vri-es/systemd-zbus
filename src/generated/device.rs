//! # DBus interface proxies for: `org.freedesktop.systemd1.Device`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `org.freedesktop.systemd1.Device.xml`.
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
    interface = "org.freedesktop.systemd1.Device",
    default_service = "org.freedesktop.systemd1"
)]
trait Device {
    /// SysFSPath property
    #[dbus_proxy(property, name = "SysFSPath")]
    fn sysfs_path(&self) -> zbus::Result<String>;
}