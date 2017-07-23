//! The fields that can be used in queries.
//!
//! Some field types can be used for multiple entities, to make it more user
//! friendly the types are reexported to the submodules corresponding to the
//! names of the entities which they can be used to query.
//!
//! Link to [MusicBrainz
//! documentation](https://musicbrainz.org/doc/Indexed_Search_Syntax).

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
             $strname:expr, $field_type:ident
        );*
        ;
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

    "aid", AreaId;
    "alias", Alias;
    "area", AreaName;
    "begin", BeginDate;
    "comment", Comment;
    "end", EndDate;
    "ended", Ended;
    "iso", AreaIso;
    "iso1", AreaIso1;
    "iso2", AreaIso2;
    "iso3", AreaIso3;
    "sortname", SortName;
    "type", AreaType;
);

define_entity_fields!(
    ArtistSearchField, artist;

    "alias", Alias;
    "area", AreaName;
    "arid", ArtistId;
    "artist", ArtistName;
    "artistaccent", ArtistNameAccent;
    "begin", BeginDate;
    "beginarea", BeginArea;
    "comment", Comment;
    "country", Country;
    "end", EndDate;
    "endarea", EndArea;
    "ended", Ended;
    "gender", Gender;
    "ipi", IpiCode;
    "sortname", SortName;
    "tag", Tag;
    "type", ArtistType;
);

// TODO what are puids?
define_entity_fields!(
    ReleaseSearchField, release;

    "arid", ArtistId;
    "artist", ArtistName;
    "asin", Asin;
    "barcode", Barcode;
    "catno", CatalogNumber;
    "comment", Comment;
    "country", Country;
    "creditname", CreditName;
    "date", ReleaseDate;
    "discids", NumDiscIds;
    "discidsmedium", NumDiscIdsMedium;
    "format", MediumFormat;
    "laid", LabelId;
    "lang", Language;
    "mediums", MediumCount;
    "primarytype", PrimaryType;
    "quality", DataQuality;
    "reid", ReleaseId;
    "release", ReleaseName;
    "releaseaccent", ReleaseNameAccent;
    "rgid", ReleaseGroupId;
    "script", Script;
    "secondarytype", SecondaryType;
    "status", ReleaseStatus;
    "tag", Tag;
    "tracks", NumTracks;
    "tracksmedium", NumTracksMedium;
);

define_entity_fields!(
    ReleaseGroupSearchField, release_group;

    "arid", ArtistId;
    "artist", ArtistCredit;
    "artistname", ArtistName;
    "comment", Comment;
    "creditname", CreditName;
    "primarytype", PrimaryType;
    "reid", ReleaseId;
    "release", ReleaseName;
    "releasegroup", ReleaseGroupName;
    "releasegroupaccent", ReleaseGroupNameAccent;
    "releases", ReleaseNumber;
    "rgid", ReleaseGroupId;
    "secondarytype", SecondaryType;
    "status", ReleaseStatus;
    "tag", Tag;
);
