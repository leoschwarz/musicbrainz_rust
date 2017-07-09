use entities::{PartialDate, Mbid, Resource};
use entities::refs::AreaRef;
use xpath_reader::{FromXml, FromXmlError, XpathReader};
use xpath_reader::reader::{FromXmlContained, FromXmlElement};

enum_mb_xml! {
    pub enum SeriesType {
        var ReleaseGroup = "Release group",
        var Release = "Release",
        var Recording = "Recording",
        var Work = "Work",
        var Catalogue = "Catalogue",
        var Event = "Event",
        var Tour = "Tour",
        var Festival = "Festival",
        var Run = "Run",
    }
}

/// TODO: Can't we read some of the relationships? Like this this is a rather
/// useless type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Series {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    /// Type of the series.
    pub series_type: SeriesType,

    pub aliases: Vec<String>,

    pub disambiguation: Option<String>,

    pub annotation: Option<String>,
}

impl FromXmlContained for Series {}
impl FromXml for Series {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(Series {
            mbid: reader.read(".//mb:series/@id")?,
            series_type: reader.read(".//mb:series/@type")?,
            aliases: reader.read_vec(".//mb:series/mb:alias-list/mb:alias/text()")?,
            disambiguation: reader.read_option(".//mb:series/mb:disambiguation/text()")?,
            annotation: reader.read_option(".//mb:series/mb:annotation/text()")?,
        })
    }
}

impl Resource for Series {
    fn get_url(mbid: &Mbid) -> String
    {
        format!(
            "https://musicbrainz.org/ws/2/series/{}?inc=annotation+aliases",
            mbid
        )
    }

    fn base_url() -> &'static str
    {
        "https://musicbrainz.org/ws/2/series/"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use xpath_reader::XpathStrReader;

    #[test]
    fn read_series_1()
    {
        // url: https://musicbrainz.org/ws/2/series/d977f7fd-96c9-4e3e-83b5-eb484a9e6582?inc=annotation+aliases
        let series: Series = ::util::extract_entity(r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><series id="d977f7fd-96c9-4e3e-83b5-eb484a9e6582" type="Catalogue" type-id="49482ff0-fc9e-3b8c-a2d0-30e84d9df002"><name>Bach-Werke-Verzeichnis</name><alias-list count="1"><alias sort-name="BWV">BWV</alias></alias-list></series></metadata>"#);

        assert_eq!(
            series.mbid,
            Mbid::from_str("d977f7fd-96c9-4e3e-83b5-eb484a9e6582").unwrap()
        );
        assert_eq!(series.series_type, SeriesType::Catalogue);
        assert_eq!(series.aliases, vec!["BWV".to_string()]);
        assert_eq!(series.disambiguation, None);
        assert_eq!(series.annotation, None);
    }
}
