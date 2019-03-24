use std::time::Duration;
use xpath_reader::{FromXml, FromXmlOptional, Error, Reader};

use crate::entities::{Language, Mbid, ResourceOld};
use crate::entities::date::PartialDate;
use crate::entities::refs::{ArtistRef, LabelRef, RecordingRef};

/// Describes a single track, `Releases` consist of multiple `ReleaseTrack`s.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseTrack {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    /// The position of the track on the `Release`.
    ///
    /// TODO (clarification) : what is the difference between `position` and
    /// `number`???
    pub position: u16,

    /// The track number as listed in the release.
    ///
    /// For CDs this will usually be numbers, but for example for vinyl this is
    /// "A", "AA", etc.
    pub number: String,

    /// The title of the track.
    pub title: String,

    /// The length of the track.
    pub length: Option<Duration>,

    /// The recording used for the track.
    pub recording: RecordingRef,
}

impl FromXml for ReleaseTrack {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, Error> {
        Ok(ReleaseTrack {
            mbid: reader.read(".//@id")?,
            position: reader.read(".//mb:position/text()")?,
            number: reader.read(".//mb:number/text()")?,
            title: reader.read(".//mb:title/text()")?,
            length: crate::entities::helper::read_mb_duration(reader, ".//mb:length/text()")?,
            recording: reader.read(".//mb:recording")?,
        })
    }
}

/*
TODO: Parse the format. We have to yet consider if everything should get its own variant or only the larger classes of mediums should get one and subclasses would be specified as string variants.
enum_mb_xml! {
    /// Specifies the format of a `ReleaseMedium`.
    pub enum ReleaseMediumFormat {
        var DigitalMedia = "Digital Media",
    }
}
*/

/// A medium is a collection of multiple `ReleaseTrack`.
///
/// For physical releases one medium might equal one CD, so an album released
/// as a release with two CDs would have two associated `ReleaseMedium`
/// instances.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseMedium {
    /// The medium's position number providing a total order between all
    /// mediums of one `Release`.
    position: u16,

    /// The format of this `ReleaseMedium`.
    ///
    /// TODO: Parse into `ReleaseMediumFormat` enum.
    format: Option<String>,

    /// The tracks stored on this medium.
    tracks: Vec<ReleaseTrack>,
}

impl FromXml for ReleaseMedium {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, Error> {
        Ok(ReleaseMedium {
            position: reader.read(".//mb:position/text()")?,
            format: reader.read(".//mb:format/text()")?,
            tracks: reader.read(".//mb:track-list/mb:track")?,
        })
    }
}

enum_mb_xml_optional! {
    pub enum ReleaseStatus {
        /// Release officially sanctioned by the artist and/or their record company.
        var Official = "Official",

        /// A give-away release or a release intended to promote an upcoming
        /// official release.
        var Promotion = "Promotion",

        /// Unofficial/underground release that was not sanctioned by the artist
        /// and/or the record company.
        /// Includes unofficial live recordings and pirated releases.
        var Bootleg = "Bootleg",

        /// An alternate version of a release where the titles have been changed,
        /// usually for transliteration.
        ///
        /// These don't correspond to a real release and should be linked to the
        /// actual release using the transliteration relationship.
        var PseudoRelease = "Pseudo-Release",
    }
}

/// Lists information about a `Release`.
///
/// Note that its both possible to find a `LabelInfo` with only one of `label`
/// or `cat_num`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LabelInfo {
    /// A reference to the label issuing the release.
    pub label: Option<LabelRef>,

    /// Catalog number of the release as released by the label.
    pub catalog_number: Option<String>,
}

impl FromXml for LabelInfo {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, Error> {
        Ok(LabelInfo {
            label: {
                let id: Option<String> = reader.read(".//@id")?;
                match id {
                    Some(_) => Some(reader.read(".")?),
                    None => None,
                }
            },
            catalog_number: reader.read(".//mb:catalog-number/text()")?,
        })
    }
}

/// A `Release` is any publication of one or more tracks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Release {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    /// The title of the release.
    pub title: String,

    /// The artists that the release is primarily credited to.
    pub artists: Vec<ArtistRef>,

    /// The date the release was issued.
    pub date: Option<PartialDate>,

    /// The country the release was issued in.
    pub country: Option<String>,

    /// The labels which issued this release.
    pub labels: Vec<LabelInfo>,

    /// Barcode of the release, if it has one.
    pub barcode: Option<String>,

    /// Official status of the release.
    pub status: Option<ReleaseStatus>,

    /// Packaging of the release.
    /// TODO: Consider an enum for the possible packaging types.
    pub packaging: Option<String>,

    /// Language of the release. ISO 639-3 conformant string.
    pub language: Option<Language>,

    /// Script used to write the track list. ISO 15924 conformant string.
    pub script: Option<String>,

    /// A disambiguation comment if present, which allows to differentiate this
    /// release easily from
    /// other releases with the same or very similar name.
    pub disambiguation: Option<String>,

    /// Any additional free form annotation for this `Release`.
    pub annotation: Option<String>,

    /// The mediums (disks) of the release.
    pub mediums: Vec<ReleaseMedium>,
}

impl FromXml for Release {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, Error> {
        Ok(Release {
            annotation: reader.read(".//mb:release/mb:annotation/mb:text/text()")?,
            artists: reader.read(".//mb:release/mb:artist-credit/mb:name-credit")?,
            barcode: reader.read(".//mb:release/mb:barcode/text()")?,
            country: reader.read(".//mb:release/mb:country/text()")?,
            date: reader.read(".//mb:release/mb:date/text()")?,
            disambiguation: reader.read(".//mb:release/mb:disambiguation/text()")?,
            labels: reader.read(".//mb:release/mb:label-info-list/mb:label-info")?,
            language: reader.read(".//mb:release/mb:text-representation/mb:language/text()")?,
            mbid: reader.read(".//mb:release/@id")?,
            mediums: reader.read(".//mb:release/mb:medium-list/mb:medium")?,
            packaging: reader.read(".//mb:release/mb:packaging/text()")?,
            script: reader.read(".//mb:release/mb:text-representation/mb:script/text()")?,
            status: reader.read(".//mb:release/mb:status/text()")?,
            title: reader.read(".//mb:release/mb:title/text()")?,
        })
    }
}

impl ResourceOld for Release {
    const NAME: &'static str = "release";
    const INCL: &'static str = "aliases+annotation+artists+labels+recordings";
}
