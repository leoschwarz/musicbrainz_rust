//! Attempt at prototyping the new entity API exemplary for the release entity.

use crate::entities::{Mbid, PartialDate, Language, Duration};
use crate::entities::refs::{ArtistRef, LabelRef, RecordingRef};
use xpath_reader::{FromXml, FromXmlOptional, Reader};

pub enum OnRequest<T> {
    Some(T),
    NotAvailable,
    NotRequested,
}

impl<T> OnRequest<T> {
    pub(crate) fn from_option(option: Option<T>, requested: bool) -> OnRequest<T> {
        match (option, requested) {
            (Some(val), _) => OnRequest::Some(option),
            (None, true) => OnRequest::NotAvailable,
            (None, false) => OnRequest::NotRequested,
        }
    }
}

/*
impl<T> From<OnRequest<T>> for Option<T> {
    fn from(o: OnRequest<T>) -> Option<T> {
        match o {
            OnRequest::Some(t) => Some(t),
            OnRequest::NotAvailable | OnRequest::NotRequested => None,
        }
    }
}
*/

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum ReleaseComponent {
    Base,
    Aliases,
    Annotation,
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum ReleaseRelations {
    Artists,
    Labels,
    Recordings,
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

#[derive(Clone, Debug)]
pub struct Release {
    response: ReleaseResponse,
    requested_annotation: bool,
    requested_artists: bool,
    requested_labels: bool,
}

/// A `Release` is any publication of one or more tracks.
#[derive(Clone, Debug)]
struct ReleaseResponse {
    mbid: Mbid,
    title: String,
    artists: Vec<ArtistRef>,
    date: Option<PartialDate>,
    country: Option<String>,
    labels: Vec<LabelInfo>,
    barcode: Option<String>,
    status: Option<ReleaseStatus>,
    packaging: Option<String>,
    language: Option<Language>,
    script: Option<String>,
    disambiguation: Option<String>,
    annotation: Option<String>,
    mediums: Vec<ReleaseMedium>,
}

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

impl Release {
    /// MBID of the entity in the MusicBrainz database.
    pub fn mbid(&self) -> &Mbid {
        &self.response.mbid
    }

    /// The title of the release.
    pub fn title(&self) -> &String {
        &self.response.title
    }

    /// The date the release was issued.
    pub fn date(&self) -> Option<&PartialDate> {
        self.response.date.as_ref()
    }

    /// The country the release was issued in.
    pub fn country(&self) -> Option<&String> {
        self.response.country.as_ref()
    }

    /// Release status of the release.
    pub fn status(&self) -> Option<ReleaseStatus> {
        self.response.status
    }

    /// Barcode of the release, if it has one.
    pub fn barcode(&self) -> Option<&String> {
        self.response.barcode.as_ref()
    }

    /// Packaging of the release.
    /// TODO: Consider an enum for the possible packaging types.
    pub fn packaging(&self) -> Option<&String> {
        self.response.packaging.as_ref()
    }

    /// Language of the release. (ISO 639-3 conformant string in DB.)
    pub fn language(&self) -> Option<&Language> {
        self.response.language.as_ref()
    }

    /// Script used to write the track list. (ISO 15924 conformant string in DB.)
    pub fn script(&self) -> Option<&String> {
        self.response.script.as_ref()
    }

    /// A disambiguation comment if present, which allows to differentiate this
    /// release easily from other releases with the same or very similar name.
    pub fn disambiguation(&self) -> Option<&String> {
        self.response.disambiguation.as_ref()
    }

    /// Any additional free form annotation for this `Release`.
    pub fn annotation(&self) -> OnRequest<&String> {
        OnRequest::from_option(self.annotation.as_ref(), self.requested_annotation)
    }

    /// The mediums (disks) of the release.
    pub fn mediums(&self) -> &[ReleaseMedium] {
        self.response.mediums.as_ref()
    }

    /// The artists that the release is primarily credited to.
    pub fn artists(&self) -> OnRequest<&[ArtistRef]> {
        if self.requested_artists {
            OnRequest::Some(self.artists.as_slice())
        } else {
            OnRequest::NotRequested
        }
    }

    /// The labels which issued this release.
    pub fn labels(&self) -> OnRequest<&[LabelRef]> {
        if self.requested_labels {
            OnRequest::Some(self.labels.as_slice())
        } else {
            OnRequest::NotRequested
        }
    }
}

impl FromXml for ReleaseResponse {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
        Ok(ReleaseResponse {
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

impl FromXml for ReleaseMedium {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
        Ok(ReleaseMedium {
            position: reader.read(".//mb:position/text()")?,
            format: reader.read(".//mb:format/text()")?,
            tracks: reader.read(".//mb:track-list/mb:track")?,
        })
    }
}

impl FromXml for ReleaseTrack {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
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

impl FromXml for LabelInfo {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
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
