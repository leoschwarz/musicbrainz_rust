use super::*;
use std::fmt::{self, Display};

/// The primary type of a release group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReleaseGroupPrimaryType {
    Album,
    Single,
    EP,
    Broadcast,
    Other,
}

// TODO: Fix this in `xpath_reader`.
// impl FromXmlElement for ReleaseGroupPrimaryType {}
impl OptionFromXml for ReleaseGroupPrimaryType {
    fn option_from_xml<'d, R>(reader: &'d R) -> Result<Option<Self>, XpathError>
        where R: XpathReader<'d>
    {
        let s = String::from_xml(reader)?;
        match s.as_str() {
            "Album" => Ok(Some(ReleaseGroupPrimaryType::Album)),
            "Single" => Ok(Some(ReleaseGroupPrimaryType::Single)),
            "EP" => Ok(Some(ReleaseGroupPrimaryType::EP)),
            "Broadcast" => Ok(Some(ReleaseGroupPrimaryType::Broadcast)),
            "Other" => Ok(Some(ReleaseGroupPrimaryType::Other)),
            "" => Ok(None),
            s => Err(format!("Unknown ReleaseGroupPrimaryType: '{}'", s).into()),
        }
    }
}

impl Display for ReleaseGroupPrimaryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        use self::ReleaseGroupPrimaryType::*;
        let s = match *self {
            Album => "Album",
            Single => "Single",
            EP => "EP",
            Broadcast => "Broadcast",
            Other => "Other",
        };
        write!(f, "{}", s)
    }
}

/// Secondary types of a release group. There can be any number of secondary
/// types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReleaseGroupSecondaryType {
    Compilation,
    Soundtrack,
    Spokenword,
    Interview,
    Audiobook,
    Live,
    Remix,
    DjMix,
    MixtapeStreet,
}

impl FromXmlElement for ReleaseGroupSecondaryType {}
impl FromXml for ReleaseGroupSecondaryType {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, XpathError>
        where R: XpathReader<'d>
    {
        let s = String::from_xml(reader)?;
        match s.as_str() {
            "Compilation" => Ok(ReleaseGroupSecondaryType::Compilation),
            "Soundtrack" => Ok(ReleaseGroupSecondaryType::Soundtrack),
            "Spokenword" => Ok(ReleaseGroupSecondaryType::Spokenword),
            "Interview" => Ok(ReleaseGroupSecondaryType::Interview),
            "Audiobook" => Ok(ReleaseGroupSecondaryType::Audiobook),
            "Live" => Ok(ReleaseGroupSecondaryType::Live),
            "Remix" => Ok(ReleaseGroupSecondaryType::Remix),
            "DJ-mix" => Ok(ReleaseGroupSecondaryType::DjMix),
            "Mixtape/Street" => Ok(ReleaseGroupSecondaryType::MixtapeStreet),
            s => Err(format!("Unknown ReleaseSecondaryPrimaryType: '{}'", s).into()),
        }
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

impl FromXmlElement for ReleaseGroupType {}
impl FromXml for ReleaseGroupType {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, XpathError>
        where R: XpathReader<'d>
    {
        Ok(ReleaseGroupType {
               primary: reader.read_option(".//mb:primary-type/text()")?,
               secondary: reader.read_vec(".//mb:secondary-type-list/mb:secondary-type/text()")?,
           })
    }
}

/// Groups multiple `Release`s into one a single logical entity.
/// Even if there is only one release of a kind, it belongs to exactly one
/// release group.
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

    // TODO docstring
    pub disambiguation: Option<String>,

    // TODO: docstring
    pub annotation: Option<String>,
}

impl Resource for ReleaseGroup {
    fn get_url(mbid: &Mbid) -> String
    {
        format!("https://musicbrainz.org/ws/2/release-group/{}?inc=annotation+artists+releases",
                mbid)
    }

