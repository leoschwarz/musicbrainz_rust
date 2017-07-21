use std::time::Duration;
use xpath_reader::{FromXml, FromXmlError, XpathReader};
use xpath_reader::reader::{FromXmlContained, FromXmlElement};

use entities::{Mbid, Resource};
use entities::date::PartialDate;
use entities::refs::{ArtistRef, LabelRef, RecordingRef};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseTrack {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    // TODO: docstring ; what is the difference between `position` and `number`???
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

impl FromXmlElement for ReleaseTrack {}
impl FromXml for ReleaseTrack {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(ReleaseTrack {
            mbid: reader.read(".//@id")?,
            position: reader.read(".//mb:position/text()")?,
            number: reader.read(".//mb:number/text()")?,
            title: reader.read(".//mb:title/text()")?,
            length: ::entities::helper::read_mb_duration(reader, ".//mb:length/text()")?,
            recording: reader.read(".//mb:recording")?,
        })
    }
}

/// A medium is a collection of multiple `ReleaseTrack`. For physical releases
/// one medium might
/// equal one CD, so an album released as a release with two CDs would have two
/// associated
/// `ReleaseMedium` instances.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseMedium {
    /// The medium's position number providing a total order between all
    /// mediums of one `Release`.
    position: u16,

    /// The tracks stored on this medium.
    tracks: Vec<ReleaseTrack>,
}

impl FromXmlElement for ReleaseMedium {}
impl FromXml for ReleaseMedium {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(ReleaseMedium {
            position: reader.read(".//mb:position/text()")?,
            tracks: reader.read_vec(".//mb:track-list/mb:track")?,
        })
    }
}

enum_mb_xml! {
    pub enum ReleaseStatus {
        /// Release officially sanctioned by the artist and/or their record company.
        var Official = "Official",

        /// A give-away release or a release intended to promote an upcoming
        /// official release.
        var Promotion = "Promotion",

        /// Unofficial/underground release that was not sanctioned by the artist
        /// and/or the record
        /// company. Includes unoffcial live recordings and pirated releases.
        var Bootleg = "Bootleg",

        /// An alternate version of a release where the titles have been changed.
        /// These don't correspond to any real release and should be linked to the
        /// original release
        /// using the transliteration relationship.
        ///
        /// TL;DR: Essentially this shouldn't be used.
        var PseudoRelease = "Pseudo-Release",
    }
}

/// Lists information about a release.
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

impl FromXmlContained for LabelInfo {}
impl FromXml for LabelInfo {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(LabelInfo {
            label: {
                let id: Option<String> = reader.read_option(".//@id")?;
                match id {
                    Some(_) => Some(reader.read(".")?),
                    None => None,
                }
            },
            catalog_number: reader.read_option(".//mb:catalog-number/text()")?,
        })
    }
}

#[derive(Clone, Debug)]
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
    pub language: Option<String>,

    /// Script used to write the track list. ISO 15924 conformant string.
    pub script: Option<String>,

    /// A disambiguation comment if present, which allows to differentiate this
    /// release easily from
    /// other releases with the same or very similar name.
    pub disambiguation: Option<String>, // TODO: annotations

    /// The mediums (disks) of the release.
    pub mediums: Vec<ReleaseMedium>,
}

impl FromXmlContained for Release {}
impl FromXml for Release {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(Release {
            mbid: reader.read(".//mb:release/@id")?,
            title: reader.read(".//mb:release/mb:title/text()")?,
            artists: reader.read_vec(".//mb:release/mb:artist-credit/mb:name-credit")?,
            date: reader.read_option(".//mb:release/mb:date/text()")?,
            country: reader.read_option(".//mb:release/mb:country/text()")?,
            labels: reader.read_vec(".//mb:release/mb:label-info-list/mb:label-info")?,
            barcode: reader.read_option(".//mb:release/mb:barcode/text()")?,
            status: reader.read_option(".//mb:release/mb:status/text()")?,
            packaging: reader.read_option(".//mb:release/mb:packaging/text()")?,
            language:
                reader.read_option(".//mb:release/mb:text-representation/mb:language/text()")?,
            script: reader.read_option(".//mb:release/mb:text-representation/mb:script/text()")?,
            disambiguation: reader.read_option(".//mb:release/mb:disambiguation/text()")?,
            mediums: reader.read_vec(".//mb:release/mb:medium-list/mb:medium")?,
        })
    }
}

