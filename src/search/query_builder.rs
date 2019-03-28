/// Code to easily generate queries for entities in a type safe way.
///
/// # Syntax Features
/// ## Wildcards
/// Terms and phrases can contain the following wildcards (except as the first character of search):
/// - `?`: single character wildcard search
/// - `*`: multiple character wildcard search
///
/// ## TODO
/// - Regular expressions: These should get their own api.
/// - Range searches.
///
/// # Further documentation
///
/// Find the basic documentation of the search syntax at:
/// https://musicbrainz.org/doc/Indexed_Search_Syntax
/// The full description of the Lucene search syntax is more complicated and includes many details
/// only very few users will ever need, for these purposes the query builder is inferior to providing
/// a direct query through the query api.
///
/// The more complicated documentation is available at
/// https://lucene.apache.org/core/4_3_0/queryparser/org/apache/lucene/queryparser/classic/package-summary.html#package_description
use std::fmt;
use crate::search::query::Query;
use crate::search::search_entities::SearchEntity;

pub trait QueryBuilder {
    /// The entity which is being queried.
    type Entity: SearchEntity;
}

trait Expression: fmt::Display {}

trait TermExpression: Expression {}

trait Term: Expression {
    fn is_boosted(&self) -> bool;
    fn is_fuzzy(&self) -> bool;
}

trait Phrase: Expression {
    fn is_boosted(&self) -> bool;
    fn is_proximity(&self) -> bool;
}

struct BasicTerm<'a> {
    value: &'a str,
}

struct BasicPhrase<'a> {
    value: &'a str,
}

struct CombinedPhrase<'a> {
    terms: Vec<Term>,
    operator: OperatorKind,
}

struct FuzzyTerm<'a, T> {
    term: T,
    max_distance: u32,
}

struct BoostTerm<'a, T> {
    term: T,
    weight: f32,
}

struct ProximityPhrase<'a, P> {
    phrase: P,
    max_distance: u32,
}

struct BoostPhrase<'a, P> {
    phrase: P,
    weight: f32,
}

impl<'a> fmt::Display for BasicTerm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}

impl<'a> Expression for BasicTerm<'a> {}

impl<'a> Term for BasicTerm<'a> {
    fn is_boosted(&self) -> bool {
        false
    }
    fn is_fuzzy(&self) -> bool {
        false
    }
}

impl<'a, T> fmt::Display for BoostTerm<'a, T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}^{}", self.term, self.weight)
    }
}

impl<'a, T> Expression for BoostTerm<'a, T> where T: Term {}

impl<'a, T> Term for BoostTerm<'a, T>
where
    T: Term,
{
    fn is_boosted(&self) -> bool {
        true
    }

    fn is_fuzzy(&self) -> bool {
        self.term.is_fuzzy()
    }
}

impl<'a, T> fmt::Display for FuzzyTerm<'a, T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}~{}", self.term, self.weight)
    }
}

impl<'a, T> Expression for FuzzyTerm<'a, T> where T: Term {}

impl<'a, T> Term for FuzzyTerm<'a, T>
where
    T: Term,
{
    fn is_boosted(&self) -> bool {
        self.term.is_boosted()
    }

    fn is_fuzzy(&self) -> bool {
        true
    }
}

impl<'a> fmt::Display for BasicPhrase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "\"{}\"", self.value)
    }
}

impl<'a> Expression for BasicPhrase<'a> {}

impl<'a> Phrase for BasicPhrase<'a> {
    fn is_boosted(&self) -> bool {
        false
    }

    fn is_proximity(&self) -> bool {
        false
    }
}

impl<'a, P> fmt::Display for BoostPhrase<'a, P>
where
    P: Phrase,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}^{}", self.phrase, self.weight)
    }
}

impl<'a, P> Expression for BoostPhrase<'a, P> where P: Phrase {}

impl<'a, P> Phrase for BoostPhrase<'a, P>
where
    P: Phrase,
{
    fn is_boosted(&self) -> bool {
        true
    }

    fn is_proximity(&self) -> bool {
        self.phrase.is_fuzzy()
    }
}

impl<'a, P> fmt::Display for ProximityPhrase<'a, P>
where
    P: Phrase,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}~{}", self.phrase, self.max_distance)
    }
}

impl<'a, P> Expression for ProximityPhrase<'a, P> where P: Phrase {}

impl<'a, P> Phrase for ProximityPhrase<'a, P>
where
    P: Phrase,
{
    fn is_boosted(&self) -> bool {
        self.phrase.is_boosted()
    }

    fn is_proximity(&self) -> bool {
        true
    }
}

impl Term {
    /// max_distance must be in {0, 1, 2} and specifies the maximum Levensthein distance to other
    /// terms
    pub fn fuzzy(self, max_distance: u32) -> FuzzyTerm<Self> {
        if self.is_fuzzy() {
            panic!("Specifying term as fuzzy which is already fuzzy.");
        } else {
            FuzzyTerm {
                term: self,
                max_distance,
            }
        }
    }

    /// With boosting a particular term can be made more or less relevant in the search.
    ///
    /// The default value is 1.0, a larger value makes it more relevant, a smaller value makes it
    /// less relevant.
    pub fn boost(self, weight: f32) -> BoostTerm<Self> {
        if self.is_boosted() {
            panic!("Boosting a term again which was already boosted before.");
        } else {
            BoostTerm { term: self, weight }
        }
    }
}

impl<'a> fmt::Display for CombinedPhrase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let n_tot = self.terms.len();
        let mut n_cur = 1;

        for term in self.terms {
            if n_cur != n_tot {
                write!(f, "{} {}", term, self.operator)?;
            } else {
                // Last item.
                write!(f, "{}", term)?;
            }
            n_cur += 1;
        }
        Ok(())
    }
}

impl<'a> Expression for CombinedPhrase<'a> {}

impl<'a> Phrase for CombinedPhrase<'a> {
    fn is_boosted(&self) -> bool {
        false
    }

    fn is_proximity(&self) -> bool {
        false
    }
}

enum OperatorKind {
    /// Either of the expressions has to hold.
    ///
    /// Lucene query: `(LHS) OR (RHS)`.
    Or,

    /// Both of the expressions have to hold.
    ///
    /// Lucene query: `(LHS) AND (RHS)`.
    And,

    /// The left hand side has to hold but the right hand side must not.
    ///
    /// Note: Technically it doesn't care what the left hand side is but there must be one.
    /// Lucene query: `(LHS) NOT (RHS)`.
    Not,
}

impl fmt::Display for OperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            OperatorKind::Or => write!(f, "OR"),
            OperatorKind::And => write!(f, "AND"),
            OperatorKind::Not => write!(f, "NOT"),
        }
    }
}

/*
/// A (logical) word, or a combination of field:word.
#[derive(Clone, Debug)]
struct Term<'a> {
    name: Option<&'a str>,
    value: &'a str,
    weight: TermWeight,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TermWeight {
    /// Specifies that the term may be present.
    Default,
    /// Specifies that the term is required to be present.
    Require,
    Exclude
}



struct Operator<'a, LHS, RHS> {
    kind: OperatorKind,
    lhs: LHS,
    rhs: RHS
}

impl Default for TermWeight {
    fn default() -> Self {
        TermWeight::Default
    }
}*/
