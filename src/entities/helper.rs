use std::time::Duration;
use xpath_reader::Reader;

/// Note that the requirement of the `var` (variant) token is rather ugly but
/// required,
/// which is a limitation of the current Rust macro implementation.
///
/// Note that the macro wont expand if you miss ommit the last comma.
/// If this macro is ever extracted into a library this should be fixed.
///
/// - https://github.com/rust-lang/rust/issues/24189
/// - https://github.com/rust-lang/rust/issues/42838
macro_rules! enum_mb_xml
{
    (
        $(#[$attr:meta])* pub enum $enum:ident {
            $(
                $(#[$attr2:meta])*
                var $variant:ident = $str:expr
            ),+
            ,
        }
    )
        =>
    {
        $(#[$attr])*
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum $enum {
            $(
                $(#[$attr2])* $variant ,
            )+
        }

        impl FromXml for $enum {
            fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, ::xpath_reader::Error>
            {
                match String::from_xml(reader)?.as_str() {
                    $(
                        $str => Ok($enum::$variant),
                    )+
                    s => Err(
                        ::xpath_reader::Error::custom_msg(
                            format!("Unknown `{}` value: '{}'", stringify!($enum), s)
                        )
                    )
                }
            }
        }

        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
            {
                let s = match *self {
                    $(
                        $enum::$variant => $str,
                    )+
                };
                write!(f, "{}", s)
            }
        }
    }
}

macro_rules! enum_mb_xml_optional
{
    (
        $(#[$attr:meta])* pub enum $enum:ident {
            $(
                $(#[$attr2:meta])*
                var $variant:ident = $str:expr
            ),+
            ,
        }
    )
        =>
    {
        $(#[$attr])*
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum $enum {
            $(
                $(#[$attr2])* $variant ,
            )+
        }

        impl FromXmlOptional for $enum {
            fn from_xml_optional<'d>(reader: &'d Reader<'d>) -> Result<Option<Self>, ::xpath_reader::Error>
            {
                let s = Option::<String>::from_xml(reader)?;
                if let Some(s) = s {
                    match s.as_ref() {
                        $(
                            $str => Ok(Some($enum::$variant)),
                        )+
                        s => Err(
                            ::xpath_reader::Error::custom_msg(
                                format!("Unknown `{}` value: '{}'", stringify!($enum), s)
                            )
                        )
                    }
                } else {
                    Ok(None)
                }
            }
        }

        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
            {
                let s = match *self {
                    $(
                        $enum::$variant => $str,
                    )+
                };
                write!(f, "{}", s)
            }
        }
    }
}

pub fn read_mb_duration<'d>(
    reader: &'d Reader<'d>,
    path: &str,
) -> Result<Option<Duration>, ::xpath_reader::Error> {
    let s: Option<String> = reader.read(path)?;
    match s {
        Some(millis) => Ok(Some(Duration::from_millis(
            millis.parse().map_err(::xpath_reader::Error::custom_err)?,
        ))),
        None => Ok(None),
    }
}
