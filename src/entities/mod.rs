//! Defines types representing the entities from the MusicBrainz database.

pub use std::time::Duration;

#[macro_use]
mod helper;

mod date;
pub use self::date::{FullDate, ParseDateError, PartialDate};

mod lang;
pub use self::lang::Language;

pub mod refs;
pub use self::refs::{AreaRef, ArtistRef, LabelRef, RecordingRef, ReleaseRef, FetchFull};

mod area;
mod artist;
mod event;
mod label;
// mod medium;
mod place;
mod recording;
mod release;
pub mod release_2;
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
// TODO it's pretty useless as of now.
// pub use self::series::Series;

use std::marker::PhantomData;

mod mbid;
pub use self::mbid::Mbid;

/// Represents an instance of an entity from the database.
///
/// Along with the data of the entity this can also optionally hold
/// relationship data from the database.
pub struct Entity<E> {
    /// The actual data of the entity.
    pub data: E,

    /// The relationship data of the entity.
    pub rels: Vec<Relationship<E>>,
}

pub struct Relationship<E> {
    _e: PhantomData<E>,
}

/// A Resource is any entity which can be directly retrieved from MusicBrainz.
///
/// We define this trait for the sake of using the `Client` type more
/// efficiently, users of the `musicbrainz` crate shouldn't need to use this
/// type directly.
pub trait Resource {
    /// Name of the resource for inclusion in api paths, e.g. `artist`.
    const NAME: &'static str;
    /// Query string component of includes to be requested by default.
    const INCL: &'static str;

    /// Returns the url where one can get a resource in the valid format for
    /// parsing from.
    fn get_url(mbid: &Mbid) -> String {
        format!(
            "https://musicbrainz.org/ws/2/{}/{}?inc={}",
            Self::NAME,
            mbid,
            Self::INCL
        )
    }
}

// TODO pub struct Work {}

// TODO pub struct Url {}

// TODO: rating, tag, collection
// TODO: discid, isrc, iswc