    fn base_url() -> &'static str
    {
        "https://musicbrainz.org/ws/2/release-group/"
    }
}

impl FromXmlContained for ReleaseGroup {}
impl FromXml for ReleaseGroup {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, XpathError>
        where R: XpathReader<'d>
    {
        Ok(ReleaseGroup {
               mbid: reader.read(".//mb:release-group/@id")?,
               title: reader.read(".//mb:release-group/mb:title/text()")?,
               releases: reader.read_vec(".//mb:release-group/mb:release-list/mb:release")?,
               artists:
                   reader.read_vec(".//mb:release-group/mb:artist-credit/mb:name-credit/mb:artist")?,
               release_type: reader.read(".//mb:release-group")?,
               disambiguation: reader.read_option(".//mb:release-group/mb:disambiguation/text()")?,
               annotation: reader.read_option(".//mb:release-group/mb:annotation/text()")?,
           })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_1()
    {
        // url: https://musicbrainz.org/ws/2/release-group/76a4e2c2-bf7a-445e-8081-5a1e291f3b16?inc=annotation+artists+releases
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><release-group type="Album" id="76a4e2c2-bf7a-445e-8081-5a1e291f3b16" type-id="f529b476-6e62-324f-b0aa-1f3e33d313fc"><title>Mixtape</title><first-release-date>2012-03</first-release-date><primary-type id="f529b476-6e62-324f-b0aa-1f3e33d313fc">Album</primary-type><secondary-type-list><secondary-type id="15c1b1f5-d893-3375-a1db-e180c5ae15ed">Mixtape/Street</secondary-type></secondary-type-list><artist-credit><name-credit><artist id="0e6b3a2c-6a42-4b43-a4f6-c6625c5855de"><name>POP ETC</name><sort-name>POP ETC</sort-name></artist></name-credit></artist-credit><release-list count="1"><release id="289bf4e7-0af5-433c-b5a2-493b863b4b47"><title>Mixtape</title><status id="4e304316-386d-3409-af2e-78857eec5cfe">Official</status><quality>normal</quality><text-representation><language>eng</language><script>Latn</script></text-representation><date>2012-03</date><country>US</country><release-event-list count="1"><release-event><date>2012-03</date><area id="489ce91b-6658-3307-9877-795b68554c98"><name>United States</name><sort-name>United States</sort-name><iso-3166-1-code-list><iso-3166-1-code>US</iso-3166-1-code></iso-3166-1-code-list></area></release-event></release-event-list></release></release-list></release-group></metadata>"#;
        let context = default_musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let rg = ReleaseGroup::from_xml(&reader).unwrap();

        assert_eq!(rg.mbid,
                   Mbid::from_str("76a4e2c2-bf7a-445e-8081-5a1e291f3b16").unwrap());
        assert_eq!(rg.title, "Mixtape".to_string());
        assert_eq!(rg.artists,
                   vec![
            ArtistRef {
                mbid: Mbid::from_str("0e6b3a2c-6a42-4b43-a4f6-c6625c5855de").unwrap(),
                name: "POP ETC".to_string(),
                sort_name: "POP ETC".to_string(),
            },
        ]);
        assert_eq!(rg.releases,
                   vec![
            ReleaseRef {
                mbid: Mbid::from_str("289bf4e7-0af5-433c-b5a2-493b863b4b47").unwrap(),
                title: "Mixtape".to_string(),
                date: Some(Date::Month {
                               year: 2012,
                               month: 03,
                           }),
                status: ReleaseStatus::Official,
                country: Some("US".to_string()),
            },
        ]);
        assert_eq!(rg.release_type.primary,
                   Some(ReleaseGroupPrimaryType::Album));
        assert_eq!(rg.release_type.secondary,
                   vec![ReleaseGroupSecondaryType::MixtapeStreet]);
        assert_eq!(rg.disambiguation, None);
        assert_eq!(rg.annotation, None);
    }
}
