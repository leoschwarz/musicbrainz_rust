//! Query builder code.
//!
//! In general you won't need to use any of these types directly, but instead
//! through the facilities provided by `Client`.

use super::*;
use regex::Regex;
use url::percent_encoding::{DEFAULT_ENCODE_SET, utf8_percent_encode};
use std::marker::PhantomData;
use crate::search::search_entities::SearchEntity;

/// Escape all lucene special characters and then escape it so it can be used
/// for a url string.
///
/// This is to be used for attribute values, like for example a release name.
pub(crate) fn escape_full(text: &str) -> String {
    // Replace all special lucene syntax elements.
    let re = Regex::new(r#"([+\-!\(\)\{\}\[\]\^"~\*\?:\\]|[&\|]{2})"#).unwrap();
    let sanitized = re.replace_all(text, "\\$0");

    // Now escape the result so it can be used in the query.
    let s = escape_query(&*sanitized);

    // Percent encode = and & which haven't been touched by escape_query.
    let s = s.replace("&", "%26");
    let s = s.replace("=", "%3D");
    s
}

/// actually it might be a good idea to not use this anywhere (TODO)
fn escape_query(text: &str) -> String {
    utf8_percent_encode(text, DEFAULT_ENCODE_SET).to_string()
}

#[derive(Clone, Debug)]
pub struct Query<Entity: SearchEntity> {
    /// Provided as Lucene Text search query, yet to be escaped to be sent as URL parameter.
    unescaped: String,

    _entity_type: PhantomData<Entity>,
}

impl<Entity: SearchEntity> Query<Entity> {
    /// Create a new Query instance from a manual specification of a search query.
    pub fn from_query<S: Into<String>>(query: S) -> Self {
        Query {
            unescaped: query.into(),
            _entity_type: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_full() {
        // lucene syntax elements
        assert_eq!(escape_full("+"), escape_query(r"\+"));
        assert_eq!(escape_full("-"), escape_query(r"\-"));
        assert_eq!(escape_full("&&"), "\\%26%26".to_string());
        assert_eq!(escape_full("||"), escape_query(r"\||"));
        assert_eq!(escape_full("!"), escape_query(r"\!"));
        assert_eq!(escape_full("("), escape_query(r"\("));
        assert_eq!(escape_full(")"), escape_query(r"\)"));
        assert_eq!(escape_full("{"), escape_query(r"\{"));
        assert_eq!(escape_full("}"), escape_query(r"\}"));
        assert_eq!(escape_full("["), escape_query(r"\["));
        assert_eq!(escape_full("]"), escape_query(r"\]"));
        assert_eq!(escape_full("^"), escape_query(r"\^"));
        assert_eq!(escape_full("\""), escape_query("\\\""));
        assert_eq!(escape_full("~"), escape_query(r"\~"));
        assert_eq!(escape_full("*"), escape_query(r"\*"));
        assert_eq!(escape_full("?"), escape_query(r"\?"));
        assert_eq!(escape_full(":"), escape_query(r"\:"));
        assert_eq!(escape_full(r"\"), escape_query(r"\\"));

        // & and = are not to be touched by escape query but we have to escape them at
        // this point
        // too because it would mess up the query component.
        assert_eq!(escape_full("&"), "%26".to_string());
        assert_eq!(escape_full("="), "%3D".to_string());

        // sanity check that the whitespace in the regex is actually ignored
        assert_eq!(escape_full(" "), escape_query(" "));
        assert_eq!(escape_full("  "), escape_query("  "));
    }

    #[test]
    fn test_escape_query() {
        // these are all legal in query component
        let legal = r#"/:@-._~!$&'()*+,;="#;
        assert_eq!(escape_query(legal), legal.to_string());
    }
}
