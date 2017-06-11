/// ! For now only including the search fields of release group.

use super::{Mbid, full_entities};
// use super::query::QueryExpression;
use super::full_entities::Date;
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
    BeginArea, String;
    BeginDate, Date;
    Comment, String;
    Country, String;
    CreditName, String;
    EndArea, String;
    EndDate, Date;
    Ended, bool;
    Gender, String;
    IpiCode, String;
    PrimaryType, full_entities::ReleaseGroupPrimaryType;
    ReleaseGroupId, Mbid;
    ReleaseGroupName, String;
    ReleaseGroupNameAccent, String;
    ReleaseId, Mbid;
    ReleaseName, String;
    ReleaseNumber, u16;
    ReleaseStatus, full_entities::ReleaseStatus;
    SecondaryType, String;
    SortName, String;
    Tag, String
);

macro_rules! define_entity_fields {
    (
        $search_entity:ty, $field_trait:ident, $modname:ident;
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

        /*
         TODO: Implement something like this. However we will have to make sure we are escaping every value exactly one time.
        impl QueryExpression for $field_trait {
            type Entity = $search_entity;

            fn build_query(&self) -> String {
                use super::query::escape_full;
                format!("{}:{}", escape_full(Self::name()), escape_full(self.to_string().as_ref()))
            }
        }
        */

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
    entities::Area, AreaSearchField, area;

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
    entities::Artist, ArtistSearchField, artist;

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

define_entity_fields!(
    entities::ReleaseGroup, ReleaseGroupSearchField, release_group;

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
