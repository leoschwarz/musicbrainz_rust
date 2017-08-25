use xpath_reader::{FromXml, FromXmlError, XpathReader};
use xpath_reader::reader::{FromXmlContained, FromXmlElement};

use entities::{Mbid, Resource};
use entities::date::PartialDate;
use entities::refs::AreaRef;

enum_mb_xml! {
    /// Specification of the gender of an artist.
    pub enum Gender {
        var Female = "Female",
        var Male = "Male",
        var Other = "Other",
    }
}

enum_mb_xml! {
    /// Specifies what an `Artist` instance actually is.
    pub enum ArtistType {
        var Person = "Person",
        var Group = "Group",
        var Orchestra = "Orchestra",
        var Choir = "Choir",
        var Character = "Character",
        var Other = "Other",
    }
}

/// A musician, a group or another music professional.
///
/// There are also a couple special purpose artists.
///
/// Additional information can be found in the [MusicBrainz
/// docs](https://musicbrainz.org/doc/Artist).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Artist {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    /// The official name of the artist.
    pub name: String,

    /// Name to properly sort the artist by.
    ///
    /// Even for artists whose `name` is written in a different script this one
    /// will be in latin script. The full
    /// [guidelines](https://musicbrainz.org/doc/Style/Artist/Sort_Name) are a
    /// bit more complicated.
    pub sort_name: String,

    /// Aliases of the `Artist`'s name. These include alternative official
    /// spellings, common misspellings, versions in different scripts and
    /// other variations of the `Artist` name.
    pub aliases: Vec<String>,

    /// Any additional free form annotation for this `Artist`.
    ///
    /// This can include things like biographies, descriptions of their musical
    /// style, etc.
    pub annotation: Option<String>,

    /// Additional disambiguation if there are multiple `Artist`s with the same
    /// name.
    pub disambiguation: Option<String>,

    /// Whether this `Artist` is a person, group, or something else.
    pub artist_type: Option<ArtistType>,

    /// If the `Artist` is a single person this indicates their gender.
    pub gender: Option<Gender>,

    /// The area an `Artist` is primarily identified with. Often, but not
    /// always, birth/formation country of the artist/group.
    pub area: Option<AreaRef>,

    /// For a single person: date of birth.
    ///
    /// For a group of people: formation date.
    pub begin_date: Option<PartialDate>,

    /// For a deceased person: date of death.
    ///
    /// For a group of people: dissolution date.
    pub end_date: Option<PartialDate>,

    /// [IPI Code](https://wiki.musicbrainz.org/IPI) of the `Artist`.
    pub ipi_code: Option<String>,

    /// [ISNI Code](https://wiki.musicbrainz.org/ISNI) of the `Artist`.
    pub isni_code: Option<String>,
}

impl FromXmlContained for Artist {}
impl FromXml for Artist {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(Artist {
            aliases: reader.read_vec(".//mb:artist/mb:alias-list/mb:alias/text()")?,
            annotation: reader.read_option(".//mb:artist/mb:annotation/text()")?,
            area: reader.read_option(".//mb:artist/mb:area")?,
            artist_type: reader.read_option(".//mb:artist/@type")?,
            begin_date: reader.read_option(".//mb:artist/mb:life-span/mb:begin/text()")?,
            disambiguation: reader.read_option(".//mb:artist/mb:disambiguation/text()")?,
            end_date: reader.read_option(".//mb:artist/mb:life-span/mb:end/text()")?,
            gender: reader.read_option(".//mb:artist/mb:gender/text()")?,
            ipi_code: reader.read_option(".//mb:artist/mb:ipi/text()")?,
            isni_code: reader.read_option(".//mb:artist/mb:isni-list/mb:isni/text()")?,
            mbid: reader.read(".//mb:artist/@id")?,
            name: reader.read(".//mb:artist/mb:name/text()")?,
            sort_name: reader.read(".//mb:artist/mb:sort-name/text()")?,
        })
    }
}

impl Resource for Artist {
    fn get_name() -> &'static str
    {
        "artist"
    }

    fn get_incs() -> &'static str
    {
        "aliases"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn artist_read_xml1()
    {

        let mbid = Mbid::from_str("90e7c2f9-273b-4d6c-a662-ab2d73ea4b8e").unwrap();
        let artist: Artist = ::util::test_utils::fetch_entity(&mbid).unwrap();

        assert_eq!(artist.mbid, mbid);
        assert_eq!(artist.name, "NECRONOMIDOL".to_string());
        assert_eq!(artist.sort_name, "NECRONOMIDOL".to_string());
        assert_eq!(artist.aliases, Vec::<String>::new());

        assert_eq!(
            artist.begin_date,
            Some(PartialDate::from_str("2014-03").unwrap())
        );
        assert_eq!(artist.end_date, None);

        let area = artist.area.unwrap();
        assert_eq!(
            area.mbid,
            Mbid::from_str("2db42837-c832-3c27-b4a3-08198f75693c").unwrap()
        );
        assert_eq!(area.name, "Japan".to_string());
        assert_eq!(area.sort_name, "Japan".to_string());
        assert_eq!(area.iso_3166, Some("JP".to_string()));

        assert_eq!(artist.artist_type, Some(ArtistType::Group));
        assert_eq!(artist.gender, None);
        assert_eq!(artist.ipi_code, None);
        assert_eq!(artist.isni_code, None);
    }

    #[test]
    fn artist_read_xml2()
    {
        let mbid = Mbid::from_str("650e7db6-b795-4eb5-a702-5ea2fc46c848").unwrap();
        let artist: Artist = ::util::test_utils::fetch_entity(&mbid).unwrap();

        assert_eq!(artist.mbid, mbid);
        assert_eq!(artist.name, "Lady Gaga".to_string());
        assert_eq!(artist.sort_name, "Lady Gaga".to_string());
        let mut aliases_sorted = artist.aliases.clone();
        aliases_sorted.sort();
        assert_eq!(
            aliases_sorted,
            vec![
                "Lady Ga Ga".to_string(),
                "Stefani Joanne Angelina Germanotta".to_string(),
            ]
        );

        assert_eq!(
            artist.begin_date,
            Some(PartialDate::from_str("1986-03-28").unwrap())
        );
        assert_eq!(artist.end_date, None);

        let area = artist.area.unwrap();
        assert_eq!(
            area.mbid,
            Mbid::from_str("489ce91b-6658-3307-9877-795b68554c98").unwrap()
        );
        assert_eq!(area.name, "United States".to_string());
        assert_eq!(area.sort_name, "United States".to_string());
        assert_eq!(area.iso_3166, Some("US".to_string()));

        assert_eq!(artist.artist_type, Some(ArtistType::Person));
        assert_eq!(artist.gender, Some(Gender::Female));
        assert_eq!(artist.ipi_code, Some("00519338344".to_string()));
        assert_eq!(artist.isni_code, Some("0000000120254559".to_string()));
    }

}
