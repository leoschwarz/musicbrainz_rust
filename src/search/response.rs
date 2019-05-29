use crate::search::search_entities::SearchEntity;
use crate::entities::Resource;
use xpath_reader::{FromXml, Reader};

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

pub struct Response<E: SearchEntity> {
    entries: Vec<Entry<E>>
}

impl<E: SearchEntity> FromXml for Entry<E> {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
        Ok(Entry {
            entity: reader.read(E::FullEntity::NAME)?,
            score: reader.read("score")?,
        })
    }
}

impl<E: SearchEntity> FromXml for Response<E> {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
        Ok(Response {
            entries: reader.read(format!("{}-list", E::FullEntity::NAME))?,
        })
    }
}
