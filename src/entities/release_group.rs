use xpath_reader::{FromXml, FromXmlOptional, Error, Reader};

use crate::entities::{Mbid, ResourceOld};
use crate::entities::refs::{ArtistRef, ReleaseRef};

enum_mb_xml_optional! {
    /// The primary type of a release group.
    pub enum ReleaseGroupPrimaryType {
        var Album = "Album",
        var Single = "Single",
        var EP = "EP",
        var Broadcast = "Broadcast",
        var Other = "Other",
    }
}

enum_mb_xml_optional! {
    /// Secondary types of a release group. There can be any number of secondary
    /// types.
    pub enum ReleaseGroupSecondaryType {
        var Compilation = "Compilation",
        var Soundtrack = "Soundtrack",
        var Spokenword = "Spokenword",
        var Interview = "Interview",
        var Audiobook = "Audiobook",
        var Live = "Live",
        var Remix = "Remix",
        var DjMix = "DJ-mix",
        var MixtapeStreet = "Mixtape/Street",
    }
}

/// The type of a `ReleaseGroup`.
///
/// For more information consult: https://musicbrainz.org/doc/Release_Group/Type
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseGroupType {
    pub primary: Option<ReleaseGroupPrimaryType>,
    pub secondary: Vec<ReleaseGroupSecondaryType>,
}

impl FromXml for ReleaseGroupType {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, Error> {
        Ok(ReleaseGroupType {
            primary: reader.read(".//mb:primary-type/text()")?,
            secondary: reader.read(".//mb:secondary-type-list/mb:secondary-type/text()")?,
        })
    }
}

/// Groups multiple `Release`s into one a single logical entity.
///
/// Even if there is only one `Release` of a kind, it belongs to exactly one
/// `ReleaseGroup`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseGroup {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    /// Title of the release group, usually the same as the title of the
    /// releases.
    pub title: String,

    /// The artists of a release group.
    pub artists: Vec<ArtistRef>,

    /// Releases of this releaes group.
    pub releases: Vec<ReleaseRef>,

    /// The type of this release group.
    pub release_type: ReleaseGroupType,

    /// Additional disambiguation if there are multiple `ReleaseGroup`s with
    /// the same name.
    pub disambiguation: Option<String>,

    /// Any additional free form annotation for this `ReleaseGroup`.
    pub annotation: Option<String>,
}

impl ResourceOld for ReleaseGroup {
    const NAME: &'static str = "release-group";
    const INCL: &'static str = "annotation+artists+releases";
}

impl FromXml for ReleaseGroup {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, Error> {
        Ok(ReleaseGroup {
            mbid: reader.read(".//mb:release-group/@id")?,
            title: reader.read(".//mb:release-group/mb:title/text()")?,
            releases: reader.read(".//mb:release-group/mb:release-list/mb:release")?,
            artists: reader
                .read(".//mb:release-group/mb:artist-credit/mb:name-credit/mb:artist")?,
            release_type: reader.read(".//mb:release-group")?,
            disambiguation: reader.read(".//mb:release-group/mb:disambiguation/text()")?,
            annotation: reader.read(".//mb:release-group/mb:annotation/text()")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use crate::entities::*;

    #[test]
    fn read_1() {
        let mbid = Mbid::from_str("76a4e2c2-bf7a-445e-8081-5a1e291f3b16").unwrap();
        let rg: ReleaseGroup = crate::util::test_utils::fetch_entity(&mbid).unwrap();

        assert_eq!(rg.mbid, mbid);
        assert_eq!(rg.title, "Mixtape".to_string());
        assert_eq!(
            rg.artists,
            vec![ArtistRef {
                mbid: Mbid::from_str("0e6b3a2c-6a42-4b43-a4f6-c6625c5855de").unwrap(),
                name: "POP ETC".to_string(),
                sort_name: "POP ETC".to_string(),
            },]
        );
        assert_eq!(
            rg.releases,
            vec![ReleaseRef {
                mbid: Mbid::from_str("289bf4e7-0af5-433c-b5a2-493b863b4b47").unwrap(),
                title: "Mixtape".to_string(),
                date: Some(PartialDate::from_str("2012-03").unwrap()),
                status: Some(ReleaseStatus::Official),
                country: Some("US".to_string()),
            },]
        );
        assert_eq!(
            rg.release_type.primary,
            Some(ReleaseGroupPrimaryType::Album)
        );
        assert_eq!(
            rg.release_type.secondary,
            vec![ReleaseGroupSecondaryType::MixtapeStreet]
        );
        assert_eq!(rg.disambiguation, None);
        assert_eq!(rg.annotation, None);
    }
}
