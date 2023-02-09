//! Upstream docs - <https://www.freedesktop.org/software/systemd/man/org.freedesktop.systemd1.html>

mod generated;
pub use generated::*;

mod types;
pub use types::*;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! enum_impl_serde_str {
    ($type_name:ident) => {
        impl serde::Serialize for $type_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.into())
            }
        }

        impl<'de> serde::Deserialize<'de> for $type_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use std::str::FromStr;
                let s = String::deserialize(deserializer)?;
                $type_name::from_str(s.as_str()).map_err(serde::de::Error::custom)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_value_conversions_as_str {
    ($type_name:ident) => {
        impl TryFrom<zbus::zvariant::OwnedValue> for $type_name {
            type Error = zbus::Error;

            fn try_from(value: zbus::zvariant::OwnedValue) -> Result<Self, zbus::Error> {
                use std::str::FromStr;
                let value = <String>::try_from(value)?;
                return Ok($type_name::from_str(value.as_str())?);
            }
        }
        impl<'a> TryFrom<zbus::zvariant::Value<'a>> for $type_name {
            type Error = zbus::Error;

            fn try_from(value: zbus::zvariant::Value<'a>) -> Result<Self, zbus::Error> {
                use std::str::FromStr;
                let value = <zbus::zvariant::Str>::try_from(value)?;
                return Ok($type_name::from_str(value.as_str())?);
            }
        }
        impl<'a> From<&'a $type_name> for zbus::zvariant::OwnedValue {
            fn from(value: &'a $type_name) -> Self {
                let string: &str = value.into();
                zbus::zvariant::Str::from(string).into()
            }
        }
        impl From<$type_name> for zbus::zvariant::OwnedValue {
            fn from(value: $type_name) -> Self {
                let string: &str = value.into();
                zbus::zvariant::Str::from(string).into()
            }
        }
        impl<'a> From<&'a $type_name> for zbus::zvariant::Value<'a> {
            fn from(value: &'a $type_name) -> Self {
                let string: &str = value.into();
                zbus::zvariant::Str::from(string).into()
            }
        }
        impl<'a> From<$type_name> for zbus::zvariant::Value<'a> {
            fn from(value: $type_name) -> Self {
                let string: &str = value.into();
                zbus::zvariant::Str::from(string).into()
            }
        }
    };
}

#[macro_export]
macro_rules! enum_impl_str_conv {
    ($type_name:ident, { $($label:tt : $variant:tt,)* }) => {
        impl std::str::FromStr for $type_name {
            type Err = zbus::fdo::Error;

            fn from_str(m: &str) -> Result<Self, Self::Err> {
                let res = match m {
                    $($label => $type_name::$variant,)+
                    _ => return Err(zbus::fdo::Error::IOError(format!("{} is an invalid variant", m))),
                };
                Ok(res)
            }
        }

        impl From<$type_name> for &str {
            fn from(m: $type_name) -> Self {
                match m {
                    $($type_name::$variant => $label,)+
                }
            }
        }

        impl From<&$type_name> for &str {
            fn from(s: &$type_name) -> Self {
                <&str>::from(*s)
            }
        }
}}
