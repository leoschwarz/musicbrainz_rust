use xpath_reader::{FromXml, FromXmlOptional, Error, Reader};

use crate::entities::{Mbid, ResourceOld, OnRequest, Alias, Resource};
use crate::entities::date::PartialDate;
use crate::entities::refs::AreaRef;
use crate::client::Request;

enum_mb_xml_optional! {
    /// Specification of the gender of an artist.
    pub enum Gender {
        var Female = "Female",
        var Male = "Male",
        var Other = "Other",
    }
}

enum_mb_xml_optional! {
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
#[derive(Clone, Debug)]
pub struct Artist {
    response: ArtistResponse,
    options: ArtistOptions,
}

#[derive(Clone, Debug)]
pub struct ArtistOptions {
    pub annotation: bool,
    pub aliases: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtistResponse {
    mbid: Mbid,
    name: String,
    sort_name: String,
    aliases: Vec<Alias>,
    annotation: Option<String>,
    disambiguation: Option<String>,
    artist_type: Option<ArtistType>,
    gender: Option<Gender>,
    area: Option<AreaRef>,
    begin_date: Option<PartialDate>,
    end_date: Option<PartialDate>,
    ipi_code: Option<String>,
    isni_code: Option<String>,
}

impl Artist {
    /// MBID of the artist in the MusicBrainz database.
    pub fn mbid(&self) -> &Mbid {
        &self.response.mbid
    }

    /// The official name of the artist.
    pub fn name(&self) -> &String {
        &self.response.name
    }

    /// Name to properly sort the artist by.
    ///
    /// Even for artists whose `name` is written in a different script this one
    /// will be in latin script. The full
    /// [guidelines](https://musicbrainz.org/doc/Style/Artist/Sort_Name) are a
    /// bit more complicated.
    pub fn sort_name(&self) -> &String {
        &self.response.sort_name
    }

    /// Aliases of the `Artist`'s name. These include alternative official
    /// spellings, common misspellings, versions in different scripts and
    /// other variations of the `Artist` name.
    pub fn aliases(&self) -> OnRequest<&[Alias]> {
        if self.options.aliases {
            OnRequest::Some(self.response.aliases.as_ref())
        } else {
            OnRequest::NotRequested
        }
    }

    /// Any additional free form annotation for this `Artist`.
    ///
    /// This can include things like biographies, descriptions of their musical
    /// style, etc.
    pub fn annotation(&self) -> OnRequest<&String> {
        OnRequest::from_option(self.response.annotation.as_ref(), self.options.annotation)
    }

    /// Additional disambiguation if there are multiple `Artist`s with the same
    /// name.
    pub fn disambiguation(&self) -> Option<&String> {
        self.response.disambiguation.as_ref()
    }

    /// Whether this `Artist` is a person, group, or something else.
    pub fn artist_type(&self) -> Option<ArtistType> {
        self.response.artist_type.clone()
    }

    /// If the `Artist` is a single person this indicates their gender.
    pub fn gender(&self) -> Option<Gender> {
        self.response.gender.clone()
    }

    /// The area an `Artist` is primarily identified with. Often, but not
    /// always, birth/formation country of the artist/group.
    pub fn area(&self) -> Option<&AreaRef> {
        self.response.area.as_ref()
    }

    /// For a single person: date of birth.
    ///
    /// For a group of people: formation date.
    pub fn begin_date(&self) -> Option<&PartialDate> {
        self.response.begin_date.as_ref()
    }

    /// For a deceased person: date of death.
    ///
    /// For a group of people: dissolution date.
    pub fn end_date(&self) -> Option<&PartialDate> {
        self.response.end_date.as_ref()
    }

    /// [IPI Code](https://wiki.musicbrainz.org/IPI) of the `Artist`.
    pub fn ipi_code(&self) -> Option<&String> {
        self.response.ipi_code.as_ref()
    }

    /// [ISNI Code](https://wiki.musicbrainz.org/ISNI) of the `Artist`.
    pub fn isni_code(&self) -> Option<&String> {
        self.response.isni_code.as_ref()
    }
}

impl ArtistOptions {
    pub fn everything() -> Self {
        ArtistOptions {
            annotation: true,
            aliases: true,
        }
    }

    pub fn minimal() -> Self {
        ArtistOptions {
            annotation: false,
            aliases: false,
        }
    }
}

impl FromXml for ArtistResponse {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, Error> {
        Ok(ArtistResponse {
            aliases: reader.read(".//mb:artist/mb:alias-list/mb:alias")?,
            annotation: reader.read(".//mb:artist/mb:annotation/text()")?,
            area: reader.read(".//mb:artist/mb:area")?,
            artist_type: reader.read(".//mb:artist/@type")?,
            begin_date: reader.read(".//mb:artist/mb:life-span/mb:begin/text()")?,
            disambiguation: reader.read(".//mb:artist/mb:disambiguation/text()")?,
            end_date: reader.read(".//mb:artist/mb:life-span/mb:end/text()")?,
            gender: reader.read(".//mb:artist/mb:gender/text()")?,
            ipi_code: reader.read(".//mb:artist/mb:ipi/text()")?,
            isni_code: reader.read(".//mb:artist/mb:isni-list/mb:isni/text()")?,
            mbid: reader.read(".//mb:artist/@id")?,
            name: reader.read(".//mb:artist/mb:name/text()")?,
            sort_name: reader.read(".//mb:artist/mb:sort-name/text()")?,
        })
    }
}

impl Resource for Artist {
    type Options = ArtistOptions;
    type Response = ArtistResponse;
    const NAME: &'static str = "artist";

    fn request(options: &Self::Options) -> Request {
        let mut includes = Vec::new();

        if options.aliases {
            includes.push("aliases");
        }
        if options.annotation {
            includes.push("annotation");
        }

        Request {
            name: "artist".into(),
            include: includes.join("+"),
        }
    }

    fn from_response(response: Self::Response, options: Self::Options) -> Self {
        Artist { response, options }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use std::iter::FromIterator;
    use crate::entities::{AliasType, Language};

    #[test]
    fn artist_read_xml1() {
        let mbid = Mbid::from_str("90e7c2f9-273b-4d6c-a662-ab2d73ea4b8e").unwrap();
        let options = ArtistOptions::minimal();
        let artist: Artist = crate::util::test_utils::fetch_entity(&mbid, options).unwrap();

        assert_eq!(artist.mbid(), &mbid);
        assert_eq!(artist.name(), &"NECRONOMIDOL".to_string());
        assert_eq!(artist.sort_name(), &"NECRONOMIDOL".to_string());
        assert_eq!(artist.aliases(), OnRequest::NotRequested);

        assert_eq!(
            artist.begin_date(),
            Some(&PartialDate::from_str("2014-03").unwrap())
        );
        assert_eq!(artist.end_date(), None);

        let area = artist.area().unwrap();
        assert_eq!(
            area.mbid,
            Mbid::from_str("2db42837-c832-3c27-b4a3-08198f75693c").unwrap()
        );
        assert_eq!(area.name, "Japan".to_string());
        assert_eq!(area.sort_name, "Japan".to_string());
        assert_eq!(area.iso_3166, Some("JP".to_string()));

        assert_eq!(artist.artist_type(), Some(ArtistType::Group));
        assert_eq!(artist.gender(), None);
        assert_eq!(artist.ipi_code(), None);
        assert_eq!(artist.isni_code(), None);
    }

    #[test]
    fn artist_read_xml2() {
        let mbid = Mbid::from_str("650e7db6-b795-4eb5-a702-5ea2fc46c848").unwrap();
        let options = ArtistOptions::everything();
        let artist: Artist = crate::util::test_utils::fetch_entity(&mbid, options).unwrap();

        assert_eq!(artist.mbid(), &mbid);
        assert_eq!(artist.name(), &"Lady Gaga".to_string());
        assert_eq!(artist.sort_name(), &"Lady Gaga".to_string());
        let mut aliases_sorted = Vec::from_iter(artist.aliases().unwrap().iter());
        aliases_sorted.sort_by(|a, b| a.name().cmp(b.name()));
        assert_eq!(
            aliases_sorted,
            vec![
                &Alias {
                    alias_type: None,
                    name: "Lady Ga Ga".into(),
                    sort_name: Some("Lady Ga Ga".into()),
                    locale: None,
                    primary: false
                },
                &Alias {
                    alias_type: Some(AliasType::LegalName),
                    name: "Stefani Joanne Angelina Germanotta".into(),
                    sort_name: Some("Germanotta, Stefani Joanne Angelina".into()),
                    locale: None,
                    primary: false
                },
                &Alias {
                    alias_type: Some(AliasType::ArtistName),
                    name: "レディー・ガガ".into(),
                    sort_name: Some("レディー・ガガ".into()),
                    locale: Some(Language::from_639_3("jpn").unwrap()),
                    primary: true,
                }
            ]
        );

        assert_eq!(
            artist.begin_date(),
            Some(&PartialDate::from_str("1986-03-28").unwrap())
        );
        assert_eq!(artist.end_date(), None);

        let area = artist.area().unwrap();
        assert_eq!(
            area.mbid,
            Mbid::from_str("489ce91b-6658-3307-9877-795b68554c98").unwrap()
        );
        assert_eq!(area.name, "United States".to_string());
        assert_eq!(area.sort_name, "United States".to_string());
        assert_eq!(area.iso_3166, Some("US".to_string()));

        assert_eq!(artist.artist_type(), Some(ArtistType::Person));
        assert_eq!(artist.gender(), Some(Gender::Female));
        assert_eq!(artist.ipi_code(), Some(&"00519338344".to_string()));
        assert_eq!(artist.isni_code(), Some(&"0000000120254559".to_string()));
    }

}
