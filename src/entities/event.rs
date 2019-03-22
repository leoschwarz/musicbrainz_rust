use xpath_reader::{FromXml, FromXmlOptional, Error, Reader};

use crate::entities::{Mbid, Resource};
use crate::entities::date::PartialDate;

enum_mb_xml_optional! {
    pub enum EventType {
        var Concert = "Concert",
        var Festival = "Festival",
        var LaunchEvent = "Launch event",
        var ConventionExpo = "Convention/Expo",
        var MasterclassClinic = "Masterclass/Clinic",
    }
}

/// An organized event people can attend, these are generally live performances.
///
/// Additional information can be found in the [MusicBrainz
/// docs](https://musicbrainz.org/doc/Event)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    /// The official name of the event or a descriptive name if the event
    /// doesn't have an official name.
    pub name: String,

    /// Aternative event names.
    pub aliases: Vec<String>,

    /// Describes what type of event this is exactly.
    pub event_type: Option<EventType>,

    /// List of songs played at the event.
    ///
    /// This is provided in an extensive text format, for which parsing is not
    /// yet implemented.
    pub setlist: Option<String>,

    /// Begin date of the event.
    pub begin_date: PartialDate,

    /// End date of the event.
    pub end_date: Option<PartialDate>,

    /// Additional disambiguation if there are multiple `Event`s with the same
    /// name.
    pub disambiguation: Option<String>,

    /// Any additional free form annotation for this `Event`.
    pub annotation: Option<String>,
}

impl Resource for Event {
    const NAME: &'static str = "event";
    const INCL: &'static str = "aliases+annotation";
}

impl FromXml for Event {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, Error> {
        Ok(Event {
            mbid: reader.read(".//mb:event/@id")?,
            name: reader.read(".//mb:event/mb:name")?,
            aliases: reader.read(".//mb:event/mb:alias-list/mb:alias/text()")?,
            event_type: reader.read(".//mb:event/@type")?,
            setlist: reader.read(".//mb:event/mb:setlist")?,
            begin_date: reader.read(".//mb:event/mb:life-span/mb:begin")?,
            end_date: reader.read(".//mb:event/mb:life-span/mb:end")?,
            disambiguation: reader.read(".//mb:event/mb:disambiguation")?,
            annotation: reader.read(".//mb:event/mb:annotation/mb:text/text()")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn read_1() {
        let mbid = Mbid::from_str("6e2ab7d5-f340-4c41-99a3-c901733402b4").unwrap();
        let event: Event = crate::util::test_utils::fetch_entity(&mbid).unwrap();

        assert_eq!(event.mbid, mbid);
        assert_eq!(event.name, "25. Wave-Gotik-Treffen".to_string());
        assert_eq!(event.aliases, vec!["WGT 2016".to_string()]);
        assert_eq!(event.event_type, Some(EventType::Festival));
        assert_eq!(event.setlist, None);
        assert_eq!(event.begin_date, "2016-05-13".parse().unwrap());
        assert_eq!(event.end_date.unwrap(), "2016-05-16".parse().unwrap());
        assert_eq!(event.disambiguation, None);
        assert_eq!(event.annotation.unwrap().len(), 2233);
    }

    #[test]
    fn read_2() {
        let mbid = Mbid::from_str("9754f4dd-6fad-49b7-8f30-940c9af6b776").unwrap();
        let event: Event = crate::util::test_utils::fetch_entity(&mbid).unwrap();

        assert_eq!(event.event_type, Some(EventType::Concert));
        assert_eq!(event.setlist.unwrap().len(), 225);
    }
}
