use crate::errors::ParseError;
use isolang::Language as IsoLang;
use std::fmt;
use xpath_reader::{FromXml, FromXmlOptional, Error, Reader};

/// Represents verbal languages.
#[derive(Clone, Eq, PartialEq)]
pub struct Language {
    inner: IsoLang,
}

impl Language {
    /// Construct a new instance from an ISO 639-1 language code.
    pub fn from_639_1(code: &str) -> Result<Language, ParseError> {
        Ok(Language {
            inner: IsoLang::from_639_1(code)
                .ok_or_else(|| ParseError::from(format!("Invalid ISO 639-1 code: {}", code)))?,
        })
    }

    /// Construct a new instance from an ISO 639-3 language code.
    ///
    /// These are used by MusicBrainz internally.
    pub fn from_639_3(code: &str) -> Result<Language, ParseError> {
        Ok(Language {
            inner: IsoLang::from_639_3(code)
                .ok_or_else(|| ParseError::from(format!("Invalid ISO 639-3 code: {}", code)))?,
        })
    }

    /// Return the ISO 639-1 language code.
    pub fn to_639_1(&self) -> Option<&'static str> {
        self.inner.to_639_1()
    }

    /// Return the ISO 639-3 language code.
    pub fn to_639_3(&self) -> &'static str {
        self.inner.to_639_3()
    }
}

impl fmt::Debug for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Language [ISO 639-3: {}]", self.to_639_3())
    }
}

impl FromXmlOptional for Language {
    fn from_xml_optional<'d>(reader: &'d Reader<'d>) -> Result<Option<Self>, Error> {
        let s = Option::<String>::from_xml(&reader)?;

        if let Some(s) = s {
            Language::from_639_3(s.as_str())
                .map_err(|e| {
                    ::xpath_reader::Error::custom_msg(format!("parse language error: {}", e))
                })
                .map(|l| Some(l))
        } else {
            Ok(None)
        }
    }
}

// This is needed when we use a `Language` as a `SearchField` value.
impl ToString for Language {
    fn to_string(&self) -> String {
        self.to_639_3().to_string()
    }
}
