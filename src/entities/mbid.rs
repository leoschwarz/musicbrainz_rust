use super::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;
use uuid::{self, Uuid};

// Identifier for entities in the MusicBrainz database.
#[derive(Clone, PartialEq, Eq)]
pub struct Mbid {
    uuid: Uuid,
}

impl From<Uuid> for Mbid {
    fn from(uuid: Uuid) -> Self
    {
        Mbid { uuid: uuid }
    }
}

impl From<Mbid> for Uuid {
    fn from(mbid: Mbid) -> Self
    {
        mbid.uuid
    }
}

impl FromStr for Mbid {
    type Err = uuid::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        Ok(Mbid { uuid: Uuid::parse_str(s)? })
    }
}

impl Debug for Mbid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "Mbid: {:?}", self.uuid)
    }
}

impl Display for Mbid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "{}", self.uuid.hyphenated())
    }
}

impl FromXml for Mbid {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, XpathError>
        where R: XpathReader<'d>
    {
        use xpath_reader::errors::ChainXpathErr;
        Ok(String::from_xml(reader)?.parse().chain_err(|| "Failed parsing MBID")?)
    }
}
