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

        impl FromXmlElement for $enum {}
        impl FromXml for $enum {
            fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
            where
                R: XpathReader<'d>,
            {
                match String::from_xml(reader)?.as_str() {
                    $(
                        $str => Ok($enum::$variant),
                    )+
                    "" => Err(FromXmlError::Absent),
                    s => Err(format!("Unknown `{}` value: '{}'", stringify!($enum), s).into()),
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
