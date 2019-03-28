use crate::entities::Mbid;

/// A request to be performed on the client.
///
/// Note: You most likely won't have to use it directly, it's public for trait visibility
///       reasons.
#[derive(Clone, Debug)]
pub struct Request {
    pub name: String,
    pub include: String,
}

impl Request {
    /// Returns the url where one can get a resource in the valid format for
    /// parsing from.
    pub fn get_by_mbid_url(&self, mbid: &Mbid) -> String {
        format!(
            "https://musicbrainz.org/ws/2/{}/{}?inc={}",
            self.name, mbid, self.include
        )
    }
}
