/// TODO consider what type to use
pub use std::time::Duration;

mod date;
pub use self::date::{Date, ParseDateError};

pub mod refs;
pub use self::refs::{AreaRef, ArtistRef, LabelRef, RecordingRef, ReleaseRef};

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
