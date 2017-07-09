// TODO: this should probably be moved to a different file/directory
// TODO: validate input dates for validity
// TODO: Write conversions to and from `chrono` date types for interoperability.
use std;
use std::str::FromStr;
use std::num::ParseIntError;
use std::error::Error;
use std::fmt::Display;
use xpath_reader::{FromXml, FromXmlError, XpathReader};

/// Represents a partial date as it is used across MusicBrainz.
///
/// Note that even completely empty dates are possible.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartialDate {
    year: Option<u16>,
    month: Option<u8>,
    day: Option<u8>,
}

/// Represents a fully specified date.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FullDate {
    year: u16,
    month: u8,
    day: u8,
}

impl PartialDate {
    pub fn new(year: Option<u16>, month: Option<u8>, day: Option<u8>) -> PartialDate
    {
        PartialDate {
            year: year,
            month: month,
            day: day,
        }
    }

    pub fn year(&self) -> Option<u16>
    {
        self.year
    }

    pub fn month(&self) -> Option<u8>
    {
        self.month
    }

    pub fn day(&self) -> Option<u8>
    {
        self.day
    }

    /// If this `PartialDate` is fully specified, `Some(FullDate)` will be
    /// returned,
    /// otherwise `None` will be returned.
    ///
    /// # Examples
    /// ```
    /// use musicbrainz::entities::{FullDate, PartialDate};
    ///
    /// assert_eq!(PartialDate::new(None, None, None).full_date(), None);
    /// assert_eq!(PartialDate::new(None, None, Some(2)).full_date(), None);
    /// assert_eq!(PartialDate::new(None, Some(2), Some(2)).full_date(), None);
    ///
    /// assert_eq!(
    ///     PartialDate::new(Some(2017), Some(2), Some(2)).full_date(),
    ///     Some(FullDate::new(2017, 2, 2))
    /// );
    /// ```
    pub fn full_date(&self) -> Option<FullDate>
    {
        if self.year.is_some() && self.month.is_some() && self.day.is_some() {
            Some(FullDate {
                year: self.year.unwrap(),
                month: self.month.unwrap(),
                day: self.day.unwrap(),
            })
        } else {
            None
        }
    }
}

impl FullDate {
    pub fn new(year: u16, month: u8, day: u8) -> FullDate
    {
        FullDate {
            year: year,
            month: month,
            day: day,
        }
    }

    pub fn year(&self) -> u16
    {
        self.year
    }

    pub fn month(&self) -> u8
    {
        self.month
    }

    pub fn day(&self) -> u8
    {
        self.day
    }
}

impl FromStr for PartialDate {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        // Get the pieces of the date.
        let ps: Result<Vec<Option<u16>>, ParseIntError> = s.split("-")
            .map(|x| if x == "??" || x == "????" {
                Ok(None)
            } else {
                x.parse().map(|y| Some(y))
            })
            .collect();

        // Create result.
        let ps = ps?;
        if ps.len() == 1 {
            Ok(PartialDate {
                year: ps[0],
                month: None,
                day: None,
            })
        } else if ps.len() == 2 {
            Ok(PartialDate {
                year: ps[0],
                month: ps[1].map(|i| i as u8),
                day: None,
            })
        } else if ps.len() == 3 {
            Ok(PartialDate {
                year: ps[0],
                month: ps[1].map(|i| i as u8),
                day: ps[2].map(|i| i as u8),
            })
        } else {
            Err(ParseDateError::WrongNumberOfComponents(ps.len()))
        }
    }
}

impl Display for PartialDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        // TODO optimize later (no allocations)
        let year = self.year.map(|n| format!("{:04}", n)).unwrap_or_else(|| "????".to_string());
        let month = self.month.map(|n| format!("{:02}", n)).unwrap_or_else(|| "??".to_string());
        let day = self.day.map(|n| format!("{:02}", n)).unwrap_or_else(|| "??".to_string());
        write!(f, "{}-{}-{}", year, month, day)
    }
}

impl FromXml for PartialDate {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        use xpath_reader::errors::ChainXpathErr;
        String::from_xml(reader)?.parse().chain_err(|| "Parse Date error").map_err(
            |e| FromXmlError::from(e),
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseDateError {
    /// A wrong number of `-` separated components was found in the string.
    WrongNumberOfComponents(usize),

    /// Failed parsing a component into the appropriate number type.
    ComponentInvalid(ParseIntError),
}

impl Error for ParseDateError {
    fn description(&self) -> &str
    {
        use self::ParseDateError::*;
        match *self {
            WrongNumberOfComponents(_) => "wrong number of components",
            ComponentInvalid(_) => "invalid component",
        }
    }
}

// TODO: Evaluate if this is what we want and if we can use this like this in
// requests.
impl Display for ParseDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        use self::ParseDateError::*;
        match *self {
            WrongNumberOfComponents(n) => {
                write!(f, "ParseDateError: Wrong number of components: {}", n)
            }
            ComponentInvalid(ref err) => write!(f, "ParseDateError: Component invalid: {:?}", err),
        }
    }
}

impl From<ParseIntError> for ParseDateError {
    fn from(e: ParseIntError) -> Self
    {
        ParseDateError::ComponentInvalid(e)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    const DATE_1: Date = Date::Year { year: 2017 };
    const DATE_2: Date = Date::Month {
        year: 2017,
        month: 4,
    };
    const DATE_3: Date = Date::Day {
        year: 2017,
        month: 4,
        day: 15,
    };

    #[test]
    fn parse_valid()
    {
        let date1 = Date::from_str("2017").unwrap();
        let date2 = Date::from_str("2017-4").unwrap();
        let date3 = Date::from_str("2017-04-15").unwrap();

        assert_eq!(date1, DATE_1);
        assert_eq!(date2, DATE_2);
        assert_eq!(date3, DATE_3);
    }

    #[test]
    fn accessors()
    {
        assert_eq!(DATE_1.year(), 2017);
        assert_eq!(DATE_1.month(), 0);
        assert_eq!(DATE_1.day(), 0);
        assert_eq!(DATE_2.year(), 2017);
        assert_eq!(DATE_2.month(), 4);
        assert_eq!(DATE_2.day(), 0);
        assert_eq!(DATE_3.year(), 2017);
        assert_eq!(DATE_3.month(), 4);
        assert_eq!(DATE_3.day(), 15);
    }

    #[test]
    fn wrong_number_comps()
    {
        let fail = Date::from_str("1-1-1-1");
        assert_eq!(
            fail.err().unwrap(),
            ParseDateError::WrongNumberOfComponents(4)
        );
    }

    #[test]
    fn invalid_components()
    {
        let fail1 = Date::from_str("abc");
        let fail2 = Date::from_str("2017-abc");
        let fail3 = Date::from_str("2017-04-abc");

        let err = ParseDateError::from("abc".parse::<u16>().err().unwrap());

        assert_eq!(fail1.err().unwrap(), err);
        assert_eq!(fail2.err().unwrap(), err);
        assert_eq!(fail3.err().unwrap(), err);
    }

    #[test]
    fn to_string()
    {
        assert_eq!(DATE_1.to_string(), "2017".to_string());
        assert_eq!(DATE_2.to_string(), "2017-04".to_string());
        assert_eq!(DATE_3.to_string(), "2017-04-15".to_string());
    }
}*/
