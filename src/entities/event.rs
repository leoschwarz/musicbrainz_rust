use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    Concert,
    Festival,
    LaunchEvent,
    ConventionExpo,
    MasterclassClinic,
}

impl FromXmlElement for EventType {}
impl FromXml for EventType {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, XpathError>
        where R: XpathReader<'d>
    {
        let s = String::from_xml(reader)?;
        match s.as_str() {
            "Concert" => Ok(EventType::Concert),
            "Festival" => Ok(EventType::Festival),
            "Launch event" => Ok(EventType::LaunchEvent),
            "Convention/Expo" => Ok(EventType::ConventionExpo),
            "Masterclass/Clinic" => Ok(EventType::MasterclassClinic),
            s => Err(format!("Unknown `EventType`: {}", s))?,
        }
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
    event_type: EventType,

    /// List of songs played at the event.
    ///
    /// This is provided in an extensive text format, for which parsing is not
    /// yet implemented.
    setlist: Option<String>,

    /// Begin date of the event.
    begin_date: Date,

    /// End date of the event.
    end_date: Date,

    // TODO:    start_time: Time
    /// Disambiguation to distinguish Event from other Events with the same
    /// name (if existent).
    disambiguation: Option<String>,

    /// Additional, unstructured information about the event.
    annotation: Option<String>,
}

impl Resource for Event {
    fn get_url(mbid: &Mbid) -> String
    {
        format!("https://musicbrainz.org/ws/2/event/{}?inc=aliases+annotation",
                mbid)
    }

    fn base_url() -> &'static str
    {
        "https://musicbrainz.org/ws/2/event/"
    }
}

impl FromXmlContained for Event {}
impl FromXml for Event {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, XpathError>
        where R: XpathReader<'d>
    {
        Ok(Event {
               mbid: reader.read(".//mb:event/@id")?,
               name: reader.read(".//mb:event/mb:name")?,
               aliases: reader.read_vec(".//mb:event/mb:alias-list/mb:alias/text()")?,
               event_type: reader.read(".//mb:event/@type")?,
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

    #[test]
    fn read_1()
    {
        // url: https://musicbrainz.
        // org/ws/2/event/6e2ab7d5-f340-4c41-99a3-c901733402b4?inc=annotation+aliases
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><event type="Festival" id="6e2ab7d5-f340-4c41-99a3-c901733402b4" type-id="b6ded574-b592-3f0e-b56e-5b5f06aa0678"><name>25. Wave-Gotik-Treffen</name><life-span><begin>2016-05-13</begin><end>2016-05-16</end></life-span><annotation><text>ANNOTATION</text></annotation><alias-list count="1"><alias sort-name="WGT 2016">WGT 2016</alias></alias-list></event></metadata>"#;
        let context = default_musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();

        let event = Event::from_xml(&reader).unwrap();

        assert_eq!(event.mbid,
                   Mbid::from_str("6e2ab7d5-f340-4c41-99a3-c901733402b4").unwrap());
        assert_eq!(event.name, "25. Wave-Gotik-Treffen".to_string());
        assert_eq!(event.aliases, vec!["WGT 2016".to_string()]);
        assert_eq!(event.event_type, EventType::Festival);
        assert_eq!(event.setlist, None);
        assert_eq!(event.begin_date, "2016-05-13".parse().unwrap());
        assert_eq!(event.end_date, "2016-05-16".parse().unwrap());
        assert_eq!(event.disambiguation, None);
        assert_eq!(event.annotation, Some("ANNOTATION".to_string()));
    }

    #[test]
    fn read_2()
    {
        // url: https://musicbrainz.org/ws/2/event/9754f4dd-6fad-49b7-8f30-940c9af6b776
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><event type="Concert" id="9754f4dd-6fad-49b7-8f30-940c9af6b776" type-id="ef55e8d7-3d00-394a-8012-f5506a29ff0b"><name>Lady Gaga at Roseland Ballroom</name><life-span><begin>2014-03-28</begin><end>2014-03-28</end></life-span><setlist>* &quot;Born This Way&quot; (Piano Version)
* &quot;Black Jesus + Amen Fashion&quot;
* &quot;Monster&quot;
* &quot;Bad Romance&quot;
* &quot;Sexxx Dreams&quot;
* &quot;Dope&quot;
* &quot;You and I&quot;
* &quot;Just Dance&quot;
* &quot;Poker Face&quot; (Piano Version)
* &quot;Artpop&quot; (Interlude)
* &quot;Applause&quot;
* &quot;G.U.Y.&quot;</setlist></event></metadata>"#;
        let context = default_musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();

        let event = Event::from_xml(&reader).unwrap();
        assert_eq!(event.event_type, EventType::Concert);
        assert_eq!(event.setlist.unwrap().len(), 225);
    }
}
