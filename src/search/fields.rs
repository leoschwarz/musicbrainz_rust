//! The fields that can be used in queries.
//!
//! Some field types can be used for multiple entities, to make it more user
//! friendly the types are reexported to the submodules corresponding to the
//! names of the entities which they can be used to query.
//!
//! Link to [MusicBrainz
//! documentation](https://musicbrainz.org/doc/Indexed_Search_Syntax).

use crate::error::{Error, ErrorKind};
use crate::entities as full_entities;
use crate::entities::{Mbid, PartialDate, Resource};
use std::fmt;

fn wrong_search_field(entity: &str, field: &str) -> Error {
    Error::new(
        format!(
            "Requested field {} of entity {}, but this is not provided.",
            field, entity
        ),
        ErrorKind::UsageError,
    )
}

pub trait SearchField {
    type Value;

    fn name<R: Resource>(&self) -> Result<&'static str, Error>;
    fn value(&self) -> String;

    fn to_string<R: Resource>(&self) -> Result<String, Error>;
}

pub struct Alias(String);

impl SearchField for Alias {
    type Value = String;

    fn name<R: Resource>(&self) -> Result<&'static str, Error> {
        match R::NAME {
            "area" | "artist" => Ok("alias"),
            s => Err(wrong_search_field(R::NAME, "Alias")),
        }
    }

    fn value(&self) -> String {
        self.0.clone()
    }

    fn to_string<R: Resource>(&self) -> Result<String, Error> {
        Ok(format!("{}:{}", self.name::<R>()?, self.value()))
    }
}

/*
macro_rules! define_fields {
    ( $( $(#[$attr:meta])* - $type:ident, $value:ty );* ) => {
        $(
            $(#[$attr])*
            pub struct $type ( pub $value );

            impl SearchField for $type {
                type Value = $value;

                fn value(&self) -> String {
                    self.0.to_string()
                }
            }
        )*
    }
}

// TODO consider whether we should rename `Comment` to `Disambiguation` or
// something like that to
// be more consistent with the rest of the crate.
//
// TODO: enums for quality, script, etc
// TODO it's a bit ugly we have `-` at the beginning of every line but its a
// workaround around the parsing ambiguity we'd have if we didn't.
define_fields!(
    /// Alias of the searched entity's name.
    - Alias, String;
    /// The MBID of the `Area`.
    - AreaMbid, Mbid;
    /// An ISO 3166-1/2/3 code attached to the `Area`.
    - AreaIso, String;
    /// An ISO 3166-1 code attached to the `Area`.
    - AreaIso1, String;
    /// An ISO 3166-2 code attached to the `Area`.
    - AreaIso2, String;
    /// An ISO 3166-3 code attached to the `Area`.
    - AreaIso3, String;
    /// The name of thea `Area`.
    - AreaName, String;
    /// The type of the `Area`.
    - AreaType, full_entities::AreaType;
    - ArtistCredit, String;
    /// The MBID of the `Artist`.
    - ArtistMbid, Mbid;
    /// The name of the `Artist` without accented characters.
    - ArtistName, String;
    /// The name of the `Artist` with accented characters.
    - ArtistNameAccent, String;
    /// The type of the `Artist`.
    - ArtistType, full_entities::ArtistType;
    - Asin, String;
    /// The barcode of a `Release`.
    - Barcode, String;
    - BeginArea, String;
    /// Begin date of the searched entity.
    ///
    /// Check the searched entity's documentation for more information what this means concretely.
    - BeginDate, PartialDate;
    - CatalogNumber, String;
    /// Disambiguation comment of the searched entity.
    - Comment, String;
    - Country, String;
    - CreditName, String;
    - DataQuality, String;
    - EndArea, String;
    /// End date of the searched entity.
    ///
    /// Check the searched entity's documentation for more information what this means concretely.
    - EndDate, PartialDate;
    /// Whether the searched entity has already ended.
    ///
    /// Check the searched entity's documentation for more information what this means concretely.
    - Ended, bool;
    /// The gender of an `Artist`.
    - Gender, String;
    - IpiCode, String;
    - LabelId, String;
    - Language, full_entities::Language;
    - MediumCount, u32;
    - MediumFormat, String;
    /// The searched entity's name. (TODO implement for all relevant searches)
    - Name, String;
    - NumDiscIds, u32;
    - NumDiscIdsMedium, u32;
    - NumTracks, u32;
    - NumTracksMedium, u32;
    - PrimaryType, full_entities::ReleaseGroupPrimaryType;
    - ReleaseDate, full_entities::PartialDate;
    - ReleaseGroupId, Mbid;
    - ReleaseGroupName, String;
    - ReleaseGroupNameAccent, String;
    - ReleaseId, Mbid;
    /// The name of the `Release`, without special accent characters.
    - ReleaseName, String;
    /// The name of the `Release`, including special accent characters.
    - ReleaseNameAccent, String;
    - ReleaseNumber, u16;
    - ReleaseStatus, full_entities::ReleaseStatus;
    - Script, String;
    - SecondaryType, String;
    /// The sort name of the searched entity.
    - SortName, String;
    - Tag, String
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

    "aid", AreaMbid;
    "alias", Alias;
    "area", AreaName;
    "area", Name;
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
    "arid", ArtistMbid;
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

    "arid", ArtistMbid;
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

    "arid", ArtistMbid;
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
*/
