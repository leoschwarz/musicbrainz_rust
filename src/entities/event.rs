use xpath_reader::{FromXml, FromXmlError, XpathReader};
use xpath_reader::reader::{FromXmlContained, FromXmlElement};

use entities::{Mbid, Resource};
use entities::date::PartialDate;

enum_mb_xml! {
    pub enum EventType {
        var Concert = "Concert",
        var Festival = "Festival",
        var LaunchEvent = "Launch event",
        var ConventionExpo = "Convention/Expo",
        var MasterclassClinic = "Masterclass/Clinic",
    }
}

pub struct Event {
    /// MBID of the entity in the MusicBrainz database.
    mbid: Mbid,

    /// The official name of the event or a descriptive name if the event
    /// doesn't have an official
    /// name.
    name: String,

    /// Aternative event names.
    aliases: Vec<String>,

    /// Describes what type of event this is exactly.
    event_type: Option<EventType>,

    /// List of songs played at the event.
    ///
    /// This is provided in an extensive text format, for which parsing is not
    /// yet implemented.
    setlist: Option<String>,

    /// Begin date of the event.
    begin_date: PartialDate,

    /// End date of the event.
    end_date: PartialDate,

    // TODO:    start_time: Time
    /// Disambiguation to distinguish Event from other Events with the same
    /// name (if existent).
    disambiguation: Option<String>,

    /// Additional, unstructured information about the event.
    annotation: Option<String>,
}

impl Resource for Event {
    fn get_name() -> &'static str
    {
        "Event"
    }

    fn get_url(mbid: &Mbid) -> String
    {
        format!(
            "https://musicbrainz.org/ws/2/event/{}?inc=aliases+annotation",
            mbid
        )
    }

    fn base_url() -> &'static str
    {
        "https://musicbrainz.org/ws/2/event/"
    }
}

impl FromXmlContained for Event {}
impl FromXml for Event {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, FromXmlError>
    where
        R: XpathReader<'d>,
    {
        Ok(Event {
            mbid: reader.read(".//mb:event/@id")?,
            name: reader.read(".//mb:event/mb:name")?,
            aliases: reader.read_vec(".//mb:event/mb:alias-list/mb:alias/text()")?,
            event_type: reader.read_option(".//mb:event/@type")?,
            setlist: reader.read_option(".//mb:event/mb:setlist")?,
            begin_date: reader.read(".//mb:event/mb:life-span/mb:begin")?,
            end_date: reader.read(".//mb:event/mb:life-span/mb:end")?,
            disambiguation: reader.read_option(".//mb:event/mb:disambiguation")?,
            annotation: reader.read_option(".//mb:event/mb:annotation/mb:text/text()")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn read_1()
    {
        let mbid = Mbid::from_str("6e2ab7d5-f340-4c41-99a3-c901733402b4").unwrap();
        let event: Event = ::util::test_utils::fetch_entity(&mbid).unwrap();

        assert_eq!(event.mbid, mbid);
        assert_eq!(event.name, "25. Wave-Gotik-Treffen".to_string());
        assert_eq!(event.aliases, vec!["WGT 2016".to_string()]);
        assert_eq!(event.event_type, Some(EventType::Festival));
        assert_eq!(event.setlist, None);
        assert_eq!(event.begin_date, "2016-05-13".parse().unwrap());
        assert_eq!(event.end_date, "2016-05-16".parse().unwrap());
        assert_eq!(event.disambiguation, None);
        assert_eq!(event.annotation.unwrap().len(), 2233);
    }

    #[test]
    fn read_2()
    {
        let mbid = Mbid::from_str("9754f4dd-6fad-49b7-8f30-940c9af6b776").unwrap();
        let event: Event = ::util::test_utils::fetch_entity(&mbid).unwrap();

        assert_eq!(event.event_type, Some(EventType::Concert));
        assert_eq!(event.setlist.unwrap().len(), 225);
    }
}
