//! This module contains structs for types we call *reference types* in this
//! library.
//!
//! These types only contain some basic data but reference a full entity in the
//! MusicBrainz
//! database which can be retrieved.

// TODO: Better documentation in this file.
// TODO: When writing the API interfacing code, provide some form of helpers so
// the full referenced
// types corresponding to these ref types can be easily retrieved from
// the server.

use std::time::Duration;
use xpath_reader::{FromXml, FromXmlError, XpathReader};
use xpath_reader::reader::FromXmlElement;

use entities::Mbid;
use entities::date::PartialDate;
use entities::release::ReleaseStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AreaRef {
    pub mbid: Mbid,
    pub name: String,
    pub sort_name: String,
    pub iso_3166: Option<String>,
}

impl FromXmlElement for AreaRef {}
impl FromXml for AreaRef {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(AreaRef {
            mbid: reader.read(".//@id")?,
            name: reader.read(".//mb:name/text()")?,
            sort_name: reader.read(".//mb:sort-name/text()")?,
            iso_3166: reader.read_option(".//mb:iso-3166-1-code-list/mb:iso-3166-1-code/text()")?,
        })
    }
}

/// A small variation of `Artist` which is used only to refer to an actual
/// artist entity from other
/// entities.
/// TODO: new docstring
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtistRef {
    pub mbid: Mbid,
    pub name: String,
    pub sort_name: String,
}

impl FromXmlElement for ArtistRef {}
impl FromXml for ArtistRef {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(ArtistRef {
            mbid: reader.read(".//@id")?,
            name: reader.read(".//mb:name/text()")?,
            sort_name: reader.read(".//mb:sort-name/text()")?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LabelRef {
    pub mbid: Mbid,
    pub name: String,
    pub sort_name: String,
    pub label_code: Option<String>,
}

impl FromXmlElement for LabelRef {}
impl FromXml for LabelRef {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(LabelRef {
            mbid: reader.read(".//@id")?,
            name: reader.read(".//mb:name/text()")?,
            sort_name: reader.read(".//mb:sort-name/text()")?,
            label_code: reader.read_option(".//mb:label-code/text()")?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordingRef {
    pub mbid: Mbid,
    pub title: String,
    pub length: Option<Duration>,
}

impl FromXmlElement for RecordingRef {}
impl FromXml for RecordingRef {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(RecordingRef {
            mbid: reader.read(".//@id")?,
            title: reader.read(".//mb:title/text()")?,
            length: ::entities::helper::read_mb_duration(reader, ".//mb:length/text()")?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseRef {
    pub mbid: Mbid,
    pub title: String,
    pub date: Option<PartialDate>,
    pub status: Option<ReleaseStatus>,
    pub country: Option<String>,
}

impl FromXmlElement for ReleaseRef {}
impl FromXml for ReleaseRef {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(ReleaseRef {
            mbid: reader.read(".//@id")?,
            title: reader.read(".//mb:title/text()")?,
            date: reader.read_option(".//mb:date/text()")?,
            status: reader.read_option(".//mb:status/text()")?,
            country: reader.read_option(".//mb:country/text()")?,
        })
    }
}
