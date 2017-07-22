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
    ( $( $type:ident, $value:ty );* ) => {
        $(
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
define_fields!(
    Alias, String;
    AreaId, Mbid;
    AreaIso, String;
    AreaIso1, String;
    AreaIso2, String;
    AreaIso3, String;
    AreaName, String;
    AreaType, full_entities::AreaType;
    ArtistCredit, String;
    ArtistId, Mbid;
    ArtistName, String;
    ArtistNameAccent, String;
    ArtistType, full_entities::ArtistType;
    Asin, String;
    Barcode, String;
    BeginArea, String;
    BeginDate, PartialDate;
    CatalogNumber, String;
    Comment, String;
    Country, String;
    CreditName, String;
    DataQuality, String;
    EndArea, String;
    EndDate, PartialDate;
    Ended, bool;
    Gender, String;
    IpiCode, String;
    LabelId, String;
    Language, String;
    MediumCount, u32;
    MediumFormat, String;
    NumDiscIds, u32;
    NumDiscIdsMedium, u32;
    NumTracks, u32;
    NumTracksMedium, u32;
    PrimaryType, full_entities::ReleaseGroupPrimaryType;
    ReleaseDate, full_entities::PartialDate;
    ReleaseGroupId, Mbid;
    ReleaseGroupName, String;
    ReleaseGroupNameAccent, String;
    ReleaseId, Mbid;
    ReleaseName, String;
    ReleaseNameAccent, String;
    ReleaseNumber, u16;
    ReleaseStatus, full_entities::ReleaseStatus;
    Script, String;
    SecondaryType, String;
    SortName, String;
    Tag, String
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
