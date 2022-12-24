# systemd-zbus

A wrapper around the dbus interfaces provided by `systemd`.

`systemd-zbus` aims to provide a convenient API abstraction of the dbus interface
of systemd in rust, where possible parsing responses to concrete structs and enums.

## Notes

This is a WIP, but entirely usable as it is. The generated methods from `zbus-xmlgen` all will work fine. The majority of these are untouched except for where the I/O of the API can benefit from some
structs or enums to clarify and tidy things up.

PR's are welcome to help speed progress along. In reality there shouldn't be much to do.

Tests are lacking due to: most requiring root, and most generated methods should work without
modification. Where the I/O of API was changed with struct or enums some tests were written to
verify stuff.

Documentation is a little lacking but referring to https://www.freedesktop.org/software/systemd/man/systemctl.html can provide good insight as the API is generated directly from systemd dbus interface using `zbus-xmlgen`.
