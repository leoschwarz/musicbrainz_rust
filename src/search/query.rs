//! Query builder code.
//!
//! In general you won't need to use any of these types directly, but instead
//! through the facilities provided by `Client`.

use super::*;
use regex::Regex;
use url::percent_encoding::{DEFAULT_ENCODE_SET, utf8_percent_encode};

/// Escape all lucene special characters and then escape it so it can be used
/// for a url string.
///
/// This is to be used for attribute values, like for example a release name.
pub(crate) fn escape_full(text: &str) -> String
{
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
fn escape_query(text: &str) -> String
{
    utf8_percent_encode(text, DEFAULT_ENCODE_SET).to_string()
}

pub trait QueryExpression: Sized {
    /// The entity which is being queried.
    type Entity: SearchEntity;

    /// Build the query. This is already supposed to be escaped properly.
    fn build_query(&self) -> String;

    fn and<O: QueryExpression<Entity = Self::Entity>>(self, other: O)
        -> And<Self, O, Self::Entity>
    {
        And { a: self, b: other }
    }

    fn or<O: QueryExpression<Entity = Self::Entity>>(self, other: O) -> Or<Self, O, Self::Entity>
    {
        Or { a: self, b: other }
    }
}

pub struct And<A, B, E>
where
    A: QueryExpression<Entity = E>,
    B: QueryExpression<Entity = E>,
    E: SearchEntity,
{
    a: A,
    b: B,
}

impl<A, B, E> QueryExpression for And<A, B, E>
where
    A: QueryExpression<Entity = E>,
    B: QueryExpression<Entity = E>,
    E: SearchEntity,
{
    type Entity = E;

    fn build_query(&self) -> String
    {
        format!("({})AND({})", self.a.build_query(), self.b.build_query())
    }
}

pub struct Or<A, B, E>
where
    A: QueryExpression<Entity = E>,
    B: QueryExpression<Entity = E>,
    E: SearchEntity,
{
    a: A,
    b: B,
}

impl<A, B, E> QueryExpression for Or<A, B, E>
where
    A: QueryExpression<Entity = E>,
    B: QueryExpression<Entity = E>,
    E: SearchEntity,
{
    type Entity = E;

    fn build_query(&self) -> String
    {
        format!("({})OR({})", self.a.build_query(), self.b.build_query())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_full()
    {
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
    fn test_escape_query()
    {
        // these are all legal in query component
        let legal = r#"/:@-._~!$&'()*+,;="#;
        assert_eq!(escape_query(legal), legal.to_string());
    }
}
