use std::time::Duration;
use xpath_reader::{FromXml, Error, Reader};

use crate::entities::{Mbid, ResourceOld};
use crate::entities::refs::ArtistRef;

/// Represents a unique audio that has been used to produce at least one
/// released track through
/// copying or mastering.
#[derive(Clone, Debug)]
pub struct Recording {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    /// The title of the recording.
    pub title: String,

    /// The artists that the recording is primarily credited to.
    pub artists: Vec<ArtistRef>,

    /// Approximation of the length of the recording, calculated from the
    /// tracks using it.
    pub duration: Option<Duration>,

    /// ISRC (International Standard Recording Code) assigned to the recording.
    pub isrc_code: Option<String>,

    /// Disambiguation comment.
    pub disambiguation: Option<String>,

    /// Any additional free form annotation for this `Recording`.
    pub annotation: Option<String>,
}

impl FromXml for Recording {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, Error> {
        Ok(Recording {
            mbid: reader.read(".//mb:recording/@id")?,
            title: reader.read(".//mb:recording/mb:title/text()")?,
            artists: reader.read(".//mb:recording/mb:artist-credit/mb:name-credit")?,
            duration: crate::entities::helper::read_mb_duration(
                reader,
                ".//mb:recording/mb:length/text()",
            )?,
            isrc_code: reader.read(".//mb:recording/mb:isrc-list/mb:isrc/@id")?,
            disambiguation: reader.read(".//mb:recording/mb:disambiguation/text()")?,
            annotation: reader.read(".//mb:recording/mb:annotation/text()")?,
        })
    }
}

impl ResourceOld for Recording {
    const NAME: &'static str = "recording";
    const INCL: &'static str = "artists+annotation+isrcs";
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn read_xml1() {
        let mbid = Mbid::from_str("fbe3d0b9-3990-4a76-bddb-12f4a0447a2c").unwrap();
        let recording: Recording = crate::util::test_utils::fetch_entity(&mbid).unwrap();

        assert_eq!(recording.mbid, mbid);
        assert_eq!(
            recording.title,
            "The Perfect Drug (Nine Inch Nails)".to_string()
        );
        assert_eq!(recording.duration, Some(Duration::from_millis(499000)));
        assert_eq!(
            recording.artists,
            vec![ArtistRef {
                mbid: Mbid::from_str("b7ffd2af-418f-4be2-bdd1-22f8b48613da").unwrap(),
                name: "Nine Inch Nails".to_string(),
                sort_name: "Nine Inch Nails".to_string(),
            },]
        );
        assert_eq!(recording.isrc_code, Some("USIR19701296".to_string()));
        assert_eq!(recording.annotation, None);
        assert_eq!(recording.disambiguation, None);
    }
}
