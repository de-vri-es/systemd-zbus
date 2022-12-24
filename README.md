# systemd-zbus

A wrapper around the dbus interfaces provided by `systemd`.

`systemd-zbus` aims to provide a convenient API abstraction of the dbus interface
of systemd in rust, where possible parsing responses to concrete structs and enums.

This is a WIP, but entirely usable as it is.

PR's are welcome to help speed progress along.

Documentation is a little lacking but referring to https://www.freedesktop.org/software/systemd/man/systemctl.html can provide good insight as the API is generated directly from systemd dbus interface using `zbus-xmlgen`.
