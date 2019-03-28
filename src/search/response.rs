use crate::search::search_entities::SearchEntity;
use crate::Error;

/// One entry of the search results.
pub struct Entry<E>
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

pub type Response<Entity> = Result<Vec<Entry<Entity>>, Error>;