impl Resource for Release {
    fn get_url(mbid: &Mbid) -> String
    {
        format!(
            "https://musicbrainz.org/ws/2/release/{}?inc=aliases+artists+labels+recordings",
            mbid
        )
    }

    fn base_url() -> &'static str
    {
        "https://musicbrainz.org/ws/2/release/"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use xpath_reader::XpathStrReader;

    #[test]
    fn release_read_xml1()
    {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><release id="ed118c5f-d940-4b52-a37b-b1a205374abe"><title>Creep</title><status id="4e304316-386d-3409-af2e-78857eec5cfe">Official</status><quality>normal</quality><text-representation><language>eng</language><script>Latn</script></text-representation><artist-credit><name-credit><artist id="a74b1b7f-71a5-4011-9441-d0b5e4122711"><name>Radiohead</name><sort-name>Radiohead</sort-name></artist></name-credit></artist-credit><date>1992-09-21</date><country>GB</country><release-event-list count="1"><release-event><date>1992-09-21</date><area id="8a754a16-0027-3a29-b6d7-2b40ea0481ed"><name>United Kingdom</name><sort-name>United Kingdom</sort-name><iso-3166-1-code-list><iso-3166-1-code>GB</iso-3166-1-code></iso-3166-1-code-list></area></release-event></release-event-list><barcode>724388023429</barcode><asin>B000EHLKNU</asin><cover-art-archive><artwork>true</artwork><count>3</count><front>true</front><back>true</back></cover-art-archive><label-info-list count="1"><label-info><catalog-number>CDR 6078</catalog-number><label id="df7d1c7f-ef95-425f-8eef-445b3d7bcbd9"><name>Parlophone</name><sort-name>Parlophone</sort-name><label-code>299</label-code></label></label-info></label-info-list></release></metadata>"#;
        let context = ::util::musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let release = Release::from_xml(&reader).unwrap();

        assert_eq!(
            release.mbid,
            Mbid::from_str("ed118c5f-d940-4b52-a37b-b1a205374abe").unwrap()
        );
        assert_eq!(release.title, "Creep".to_string());
        assert_eq!(
            release.artists,
            vec![
                ArtistRef {
                    mbid: Mbid::from_str("a74b1b7f-71a5-4011-9441-d0b5e4122711").unwrap(),
                    name: "Radiohead".to_string(),
                    sort_name: "Radiohead".to_string(),
                },
            ]
        );
        assert_eq!(
            release.date,
            Some(PartialDate::from_str("1992-09-21").unwrap())
        );
        assert_eq!(release.country, Some("GB".to_string()));
        assert_eq!(
            release.labels,
            vec![
                LabelInfo {
                    label: Some(LabelRef {
                        mbid: Mbid::from_str("df7d1c7f-ef95-425f-8eef-445b3d7bcbd9").unwrap(),
                        name: "Parlophone".to_string(),
                        sort_name: "Parlophone".to_string(),
                        label_code: Some("299".to_string()),
                    }),
                    catalog_number: Some("CDR 6078".to_string()),
                },
            ]
        );
        assert_eq!(release.barcode, Some("724388023429".to_string()));
        assert_eq!(release.status, Some(ReleaseStatus::Official));
        assert_eq!(release.language, Some("eng".to_string()));
        assert_eq!(release.script, Some("Latn".to_string()));
        // TODO: check disambiguation
        // assert_eq!(release.disambiguation,
        assert_eq!(release.mediums, Vec::new());
    }

    #[test]
    fn release_read_xml2()
    {
        // url: https://musicbrainz.org/ws/2/release/785d7c67-a920-4cee-a871-8cd9896eb8aa?inc=aliases+artists+labels
        let context = ::util::musicbrainz_context();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><release id="785d7c67-a920-4cee-a871-8cd9896eb8aa"><title>The Fame</title><status id="4e304316-386d-3409-af2e-78857eec5cfe">Official</status><quality>normal</quality><packaging id="ec27701a-4a22-37f4-bfac-6616e0f9750a">Jewel Case</packaging><text-representation><language>eng</language><script>Latn</script></text-representation><artist-credit><name-credit><artist id="650e7db6-b795-4eb5-a702-5ea2fc46c848"><name>Lady Gaga</name><sort-name>Lady Gaga</sort-name><alias-list count="2"><alias sort-name="Lady Ga Ga">Lady Ga Ga</alias><alias sort-name="Germanotta, Stefani Joanne Angelina" type-id="d4dcd0c0-b341-3612-a332-c0ce797b25cf" type="Legal name">Stefani Joanne Angelina Germanotta</alias></alias-list></artist></name-credit></artist-credit><date>2008-08-19</date><country>CA</country><release-event-list count="1"><release-event><date>2008-08-19</date><area id="71bbafaa-e825-3e15-8ca9-017dcad1748b"><name>Canada</name><sort-name>Canada</sort-name><iso-3166-1-code-list><iso-3166-1-code>CA</iso-3166-1-code></iso-3166-1-code-list></area></release-event></release-event-list><barcode>602517664890</barcode><asin>B001D25N2Y</asin><cover-art-archive><artwork>true</artwork><count>1</count><front>true</front><back>false</back></cover-art-archive><label-info-list count="5"><label-info><catalog-number>0251766489</catalog-number><label id="376d9b4d-8cdd-44be-bc0f-ed5dfd2d2340"><name>Cherrytree Records</name><sort-name>Cherrytree Records</sort-name></label></label-info><label-info><catalog-number>0251766489</catalog-number><label id="2182a316-c4bd-4605-936a-5e2fac52bdd2"><name>Interscope Records</name><sort-name>Interscope Records</sort-name><label-code>6406</label-code><alias-list count="3"><alias sort-name="Flip/Interscope Records">Flip/Interscope Records</alias><alias sort-name="Interscape Records">Interscape Records</alias><alias sort-name="Nothing/Interscope">Nothing/Interscope</alias></alias-list></label></label-info><label-info><catalog-number>0251766489</catalog-number><label id="061587cb-0262-46bc-9427-cb5e177c36a2"><name>Konlive</name><sort-name>Konlive</sort-name><alias-list count="1"><alias sort-name="Kon Live">Kon Live</alias></alias-list></label></label-info><label-info><catalog-number>0251766489</catalog-number><label id="244dd29f-b999-40e4-8238-cb760ad05ac6"><name>Streamline Records</name><sort-name>Streamline Records</sort-name><disambiguation>Interscope imprint</disambiguation></label></label-info><label-info><catalog-number>0251766489</catalog-number><label id="6cee07d5-4cc3-4555-a629-480590e0bebd"><name>Universal Music Canada</name><sort-name>Universal Music Canada</sort-name><disambiguation>1995–</disambiguation><alias-list count="2"><alias sort-name="Universal Music (Canada)">Universal Music (Canada)</alias><alias sort-name="Universal Music Canada in.">Universal Music Canada in.</alias></alias-list></label></label-info></label-info-list></release></metadata>"#;
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let release = Release::from_xml(&reader).unwrap();

        // We check for the things we didn't check in the previous test.
        assert_eq!(release.packaging, Some("Jewel Case".to_string()));
        assert_eq!(
            release.labels,
            vec![
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
        assert_eq!(release.mediums, Vec::new());
    }

    #[test]
    fn read_tracks()
    {
        // url: https://musicbrainz.org/ws/2/release/d1881a4c-0188-4f0f-a2e7-4e7849aec109?inc=artists+labels+recordings
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><release id="d1881a4c-0188-4f0f-a2e7-4e7849aec109"><title>EXITIUM</title><status id="4e304316-386d-3409-af2e-78857eec5cfe">Official</status><quality>normal</quality><text-representation><language>jpn</language><script>Jpan</script></text-representation><artist-credit><name-credit><artist id="90e7c2f9-273b-4d6c-a662-ab2d73ea4b8e"><name>NECRONOMIDOL</name><sort-name>NECRONOMIDOL</sort-name></artist></name-credit></artist-credit><date>2015-10-04</date><country>JP</country><release-event-list count="1"><release-event><date>2015-10-04</date><area id="2db42837-c832-3c27-b4a3-08198f75693c"><name>Japan</name><sort-name>Japan</sort-name><iso-3166-1-code-list><iso-3166-1-code>JP</iso-3166-1-code></iso-3166-1-code-list></area></release-event></release-event-list><asin>B014GUVIM8</asin><cover-art-archive><artwork>false</artwork><count>0</count><front>false</front><back>false</back></cover-art-archive><label-info-list count="1"><label-info><label id="58592b07-de7e-4231-9b0b-4b9c9e1f3a03"><name>VELOCITRON</name><sort-name>VELOCITRON</sort-name></label></label-info></label-info-list><medium-list count="1"><medium><position>1</position><track-list offset="0" count="3"><track id="ac898be7-2965-4d17-9ac8-48d45852d73c"><position>1</position><number>1</number><title>puella tenebrarum</title><length>232000</length><recording id="fd6f4cd8-9cff-43da-8cd7-3351357b6f5a"><title>Puella Tenebrarum</title><length>232000</length></recording></track><track id="21648b0b-deaf-4b93-a257-5fc18363b25d"><position>2</position><number>2</number><title>LAMINA MALEDICTUM</title><length>258000</length><recording id="0eeb0621-8013-4c0e-8e49-ddfd78d56051"><title>Lamina Maledictum</title><length>258000</length></recording></track><track id="e57b3990-eb36-476e-beac-583e0bbe6f87"><position>3</position><number>3</number><title>SARNATH</title><length>228000</length><recording id="53f87e98-351e-453e-b949-bdacf4cbeccd"><title>Sarnath</title><length>228000</length></recording></track></track-list></medium></medium-list></release></metadata>"#;
        let context = ::util::musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let release = Release::from_xml(&reader).unwrap();

        let mediums = release.mediums;
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
    fn tracks_without_length()
    {
        // url: http://musicbrainz.org/ws/2/release/02173013-59ed-4229-b0a5-e5aa486ed5d7?inc=aliases+artists+labels+recordings
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><release id="02173013-59ed-4229-b0a5-e5aa486ed5d7"><title>蜃気楼 第二章</title><status id="4e304316-386d-3409-af2e-78857eec5cfe">Official</status><quality>normal</quality><text-representation><language>jpn</language><script>Jpan</script></text-representation><artist-credit><name-credit><artist id="477d67fd-65f3-4054-96a9-7117c8f0fec3"><name>童子-T</name><sort-name>Dohzi-T</sort-name><alias-list count="4"><alias sort-name="DOHZI-T">DOHZI-T</alias><alias sort-name="どうじティー">どうじティー</alias><alias locale="ja" sort-name="どうじT" type-id="894afba6-2816-3c24-8072-eadb66bd04bc" primary="primary" type="Artist name">童子-T</alias><alias type="Legal name" type-id="d4dcd0c0-b341-3612-a332-c0ce797b25cf" sort-name="たけすえみつる" locale="ja">竹末充</alias></alias-list></artist></name-credit></artist-credit><date>2004-06-09</date><country>JP</country><release-event-list count="1"><release-event><date>2004-06-09</date><area id="2db42837-c832-3c27-b4a3-08198f75693c"><name>Japan</name><sort-name>Japan</sort-name><iso-3166-1-code-list><iso-3166-1-code>JP</iso-3166-1-code></iso-3166-1-code-list></area></release-event></release-event-list><barcode>4988009031156</barcode><asin>B000EBCLLQ</asin><cover-art-archive><artwork>false</artwork><count>0</count><front>false</front><back>false</back></cover-art-archive><label-info-list count="1"><label-info><catalog-number>SRCL-6190</catalog-number><label id="ec7f9b15-9bc5-471e-ae8c-a35f0a968635"><name>Atomic Bomb</name><sort-name>Atomic Bomb</sort-name><disambiguation>Japanese imprint</disambiguation></label></label-info></label-info-list><medium-list count="1"><medium><position>1</position><format id="9712d52a-4509-3d4b-a1a2-67c88c643e31">CD</format><track-list offset="0" count="4"><track id="d0602913-34e8-406e-8071-ab4590796917"><position>1</position><number>1</number><recording id="ea93ce0e-2e9e-4727-8fee-e7774a8866ab"><title>蜃気楼 第二章</title></recording></track><track id="91f34974-612e-4b5f-9f24-07a35c50777e"><position>2</position><number>2</number><recording id="52fdd010-710f-424f-bd8d-7cad27a40044"><title>ファンファーレ</title></recording></track><track id="50d13b6e-0236-4242-aa53-9d35cfdbac11"><position>3</position><number>3</number><recording id="019ad40f-cf12-4023-b9eb-fa3ead48bf30"><title>蜃気楼 第二章 (Instrumental)</title></recording></track><track id="1eadfd07-4d24-4771-b48c-33e81e0214e8"><position>4</position><number>4</number><recording id="0ca76f38-0647-4fe2-bbff-4cee6c016a4d"><title>ファンファーレ (Instrumental)</title></recording></track></track-list></medium></medium-list></release></metadata>"#;

        let context = ::util::musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let release = Release::from_xml(&reader).unwrap();

        let ref medium = release.mediums[0];
        assert_eq!(medium.tracks[0].length, None);
        assert_eq!(medium.tracks[1].length, None);
        assert_eq!(medium.tracks[2].length, None);
        assert_eq!(medium.tracks[3].length, None);
    }

    #[test]
    fn multi_cd()
    {
        // url: https://musicbrainz.org/ws/2/release/ce22b20d-3a45-4e47-abaa-b7c8d10281fa?inc=artists+labels+recordings
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><release id="ce22b20d-3a45-4e47-abaa-b7c8d10281fa"><title>PINK</title><status id="4e304316-386d-3409-af2e-78857eec5cfe">Official</status><quality>normal</quality><disambiguation>deluxe edition</disambiguation><packaging id="ec27701a-4a22-37f4-bfac-6616e0f9750a">Jewel Case</packaging><text-representation><language>eng</language><script>Latn</script></text-representation><artist-credit><name-credit><artist id="57652bf8-cfe8-42e7-b9a7-5572a7080d8d"><name>Boris</name><sort-name>Boris</sort-name><disambiguation>Japanese stoner rock band</disambiguation></artist></name-credit></artist-credit><date>2016-07-08</date><country>US</country><release-event-list count="1"><release-event><date>2016-07-08</date><area id="489ce91b-6658-3307-9877-795b68554c98"><name>United States</name><sort-name>United States</sort-name><iso-3166-1-code-list><iso-3166-1-code>US</iso-3166-1-code></iso-3166-1-code-list></area></release-event></release-event-list><barcode>634457723825</barcode><asin>B01G5FGXKO</asin><cover-art-archive><artwork>false</artwork><count>0</count><front>false</front><back>false</back></cover-art-archive><label-info-list count="1"><label-info><catalog-number>SH-160</catalog-number><label id="8e3fe8a2-3c49-4ec1-8a1f-c31c2814611f"><name>Sargent House</name><sort-name>Sargent House</sort-name></label></label-info></label-info-list><medium-list count="2"><medium><title>PINK</title><position>1</position><format id="9712d52a-4509-3d4b-a1a2-67c88c643e31">CD</format><track-list offset="0" count="11"><track id="6274d68c-6d29-493f-88c9-4aec708069ce"><position>1</position><number>1</number><title>Farewell</title><length>453440</length><recording id="5ba6314d-a27f-43a6-8972-4c8b4f69315e"><title>決別</title><length>453400</length></recording></track><track id="5e67884f-1c13-4aa5-bb1a-caa0dbeabe42"><position>2</position><number>2</number><title>PINK</title><length>260027</length><recording id="4940d931-771d-4f5c-92cc-759124510ef3"><title>Pink</title><length>260040</length></recording></track><track id="c99f5afd-d2a3-40fa-9542-2598404e2f0a"><position>3</position><number>3</number><title>Woman on the Screen</title><length>158520</length><recording id="52ae10f9-b2c2-4222-b473-90dfc6969eef"><title>スクリーンの女</title><length>158520</length></recording></track><track id="2a657e73-6c55-43b8-8425-ba768e8eacc2"><position>4</position><number>4</number><title>Nothing Special</title><length>137920</length><recording id="d052834e-fba4-44f6-8950-e3fc36919f27"><title>別になんでもない</title><length>137920</length></recording></track><track id="971a34b3-e605-4a25-8736-8217da4b69c1"><position>5</position><number>5</number><title>Blackout</title><length>289680</length><recording id="2035f506-7c78-4af2-96f5-44600706e43b"><title>ブラックアウト</title><length>289680</length></recording></track><track id="cbf9e05f-28aa-4f9c-bdb2-d77be85d9a68"><position>6</position><number>6</number><length>105120</length><recording id="0dbf447d-dccb-4611-bf8c-ffd1f6b1a547"><title>Electric</title><length>105053</length></recording></track><track id="53fffbdf-d99e-491c-bbc2-30b76da427bd"><position>7</position><number>7</number><title>Pseudo Bread</title><length>269867</length><recording id="0cccd22c-b503-47d8-bb38-a9dfab973f24"><title>偽ブレッド</title><length>269867</length></recording></track><track id="d1762100-6008-47c3-9bf3-f6acfa072924"><position>8</position><number>8</number><title>Afterburner</title><length>262267</length><recording id="2dcb9772-99c9-495a-b7b6-791ef0844b52"><title>ぬるい炎</title><length>262267</length></recording></track><track id="76a521e6-73d6-47cb-a75a-fba751058a81"><position>9</position><number>9</number><title>Six, Three Times</title><length>173200</length><recording id="2fa562d2-2280-4f48-afa2-1a5267409c00"><title>6を3つ</title><length>173200</length></recording></track><track id="1b85a40e-03ae-431a-bfdf-6991327dfc74"><position>10</position><number>10</number><length>121493</length><recording id="b44b1ac2-feb0-425c-9bd7-b48879c7281b"><title>My Machine</title><length>121333</length></recording></track><track id="c8930110-bfcb-4991-9f3a-d9cc1e9b0d89"><position>11</position><number>11</number><title>Just Abandoned My-Self</title><length>1095770</length><recording id="0f11623c-a4f6-403b-9c72-8d74decf070a"><title>俺を捨てたところ</title><length>1094666</length></recording></track></track-list></medium><medium><title>PINK Sessions &quot;Forbidden Songs&quot;</title><position>2</position><format id="9712d52a-4509-3d4b-a1a2-67c88c643e31">CD</format><track-list offset="0" count="9"><track id="f7c6667a-46b6-4df5-9b0e-1702c80b3712"><position>1</position><number>1</number><length>375040</length><recording id="d69de264-f6e6-49a9-934a-79914c245263"><title>Your Name -Part 2-</title><length>375040</length></recording></track><track id="5d5450fe-92e5-435f-b845-a43c07508b34"><position>2</position><number>2</number><length>198560</length><recording id="986cfca3-9ac5-4249-afa0-6e40fa284ad6"><title>Heavy Rock Industry</title><length>198560</length></recording></track><track id="d2ae8b58-f020-4cdf-a5a3-d467cbc06821"><position>3</position><number>3</number><length>237386</length><recording id="a84f0c2c-af6e-4f4d-be22-4571a37280a2"><title>SOFUN</title><length>237386</length></recording></track><track id="739b61f3-9416-4899-8579-a06028ac80bc"><position>4</position><number>4</number><length>155240</length><recording id="a5147598-2347-4099-b3aa-0ed77e8d37be"><title>non/sha/lant</title><length>155240</length></recording></track><track id="1fd119c4-593f-4724-82ac-bd3b46aefeb8"><position>5</position><number>5</number><length>225066</length><recording id="8dba3582-ba98-4b38-a552-91280529faad"><title>Room Noise</title><length>225066</length></recording></track><track id="f9f564a5-92f1-4e00-845b-27b55ff2322b"><position>6</position><number>6</number><length>266106</length><recording id="3f2af038-4acb-4868-b38d-8599b1d5c09b"><title>Talisman</title><length>266106</length></recording></track><track id="a8c132ea-7a74-4c5f-8b18-724122901e6c"><position>7</position><number>7</number><length>470480</length><recording id="f8650ed2-7cd7-4616-9173-0a645ee250db"><title>N.F. Sorrow</title><length>470480</length></recording></track><track id="f3ae7bea-9bd0-4d13-b4fc-0fdb784117ae"><position>8</position><number>8</number><length>261826</length><recording id="67b98d9a-7549-4f29-a4b0-1a7f0312fff7"><title>Are You Ready?</title><length>261826</length></recording></track><track id="db700691-19c4-439a-a278-38b6b90c1c1c"><position>9</position><number>9</number><length>138373</length><recording id="e4356c5a-92bd-45a7-98ec-e23bf73ae1b1"><title>Tiptoe</title><length>138373</length></recording></track></track-list></medium></medium-list></release></metadata>"#;
        // TODO move long strings to external file as they break syntax highlighting
        // for me... "
        let context = ::util::musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let release = Release::from_xml(&reader).unwrap();
        let mediums = release.mediums;

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
    /// any label in the
    /// database.
    #[test]
    fn catalog_number_but_no_label_ref()
    {
        // url: http://musicbrainz.org/ws/2/release/61f8b05f-a3b5-49f4-a3a6-8f0d564c1664?inc=aliases+artists+labels+recordings
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><release id="61f8b05f-a3b5-49f4-a3a6-8f0d564c1664"><title>Love Somebody EP</title><status id="4e304316-386d-3409-af2e-78857eec5cfe">Official</status><quality>normal</quality><packaging id="119eba76-b343-3e02-a292-f0f00644bb9b">None</packaging><text-representation><language>eng</language></text-representation><artist-credit><name-credit><artist id="29042cf5-0583-45a6-b77c-ba2601008691"><name>Robyn Sherwell</name><sort-name>Sherwell, Robyn</sort-name></artist></name-credit></artist-credit><date>2014-09-07</date><country>XW</country><release-event-list count="1"><release-event><date>2014-09-07</date><area id="525d4e18-3d00-31b9-a58b-a146a916de8f"><name>[Worldwide]</name><sort-name>[Worldwide]</sort-name><iso-3166-1-code-list><iso-3166-1-code>XW</iso-3166-1-code></iso-3166-1-code-list></area></release-event></release-event-list><barcode>5065002081001</barcode><cover-art-archive><artwork>true</artwork><count>1</count><front>true</front><back>false</back></cover-art-archive><label-info-list count="1"><label-info><catalog-number>BIRD 4</catalog-number></label-info></label-info-list><medium-list count="1"><medium><position>1</position><track-list offset="0" count="4"><track id="0d0fea83-07c3-47b2-80ea-7d2dc1099e49"><position>1</position><number>1</number><length>249000</length><recording id="3a2de788-a07f-450f-baaf-b5e71182cd7c"><title>Love Somebody</title><length>249000</length></recording></track><track id="2f318359-3f93-4cc5-a095-f8971fbe7377"><position>2</position><number>2</number><length>211000</length><recording id="e3991f48-0ac0-4273-b45a-05aedf5ece22"><title>Low</title><length>211000</length></recording></track><track id="0f85ae3a-61f9-4aff-ae8f-87c06070d304"><position>3</position><number>3</number><length>353000</length><recording id="7a42290f-1031-4902-9a62-35fbdcbf45d6"><title>Love Somebody (Turtle remix)</title><length>353000</length></recording></track><track id="d1cb1d31-1a9e-4aa3-ad7a-d41fdaf81290"><position>4</position><number>4</number><length>274000</length><recording id="f6513a66-1217-4170-bf60-288f624011d0"><title>Love Somebody (Field Kit remix)</title><length>274000</length></recording></track></track-list></medium></medium-list></release></metadata>"#;

        let context = ::util::musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let release = Release::from_xml(&reader).unwrap();

        // TODO check extracted entities
    }
}
