//! Internal macros shared by the model modules.

/// Declares a string-valued enum that serializes as its wire string and
/// deserializes tolerantly: unknown values round-trip through an
/// `Other(String)` variant instead of failing, so additions on OANDA's side
/// never break deserialization.
macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($(#[$vmeta:meta])* $variant:ident => $wire:literal,)+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $(
                #[doc = concat!("`", $wire, "`.")]
                $(#[$vmeta])*
                $variant,
            )+
            /// A value not (yet) known to this version of the SDK.
            Other(String),
        }

        impl $name {
            /// The canonical wire representation of this value.
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $wire,)+
                    Self::Other(s) => s,
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = ::std::convert::Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $($wire => Self::$variant,)+
                    other => Self::Other(other.to_owned()),
                })
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                value.parse().expect("infallible")
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                value.as_str().into()
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let s = <::std::string::String as ::serde::Deserialize>::deserialize(deserializer)?;
                Ok(s.into())
            }
        }
    };
}

pub(crate) use string_enum;
