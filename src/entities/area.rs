use super::*;
use std::fmt;

/// Specifies what a specific `Area` instance actually is.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AreaType {
    /// Areas included (or previously included) in ISO 3166-1.
    Country,

    /// Main administrative divisions of a countryr
    Subdivision,

    /// Smaller administrative divisions of a country, which are not one of the
    /// main administrative
    /// divisions but are also not muncipalities.
    County,

    /// Small administrative divisions. Urban muncipalities often contain only
    /// a single city and a
    /// few surrounding villages, while rural muncipalities often group several
    /// villages together.
    Muncipality,

    /// Settlements of any size, including towns and villages.
    City,

    /// Used for a division of a large city.
    District,

    /// Islands and atolls which don't form subdivisions of their own.
    Island,
}

impl FromXmlElement for AreaType {}
impl FromXml for AreaType {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, XpathError>
        where R: XpathReader<'d>
    {
        let s = String::from_xml(reader)?;
        match &s[..] {
            "Country" => Ok(AreaType::Country),
            "Subdivision" => Ok(AreaType::Subdivision),
            "County" => Ok(AreaType::County),
            "Muncipality" => Ok(AreaType::Muncipality),
            "City" => Ok(AreaType::City),
            "District" => Ok(AreaType::District),
            "Island" => Ok(AreaType::Island),
            s => Err(format!("Invalid `AreaType`: '{}'", s).into()),
        }
    }
}

impl fmt::Display for AreaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            AreaType::Country => write!(f, "Country"),
            AreaType::Subdivision => write!(f, "Subdivision"),
            AreaType::County => write!(f, "County"),
            AreaType::Muncipality => write!(f, "Muncipality"),
            AreaType::City => write!(f, "City"),
            AreaType::District => write!(f, "District"),
            AreaType::Island => write!(f, "Island"),
        }
    }
}

/// A geographic region or settlement.
/// The exact type is distinguished by the `area_type` field.
/// This is one of the *core entities* of MusicBrainz.
///
/// https://musicbrainz.org/doc/Area
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Area {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    /// The name of the area.
    pub name: String,

    /// Name that is supposed to be used for sorting, containing only latin
    /// characters.
    pub sort_name: String,

    /// The type of the area.
    pub area_type: AreaType,

    /// ISO 3166 code, assigned to countries and subdivisions.
    pub iso_3166: Option<String>,
}

impl FromXmlContained for Area {}
impl FromXml for Area {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Area, XpathError>
        where R: XpathReader<'d>
    {
        Ok(Area {
               mbid: reader.read(".//mb:area/@id")?,
               name: reader.read(".//mb:area/mb:name/text()")?,
               sort_name: reader.read(".//mb:area/mb:sort-name/text()")?,
               area_type: reader.read(".//mb:area/@type")?,
               iso_3166:
                   reader
                       .read_option(".//mb:area/mb:iso-3166-1-code-list/mb:iso-3166-1-code/text()")?,
           })
    }
}

impl Resource for Area {
    fn get_url(mbid: &Mbid) -> String
    {
        format!("https://musicbrainz.org/ws/2/area/{}", mbid)
    }

    fn base_url() -> &'static str
    {
        "https://musicbrainz.org/ws/2/area/"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_read_xml1()
    {
        // url: https://musicbrainz.org/ws/2/area/a1411661-be21-4290-8dc1-50f3d8e3ea67
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><area type="City" type-id="6fd8f29a-3d0a-32fc-980d-ea697b69da78" id="a1411661-be21-4290-8dc1-50f3d8e3ea67"><name>Honolulu</name><sort-name>Honolulu</sort-name></area></metadata>"#;
        let context = default_musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let result = Area::from_xml(&reader).unwrap();

        assert_eq!(result.mbid,
                   Mbid::from_str("a1411661-be21-4290-8dc1-50f3d8e3ea67").unwrap());
        assert_eq!(result.name, "Honolulu".to_string());
        assert_eq!(result.sort_name, "Honolulu".to_string());
        assert_eq!(result.area_type, AreaType::City);
        assert_eq!(result.iso_3166, None);
    }

    #[test]
    fn area_read_xml2()
    {
        // url: https://musicbrainz.org/ws/2/area/2db42837-c832-3c27-b4a3-08198f75693c
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><area type="Country" id="2db42837-c832-3c27-b4a3-08198f75693c" type-id="06dd0ae4-8c74-30bb-b43d-95dcedf961de"><name>Japan</name><sort-name>Japan</sort-name><iso-3166-1-code-list><iso-3166-1-code>JP</iso-3166-1-code></iso-3166-1-code-list></area></metadata>"#;
        let context = default_musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let result = Area::from_xml(&reader).unwrap();

        assert_eq!(result.mbid,
                   Mbid::from_str("2db42837-c832-3c27-b4a3-08198f75693c").unwrap());
        assert_eq!(result.name, "Japan".to_string());
        assert_eq!(result.sort_name, "Japan".to_string());
        assert_eq!(result.area_type, AreaType::Country);
        assert_eq!(result.iso_3166, Some("JP".to_string()));
    }
}
