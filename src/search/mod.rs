//! Provides an ergonomic way to query entries from the MusicBrainz database.
//!
//! First a bit of terminology: You can search entities using different fields,
//! which are all in the module [::search::fields](fields/index.html). Either
//! use them directly from that module or use on of the  submodules, which
//! organize the fields by entity. It's impossible to use  invalid search
//! fields  on the wrong entity.
//!
//! When performing a search you will get an  instance of
//! [SearchResult](struct.SearchResult.html)  wrapping instances of
//! [SearchEntity](search_entities/trait.SearchEntity.html) corresponding to
//! the full entity  you want to query from the database. You  can fetch the
//! full  entity from a  search entity, using the `fetch_full()` method on the
//! search entity.

use crate::entities as full_entities;
use crate::entities::Resource;
use crate::errors::ClientError;
use crate::client::Client;

use reqwest_mock::Url;
use url::percent_encoding::{DEFAULT_ENCODE_SET, utf8_percent_encode};
use xpath_reader::{FromXml, Error, Reader};

pub mod fields;
use self::fields::{AreaSearchField, ArtistSearchField, ReleaseGroupSearchField, ReleaseSearchField};

pub mod search_entities;
use self::search_entities::SearchEntity;

pub type SearchResult<Entity> = Result<Vec<SearchEntry<Entity>>, ClientError>;

pub mod query;

pub trait SearchBuilder {
    /// The entity from the client::search::entities module,
    /// this is the entity contained in the search result.
    type Entity: SearchEntity;

    /// The full entity a search entity can be expanded into.
    type FullEntity: Resource + FromXml;

    /// Perform the search.
    fn search(self) -> SearchResult<Self::Entity>;
}

/// One entry of the search results.
pub struct SearchEntry<E>
where
    E: SearchEntity,
{
    /// The returned entity.
    pub entity: E,

    /// A value from 0 to 100 indicating in percent how much this specific
    /// search result matches
    /// the search query.
    pub score: u8,
}

macro_rules! define_search_builder {
    ( $builder:ident,
      $fields:ident,
      $entity:ty,
      $full_entity:ty,
      $list_tag:expr ) => {
        pub struct $builder<'cl> {
            params: Vec<(&'static str, String)>,
            client: &'cl mut Client,
        }

        impl<'cl> $builder<'cl> {
            pub fn new(client: &'cl mut Client) -> Self {
                Self {
                    params: Vec::new(),
                    client: client,
                }
            }

            /// Specify an additional parameter for the query.
            ///
            /// Currently all parameters will be combined using `AND`.
            pub fn add<F>(mut self, field: F) -> Self
            where
                F: $fields,
            {
                self.params.push((F::name(), field.to_string()));
                self
            }

            /// Builds the full url to be used to perform the search request.
            fn build_url(&self) -> Result<Url, ClientError> {
                let mut query_parts: Vec<String> = Vec::new();
                for &(p_name, ref p_value) in self.params.iter() {
                    // TODO (FIXME): Does this also encode ":" ?
                    let value = utf8_percent_encode(p_value.as_ref(), DEFAULT_ENCODE_SET);
                    query_parts.push(format!("{}:{}", p_name, value));
                }

                // TODO: In the future support OR queries too.
                let query = query_parts.join("%20AND%20");
                type FE = $full_entity;
                let base_url = format!("https://musicbrainz.org/ws/2/{}/", FE::NAME);
                Ok(Url::parse(
                    format!("{}?query={}", base_url, query).as_ref(),
                )?)
            }

            /// Parse the search result.
            fn parse_xml(xml: &str) -> SearchResult<$entity> {
                let mut context = crate::util::musicbrainz_context();
                context.set_namespace("ext", "http://musicbrainz.org/ns/ext#-2.0");

                let reader = Reader::from_str(xml, Some(&context))?;
                crate::client::check_response_error(&reader)?;
                Ok(reader.read("//mb:metadata")?)
            }
        }

        impl<'cl> SearchBuilder for $builder<'cl> {
            type Entity = $entity;
            type FullEntity = $full_entity;

            fn search(self) -> SearchResult<Self::Entity> {
                let url = self.build_url()?;

                // Perform the request.
                let response_body = self.client.get_body(url)?;
                Self::parse_xml(response_body.as_str())
            }
        }

        impl FromXml for SearchEntry<$entity> {
            fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, Error> {
                Ok(Self {
                    entity: reader.read(format!(".//mb:{}", $list_tag).as_str())?,
                    score: reader.read(format!(".//mb:{}/*/@ext:score", $list_tag).as_str())?,
                })
            }
        }
    };
}

define_search_builder!(
    AreaSearchBuilder,
    AreaSearchField,
    search_entities::Area,
    full_entities::Area,
    "area-list"
);

define_search_builder!(
    ArtistSearchBuilder,
    ArtistSearchField,
    search_entities::Artist,
    full_entities::Artist,
    "artist-list"
);

/* TODO
define_search_builder!(
    ReleaseSearchBuilder,
    ReleaseSearchField,
    search_entities::Release,
    full_entities::Release,
    "release-list"
);
*/

define_search_builder!(
    ReleaseGroupSearchBuilder,
    ReleaseGroupSearchField,
    search_entities::ReleaseGroup,
    full_entities::ReleaseGroup,
    "release-group-list"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_releasegroup() {
        // url: https://musicbrainz.org/ws/2/release-group/?query=releasegroup:
        // %E9%9C%8A%E9%AD%82%E6%B6%88%E6%BB%85
        let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><metadata created="2017-05-06T09:45:01.432Z" xmlns="http://musicbrainz.org/ns/mmd-2.0#" xmlns:ext="http://musicbrainz.org/ns/ext#-2.0"><release-group-list count="1" offset="0"><release-group id="739de9cd-7e81-4bb0-9fdb-0feb7ea709c7" type="Single" ext:score="100"><title>霊魂消滅</title><primary-type>Single</primary-type><artist-credit><name-credit><artist id="90e7c2f9-273b-4d6c-a662-ab2d73ea4b8e"><name>NECRONOMIDOL</name><sort-name>NECRONOMIDOL</sort-name></artist></name-credit></artist-credit><release-list count="1"><release id="d3d2a860-0093-461d-8d95-b77939c2e944"><title>霊魂消滅</title><status>Official</status></release></release-list></release-group></release-group-list></metadata>"#;
        let res: Vec<SearchEntry<search_entities::ReleaseGroup>> =
            ReleaseGroupSearchBuilder::parse_xml(xml).unwrap();

        assert_eq!(res.len(), 1);
        let ref rg = res[0];

        assert_eq!(rg.score, 100);
        assert_eq!(
            rg.entity.mbid,
            "739de9cd-7e81-4bb0-9fdb-0feb7ea709c7".parse().unwrap()
        );
        assert_eq!(rg.entity.title, "霊魂消滅".to_string());
    }
}
