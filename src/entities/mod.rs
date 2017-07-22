//! Defines types representing the entities from the MusicBrainz database.

/// TODO consider what type to use
pub use std::time::Duration;

#[macro_use]
mod helper;

mod date;
pub use self::date::{FullDate, ParseDateError, PartialDate};

pub mod refs;
pub use self::refs::{AreaRef, ArtistRef, LabelRef, RecordingRef, ReleaseRef};

mod area;
mod artist;
mod event;
mod label;
// mod medium;
mod place;
mod recording;
mod release;
mod release_group;
mod series;
// mod track
// mod url
// mod work
pub use self::area::{Area, AreaType};
pub use self::artist::{Artist, ArtistType, Gender};
pub use self::event::{Event, EventType};
pub use self::label::Label;
pub use self::place::{Coordinates, Place, PlaceType};
pub use self::recording::Recording;
pub use self::release::{LabelInfo, Release, ReleaseMedium, ReleaseStatus, ReleaseTrack};
pub use self::release_group::{ReleaseGroup, ReleaseGroupPrimaryType, ReleaseGroupSecondaryType,
                              ReleaseGroupType};
pub use self::series::Series;

mod mbid;
pub use self::mbid::Mbid;

// TODO: Convert get_name and base_url into associated consts once these land
// in stable rust.
/// A Resource is any entity which can be directly retrieved from MusicBrainz.
///
/// We define this trait for the sake of using the `Client` type more
/// efficiently, users of the `musicbrainz` crate shouldn't need to use this
/// type directly.
pub trait Resource {
    /// Returns the name of the Resource, e. g. `"Artist"`.
    fn get_name() -> &'static str;

    /// Returns the url where one can get a resource in the valid format for
    /// parsing from.
    fn get_url(mbid: &Mbid) -> String;

    /// Base url of the entity, e. g. `"https://musicbrainz.org/ws/2/artist/"`.
    ///
    /// These are used for building search requests.
    fn base_url() -> &'static str;
}

// TODO pub struct Work {}

// TODO pub struct Url {}

// TODO: rating, tag, collection
// TODO: discid, isrc, iswc
