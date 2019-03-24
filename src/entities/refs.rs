//! Reference entities holding some basic information about an entity and
//! pointing to a full entity.

// TODO: Better documentation in this file.

use std::time::Duration;
use xpath_reader::{FromXml, FromXmlOptional, Reader};

use crate::entities::Mbid;
use crate::entities::date::PartialDate;
use crate::entities::release::ReleaseStatus;
use crate::client::Client;
use crate::Error;

pub trait FetchFull {
    type Full;

    fn fetch_full(&self, client: &mut Client) -> Result<Self::Full, Error>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AreaRef {
    pub mbid: Mbid,
    pub name: String,
    pub sort_name: String,
    pub iso_3166: Option<String>,
}

impl FromXmlOptional for AreaRef {
    fn from_xml_optional<'d>(reader: &'d Reader<'d>) -> Result<Option<Self>, xpath_reader::Error> {
        // TODO: is this correct
        if reader.anchor_nodeset().size() < 1 {
            Ok(None)
        } else {
            Ok(Some(AreaRef {
                mbid: reader.read(".//@id")?,
                name: reader.read(".//mb:name/text()")?,
                sort_name: reader.read(".//mb:sort-name/text()")?,
                iso_3166: reader.read(".//mb:iso-3166-1-code-list/mb:iso-3166-1-code/text()")?,
            }))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtistRef {
    pub mbid: Mbid,
    pub name: String,
    pub sort_name: String,
}

impl FromXml for ArtistRef {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
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

impl FromXml for LabelRef {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
        Ok(LabelRef {
            mbid: reader.read(".//@id")?,
            name: reader.read(".//mb:name/text()")?,
            sort_name: reader.read(".//mb:sort-name/text()")?,
            label_code: reader.read(".//mb:label-code/text()")?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordingRef {
    pub mbid: Mbid,
    pub title: String,
    pub length: Option<Duration>,
}

impl FromXml for RecordingRef {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
        Ok(RecordingRef {
            mbid: reader.read(".//@id")?,
            title: reader.read(".//mb:title/text()")?,
            length: crate::entities::helper::read_mb_duration(reader, ".//mb:length/text()")?,
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

impl FromXml for ReleaseRef {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
        Ok(ReleaseRef {
            mbid: reader.read(".//@id")?,
            title: reader.read(".//mb:title/text()")?,
            date: reader.read(".//mb:date/text()")?,
            status: reader.read(".//mb:status/text()")?,
            country: reader.read(".//mb:country/text()")?,
        })
    }
}

macro_rules! ref_fetch_full
{
    ($($ref:ty, $full:ty);+)
=>
    {
        $(
            impl FetchFull for $ref {
                type Full = $full;

                fn fetch_full(&self, client: &mut Client) -> Result<Self::Full, Error>
                {
                    client.get_by_mbid_old(&self.mbid)
                }
            }
        )+
    }
}

ref_fetch_full!(
    AreaRef, crate::entities::Area;
    ArtistRef, crate::entities::Artist;
    LabelRef, crate::entities::Label;
    RecordingRef, crate::entities::Recording;
    ReleaseRef, crate::entities::Release
);
