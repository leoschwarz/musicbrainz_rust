use std::str::FromStr;
/// TODO consider what type to use
pub use std::time::Duration;

use super::{ParseError, ParseErrorKind};

use xpath_reader::{Context, FromXml, OptionFromXml, XpathError, XpathReader};
use xpath_reader::reader::{FromXmlContained, FromXmlElement};

mod date;
pub use self::date::{Date, ParseDateError};

pub mod refs;
pub use self::refs::{AreaRef, ArtistRef, LabelRef, RecordingRef, ReleaseRef};

#[cfg(test)]
use xpath_reader::XpathStrReader;

mod area;
mod artist;
mod event;
mod label;
mod recording;
mod release;
mod release_group;
pub use self::area::{Area, AreaType};
pub use self::artist::{Artist, ArtistType, Gender};
pub use self::event::{Event, EventType};
pub use self::label::Label;
pub use self::recording::Recording;
pub use self::release::{Release, ReleaseMedium, ReleaseStatus, ReleaseTrack};
pub use self::release_group::{ReleaseGroup, ReleaseGroupPrimaryType, ReleaseGroupSecondaryType,
                              ReleaseGroupType};

mod mbid;
pub use self::mbid::Mbid;


// TODO this is mostly a convenience thing that will have to be removed
// completely at a later
// point.
/// Don't use this outside this crate.
pub fn default_musicbrainz_context<'d>() -> Context<'d>
{
    let mut context = Context::default();
    context.set_namespace("mb", "http://musicbrainz.org/ns/mmd-2.0#");
    context
}

pub trait Resource {
    /// Returns the url where one can get a ressource in the valid format for
    /// parsing from.
    fn get_url(mbid: &Mbid) -> String;

    /// Base url of the entity, for example:
    /// `https://musicbrainz.org/ws/2/artist/`.
    /// These are used for searches for example.
    fn base_url() -> &'static str;
}

pub struct Instrument {}

pub struct Series {}

pub struct Work {}

pub struct Url {}

// TODO: rating, tag, collection
// TODO: discid, isrc, iswc
