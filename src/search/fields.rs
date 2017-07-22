//! The fields that can be used in queries.
//!
//! Some field types can be used for multiple entities, to make it more user
//! friendly the types are reexported to the submodules corresponding to the
//! names of the entities which they can be used to query.
//!
//! Link to [MusicBrainz
//! documentation](https://musicbrainz.org/doc/Indexed_Search_Syntax).

// TODO: To verify we actually implemented all fields it might make more sense to swap our type
// and the string value in the following declarations and then sort the entries alphabetically
// again.

use super::{full_entities};
// use super::query::QueryExpression;
use super::full_entities::{Mbid, PartialDate};
// use super::entities;

pub trait SearchField {
    type Value: ToString;

    fn to_string(&self) -> String;
}

macro_rules! define_fields {
    ( $( $(#[$attr:meta])* f, $type:ident, $value:ty );* ) => {
        $(
            $(#[$attr])*
            pub struct $type ( pub $value );

            impl SearchField for $type {
                type Value = $value;

                fn to_string(&self) -> String {
                    self.0.to_string()
                }
            }
        )*
    }
}

// TODO consider whether we should rename `Comment` to `Disambiguation` or something like that to
// be more consistent with the rest of the crate.
//
// TODO: enums for quality, lang, script, etc
// TODO it's a bit ugly we have f, at the beginning of every line but its a workaround around the
// parsing ambiguity we'd have if we don't.
define_fields!(
    /// Alias of the searched entity's name.
    f, Alias, String;
    f, AreaId, Mbid;
    f, AreaIso, String;
    f, AreaIso1, String;
    f, AreaIso2, String;
    f, AreaIso3, String;
    f, AreaName, String;
    f, AreaType, full_entities::AreaType;
    f, ArtistCredit, String;
    f, ArtistId, Mbid;
    f, ArtistName, String;
    f, ArtistNameAccent, String;
    f, ArtistType, full_entities::ArtistType;
    f, Asin, String;
    /// The barcode of a `Release`.
    f, Barcode, String;
    f, BeginArea, String;
    f, BeginDate, PartialDate;
    f, CatalogNumber, String;
    f, Comment, String;
    f, Country, String;
    f, CreditName, String;
    f, DataQuality, String;
    f, EndArea, String;
    f, EndDate, PartialDate;
    f, Ended, bool;
    /// The gender of an `Artist`.
    f, Gender, String;
    f, IpiCode, String;
    f, LabelId, String;
    f, Language, String;
    f, MediumCount, u32;
    f, MediumFormat, String;
    /// The searched entity's name. (TODO implement for all relevant searches)
    f, Name, String;
    f, NumDiscIds, u32;
    f, NumDiscIdsMedium, u32;
    f, NumTracks, u32;
    f, NumTracksMedium, u32;
    f, PrimaryType, full_entities::ReleaseGroupPrimaryType;
    f, ReleaseDate, full_entities::PartialDate;
    f, ReleaseGroupId, Mbid;
    f, ReleaseGroupName, String;
    f, ReleaseGroupNameAccent, String;
    f, ReleaseId, Mbid;
    /// The name of the `Release`, without special accent characters.
    f, ReleaseName, String;
    /// The name of the `Release`, including special accent characters.
    f, ReleaseNameAccent, String;
    f, ReleaseNumber, u16;
    f, ReleaseStatus, full_entities::ReleaseStatus;
    f, Script, String;
    f, SecondaryType, String;
    f, SortName, String;
    f, Tag, String
);

macro_rules! define_entity_fields {
    (
        $field_trait:ident, $modname:ident;
        $(
            $field_type:ident, $strname:expr
        );*
    )
        =>
    {
        /// Acceptable fields searching for instances of the entity.
        pub trait $field_trait : SearchField {
            fn name() -> &'static str;
        }

        pub mod $modname {
            pub use super::$field_trait;

            $(
                pub use super::$field_type;

                impl $field_trait for $field_type {
                    fn name() -> &'static str { $strname }
                }
            )*
        }

    }
}

define_entity_fields!(
    AreaSearchField, area;

    Alias, "alias";
    AreaId, "aid";
    AreaIso, "iso";
    AreaIso1, "iso1";
    AreaIso2, "iso2";
    AreaIso3, "iso3";
    AreaName, "area";
    AreaType, "type";
    BeginDate, "begin";
    Comment, "comment";
    EndDate, "end";
    Ended, "ended";
    SortName, "sortname"
);

define_entity_fields!(
    ArtistSearchField, artist;

    Alias, "alias";
    AreaName, "area";
    ArtistId, "arid";
    ArtistName, "artist";
    ArtistNameAccent, "artistaccent";
    ArtistType, "type";
    BeginArea, "beginarea";
    BeginDate, "begin";
    Comment, "comment";
    Country, "country";
    EndArea, "endarea";
    EndDate, "end";
    Ended, "ended";
    Gender, "gender";
    IpiCode, "ipi";
    SortName, "sortname";
    Tag, "tag"
);

// TODO what are puids?
define_entity_fields!(
    ReleaseSearchField, release;

    ArtistId, "arid";
    ArtistName, "artist";
    Asin, "asin";
    Barcode, "barcode";
    CatalogNumber, "catno";
    Comment, "comment";
    Country, "country";
    CreditName, "creditname";
    ReleaseDate, "date";
    NumDiscIds, "discids";
    NumDiscIdsMedium, "discidsmedium";
    MediumFormat, "format";
    LabelId, "laid";
    Language, "lang";
    MediumCount, "mediums";
    PrimaryType, "primarytype";
    DataQuality, "quality";
    ReleaseId, "reid";
    ReleaseName, "release";
    ReleaseNameAccent, "releaseaccent";
    ReleaseGroupId, "rgid";
    Script, "script";
    SecondaryType, "secondarytype";
    ReleaseStatus, "status";
    Tag, "tag";
    NumTracks, "tracks";
    NumTracksMedium, "tracksmedium"
);

define_entity_fields!(
    ReleaseGroupSearchField, release_group;

    ArtistCredit, "artist";
    ArtistId, "arid";
    ArtistName, "artistname";
    Comment, "comment";
    CreditName, "creditname";
    PrimaryType, "primarytype";
    ReleaseGroupId, "rgid";
    ReleaseGroupName, "releasegroup";
    ReleaseGroupNameAccent, "releasegroupaccent";
    ReleaseId, "reid";
    ReleaseName, "release";
    ReleaseNumber, "releases";
    ReleaseStatus, "status";
    SecondaryType, "secondarytype";
    Tag, "tag"
);
