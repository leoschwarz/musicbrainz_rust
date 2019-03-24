//! Attempt at prototyping the new entity API exemplary for the release entity.

use crate::entities::{Alias, Mbid, PartialDate, Language, Duration};
use crate::entities::refs::{ArtistRef, LabelRef, RecordingRef};
use xpath_reader::{FromXml, FromXmlOptional, Reader};
use crate::client::Request;
use crate::entities::{OnRequest, Resource};

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
    options: ReleaseOptions,
}

/// A `Release` is any publication of one or more tracks.
#[derive(Clone, Debug)]
pub struct ReleaseResponse {
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

#[derive(Clone, Debug)]
pub struct ReleaseOptions {
    pub annotation: bool,
    pub artists: bool,
    pub recordings: bool,
    pub labels: bool,
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
        self.response.status.clone()
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
        OnRequest::from_option(self.response.annotation.as_ref(), self.options.annotation)
    }

    /// The mediums (disks) of the release.
    pub fn mediums(&self) -> OnRequest<&[ReleaseMedium]> {
        if self.options.recordings {
            OnRequest::Some(self.response.mediums.as_ref())
        } else {
            OnRequest::NotRequested
        }
    }

    /// The artists that the release is primarily credited to.
    pub fn artists(&self) -> OnRequest<&[ArtistRef]> {
        if self.options.artists {
            OnRequest::Some(self.response.artists.as_slice())
        } else {
            OnRequest::NotRequested
        }
    }

    /// The labels which issued this release.
    pub fn labels(&self) -> OnRequest<&[LabelInfo]> {
        if self.options.labels {
            OnRequest::Some(self.response.labels.as_slice())
        } else {
            OnRequest::NotRequested
        }
    }
}

impl ReleaseOptions {
    /// Request everything from the server.
    pub fn everything() -> Self {
        ReleaseOptions {
            annotation: true,
            artists: true,
            recordings: true,
            labels: true,
        }
    }

    /// Only request the minimal amount of fields.
    pub fn minimal() -> Self {
        ReleaseOptions {
            annotation: false,
            artists: false,
            recordings: false,
            labels: false,
        }
    }
}

impl Resource for Release {
    type Options = ReleaseOptions;
    type Response = ReleaseResponse;

    const NAME: &'static str = "release";

    fn request(options: &Self::Options) -> Request {
        let mut includes = Vec::new();

        if options.annotation {
            includes.push("annotation");
        }
        if options.artists {
            includes.push("artists");
        }
        if options.labels {
            includes.push("labels");
        }
        if options.recordings {
            includes.push("recordings");
        }

        Request {
            name: "release".into(),
            include: includes.join("+"),
        }
    }

