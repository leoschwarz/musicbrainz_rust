use entities::{Mbid, PartialDate, Resource};
use entities::refs::AreaRef;
use xpath_reader::{FromXml, FromXmlError, XpathReader};
use xpath_reader::reader::{FromXmlContained, FromXmlElement};

enum_mb_xml! {
    /// Specifies what a `Place` instance actually is.
    pub enum PlaceType {
        var Studio = "Studio",
        var Venue = "Venue",
        var Stadium = "Stadium",
        var IndoorArena = "Indoor arena",
        var ReligiousBuilding = "Religious building",
        var Other = "Other",
    }
}

/// A pair of coordinates on the surface of planet earth.
///
/// TODO: Parsing of the coordinate values, currently they are only unchecked
/// string values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

impl FromXmlElement for Coordinates {}
impl FromXml for Coordinates {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(Coordinates {
            latitude: reader.read(".//mb:latitude/text()")?,
            longitude: reader.read(".//mb:longitude/text()")?,
        })
    }
}

/// A venue, studio or other place where music is performed, recorded,
/// engineered, etc.
///
/// Additional information can be found in the [MusicBrainz
/// docs](https://musicbrainz.org/doc/Place).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Place {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    /// The official name of a `Place`.
    pub name: String,

    /// The type of the `Place`.
    pub place_type: Option<PlaceType>,

    /// Address of the `Place` in the local adressing format.
    pub address: Option<String>,

    /// The exact coordinates of the place.
    pub coordinates: Option<Coordinates>,

    /// Specifies the `Area` the `Place` is located in.
    pub area: Option<AreaRef>,

    /// When the `Place` was founded.
    pub begin: Option<PartialDate>,

    /// When the `Place` closed down.
    pub end: Option<PartialDate>,

    /// Alternative versions of this `Place`'s name.
    pub aliases: Vec<String>,

    /// Additional disambiguation if there are multiple places with the same
    /// name.
    pub disambiguation: Option<String>,

    /// Any additional free form annotation for this `Place`.
    pub annotation: Option<String>,
}

impl FromXmlContained for Place {}
impl FromXml for Place {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(Place {
            address: reader.read_option(".//mb:place/mb:address/text()")?,
            aliases: reader.read_vec(".//mb:place/mb:aliases/text()")?,
            annotation: reader.read_option(".//mb:place/mb:annotation/text()")?,
            area: reader.read_option(".//mb:place/mb:area")?,
            begin: reader.read_option(".//mb:place/mb:life-span/mb:begin/text()")?,
            coordinates: reader.read_option(".//mb:place/mb:coordinates")?,
            disambiguation: reader.read_option(".//mb:place/mb:disambiguation/text()")?,
            end: reader.read_option(".//mb:place/mb:life-span/mb:end/text()")?,
            mbid: reader.read(".//mb:place/@id")?,
            name: reader.read(".//mb:place/mb:name/text()")?,
            place_type: reader.read_option(".//mb:place/@type")?,
        })
    }
}

impl Resource for Place {
    fn get_name() -> &'static str
    {
        "place"
    }

    fn get_incs() -> &'static str
    {
        "annotation+aliases"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn place_read_1()
    {
        let mbid = Mbid::from_str("d1ab65f8-d082-492a-bd70-ce375548dabf").unwrap();
        let p: Place = ::util::test_utils::fetch_entity(&mbid).unwrap();

        // Check parsed values.
        assert_eq!(p.mbid, mbid);
        assert_eq!(p.name, "Chipping Norton Recording Studios".to_string());
        assert_eq!(p.place_type, Some(PlaceType::Studio));
        assert_eq!(
            p.address,
            Some("28–30 New Street, Chipping Norton".to_string())
        );
        assert_eq!(
            p.coordinates,
            Some(Coordinates {
                latitude: "51.9414".to_string(),
                longitude: "-1.548".to_string(),
            })
        );
        assert_eq!(
            p.area,
            Some(AreaRef {
                mbid: Mbid::from_str("716234d3-b8ed-45ac-8983-e7219eb85956").unwrap(),
                name: "Chipping Norton".to_string(),
                sort_name: "Chipping Norton".to_string(),
                iso_3166: None,
            })
        );
        assert_eq!(p.begin, PartialDate::from_str("1971").ok());
        assert_eq!(p.end, PartialDate::from_str("1999-10").ok());
        assert_eq!(p.aliases, Vec::<String>::new());
        assert_eq!(p.disambiguation, None);
        assert_eq!(p.annotation, None);
    }

    // TODO more expansive example testing all fields
}