    fn from_response(response: Self::Response, options: Self::Options) -> Self {
        Release { response, options }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn release_read_xml1() {
        let mbid = Mbid::from_str("ed118c5f-d940-4b52-a37b-b1a205374abe").unwrap();
        let options = ReleaseOptions::everything();
        let release: Release = crate::util::test_utils::fetch_entity(&mbid, options).unwrap();

        assert_eq!(release.mbid(), &mbid);
        assert_eq!(release.title(), "Creep");
        assert_eq!(
            release.artists().unwrap(),
            &[ArtistRef {
                mbid: Mbid::from_str("a74b1b7f-71a5-4011-9441-d0b5e4122711").unwrap(),
                name: "Radiohead".to_string(),
                sort_name: "Radiohead".to_string(),
            }]
        );
        assert_eq!(
            release.date(),
            Some(&PartialDate::from_str("1992-09-21").unwrap())
        );
        assert_eq!(release.country(), Some(&"GB".to_string()));
        assert_eq!(
            release.labels().unwrap(),
            &[
                LabelInfo {
                    label: Some(LabelRef {
                        mbid: Mbid::from_str("df7d1c7f-ef95-425f-8eef-445b3d7bcbd9").unwrap(),
                        name: "Parlophone".to_string(),
                        sort_name: "Parlophone".to_string(),
                        label_code: Some("299".to_string()),
                    }),
                    catalog_number: Some("7243 8 80234 2 9".to_string()),
                },
                LabelInfo {
                    label: Some(LabelRef {
                        mbid: Mbid::from_str("df7d1c7f-ef95-425f-8eef-445b3d7bcbd9").unwrap(),
                        name: "Parlophone".to_string(),
                        sort_name: "Parlophone".to_string(),
                        label_code: Some("299".to_string()),
                    }),
                    catalog_number: Some("CDR 6078".to_string()),
                }
            ]
        );
        assert_eq!(release.barcode(), Some(&"724388023429".to_string()));
        assert_eq!(release.status(), Some(ReleaseStatus::Official));
        assert_eq!(
            release.language(),
            Some(&Language::from_639_3("eng").unwrap())
        );
        assert_eq!(release.script(), Some(&"Latn".to_string()));
        assert_eq!(release.disambiguation(), None);
        assert_eq!(release.mediums().unwrap().len(), 1);
    }

    #[test]
    fn disambiguation() {
        let mbid = Mbid::from_str("9642c552-a5b3-4b7e-9168-aeb2a1a06f27").unwrap();
        let options = ReleaseOptions::minimal();
        let release: Release = crate::util::test_utils::fetch_entity(&mbid, options).unwrap();

        assert_eq!(release.disambiguation(), Some(&"通常盤".to_string()));
    }

    #[test]
    fn release_read_xml2() {
        let mbid = Mbid::from_str("785d7c67-a920-4cee-a871-8cd9896eb8aa").unwrap();
        let mut options = ReleaseOptions::minimal();
        options.labels = true;
        let release: Release = crate::util::test_utils::fetch_entity(&mbid, options).unwrap();

        // We check for the things we didn't check in the previous test.
        assert_eq!(release.packaging(), Some(&"Jewel Case".to_string()));
        assert_eq!(
            release.labels().unwrap(),
            &[
                LabelInfo {
                    label: Some(LabelRef {
                        mbid: Mbid::from_str("376d9b4d-8cdd-44be-bc0f-ed5dfd2d2340").unwrap(),
                        name: "Cherrytree Records".to_string(),
                        sort_name: "Cherrytree Records".to_string(),
                        label_code: None,
                    }),
                    catalog_number: Some("0251766489".to_string()),
                },
                LabelInfo {
                    label: Some(LabelRef {
                        mbid: Mbid::from_str("2182a316-c4bd-4605-936a-5e2fac52bdd2").unwrap(),
                        name: "Interscope Records".to_string(),
                        sort_name: "Interscope Records".to_string(),
                        label_code: Some("6406".to_string()),
                    }),
                    catalog_number: Some("0251766489".to_string()),
                },
                LabelInfo {
                    label: Some(LabelRef {
                        mbid: Mbid::from_str("061587cb-0262-46bc-9427-cb5e177c36a2").unwrap(),
                        name: "Konlive".to_string(),
                        sort_name: "Konlive".to_string(),
                        label_code: None,
                    }),
                    catalog_number: Some("0251766489".to_string()),
                },
                LabelInfo {
                    label: Some(LabelRef {
                        mbid: Mbid::from_str("244dd29f-b999-40e4-8238-cb760ad05ac6").unwrap(),
                        name: "Streamline Records".to_string(),
                        sort_name: "Streamline Records".to_string(),
                        label_code: None,
                    }),
                    catalog_number: Some("0251766489".to_string()),
                },
                LabelInfo {
                    label: Some(LabelRef {
                        mbid: Mbid::from_str("6cee07d5-4cc3-4555-a629-480590e0bebd").unwrap(),
                        name: "Universal Music Canada".to_string(),
                        sort_name: "Universal Music Canada".to_string(),
                        label_code: None,
                    }),
                    catalog_number: Some("0251766489".to_string()),
                },
            ]
        );
        assert_eq!(release.mediums(), OnRequest::NotRequested);
    }

    #[test]
    fn read_tracks() {
        let mbid = Mbid::from_str("d1881a4c-0188-4f0f-a2e7-4e7849aec109").unwrap();
        let mut options = ReleaseOptions::minimal();
        options.recordings = true;
        let release: Release = crate::util::test_utils::fetch_entity(&mbid, options).unwrap();

        let mediums = release.mediums().unwrap();
        assert_eq!(mediums.len(), 1);
        let medium = mediums.get(0).unwrap();
        assert_eq!(medium.position, 1);
        assert_eq!(medium.tracks.len(), 3);
        assert_eq!(
            medium.tracks[0],
            ReleaseTrack {
                mbid: Mbid::from_str("ac898be7-2965-4d17-9ac8-48d45852d73c").unwrap(),
                position: 1,
                number: "1".to_string(),
                title: "puella tenebrarum".to_string(),
                length: Some(Duration::from_millis(232000)),
                recording: RecordingRef {
                    mbid: Mbid::from_str("fd6f4cd8-9cff-43da-8cd7-3351357b6f5a").unwrap(),
                    title: "Puella Tenebrarum".to_string(),
                    length: Some(Duration::from_millis(232000)),
                },
            }
        );
        assert_eq!(
            medium.tracks[1],
            ReleaseTrack {
                mbid: Mbid::from_str("21648b0b-deaf-4b93-a257-5fc18363b25d").unwrap(),
                position: 2,
                number: "2".to_string(),
                title: "LAMINA MALEDICTUM".to_string(),
                length: Some(Duration::from_millis(258000)),
                recording: RecordingRef {
                    mbid: Mbid::from_str("0eeb0621-8013-4c0e-8e49-ddfd78d56051").unwrap(),
                    title: "Lamina Maledictum".to_string(),
                    length: Some(Duration::from_millis(258000)),
                },
            }
        );
        assert_eq!(
            medium.tracks[2],
            ReleaseTrack {
                mbid: Mbid::from_str("e57b3990-eb36-476e-beac-583e0bbe6f87").unwrap(),
                position: 3,
                number: "3".to_string(),
                title: "SARNATH".to_string(),
                length: Some(Duration::from_millis(228000)),
                recording: RecordingRef {
                    mbid: Mbid::from_str("53f87e98-351e-453e-b949-bdacf4cbeccd").unwrap(),
                    title: "Sarnath".to_string(),
                    length: Some(Duration::from_millis(228000)),
                },
            }
        );
    }

    #[test]
    fn tracks_without_length() {
        let mbid = Mbid::from_str("02173013-59ed-4229-b0a5-e5aa486ed5d7").unwrap();
        let mut options = ReleaseOptions::minimal();
        options.recordings = true;
        let release: Release = crate::util::test_utils::fetch_entity(&mbid, options).unwrap();

        let ref medium = release.mediums().unwrap()[0];
        assert_eq!(medium.tracks[0].length, None);
        assert_eq!(medium.tracks[1].length, None);
        assert_eq!(medium.tracks[2].length, None);
        assert_eq!(medium.tracks[3].length, None);
    }

    #[test]
    fn multi_cd() {
        let mbid = Mbid::from_str("ce22b20d-3a45-4e47-abaa-b7c8d10281fa").unwrap();
        let mut options = ReleaseOptions::minimal();
        options.recordings = true;
        let release: Release = crate::util::test_utils::fetch_entity(&mbid, options).unwrap();

        let mediums = release.mediums().unwrap();

        assert_eq!(mediums.len(), 2);

        assert_eq!(mediums[0].position, 1);
        assert_eq!(mediums[0].tracks.len(), 11);
        assert_eq!(mediums[0].tracks[0].position, 1);
        assert_eq!(mediums[0].tracks[0].number, "1".to_string());
        assert_eq!(mediums[0].tracks[1].position, 2);
        assert_eq!(mediums[0].tracks[1].number, "2".to_string());

        assert_eq!(mediums[1].position, 2);
        assert_eq!(mediums[1].tracks.len(), 9);
        assert_eq!(mediums[1].tracks[0].position, 1);
        assert_eq!(mediums[1].tracks[0].number, "1".to_string());
        assert_eq!(mediums[1].tracks[1].position, 2);
        assert_eq!(mediums[1].tracks[1].number, "2".to_string());
    }

    /// It's possible that a release has a catalog number but is not linked to
    /// any label in the database.
    #[test]
    fn catalog_number_but_no_label_ref() {
        let mbid = Mbid::from_str("61f8b05f-a3b5-49f4-a3a6-8f0d564c1664").unwrap();
        let mut options = ReleaseOptions::minimal();
        options.labels = true;
        let release: Release = crate::util::test_utils::fetch_entity(&mbid, options).unwrap();

        assert_eq!(
            release.labels().unwrap(),
            &[LabelInfo {
                label: None,
                catalog_number: Some("BIRD 4".to_string()),
            },]
        );
    }
}
