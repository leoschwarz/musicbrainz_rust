use super::{ClientError, ClientErrorKind, hyper};
use super::entities::{Mbid, Resource};

use hyper::Url;
use hyper::header::UserAgent;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;
use xpath_reader::reader::{FromXmlContained, XpathStrReader};

pub mod search;
use self::search::{AreaSearchBuilder, ArtistSearchBuilder, ReleaseGroupSearchBuilder};
pub use self::search::SearchBuilder;

mod error;
use self::error::check_response_error;

/// Configuration for the client.
pub struct ClientConfig {
    /// The user-agent to be sent with every request to the API.
    ///
    /// Provide a meaningful one as it will be used by MusicBrainz to identify
    /// your application and
    /// without a user agent sever throttling will be undertaken. The official
    /// suggestion is to use
    /// either one of the following two options:
    ///
    /// * `Application name/<version> ( contact-url )`
    /// * `Application name/<version> ( contact-email )`
    ///
    /// For more information see:
    /// https://musicbrainz.org/doc/XML_Web_Service/Rate_Limiting
    pub user_agent: String,
}

/// The main struct to be used to communicate with the MusicBrainz API.
pub struct Client {
    config: ClientConfig,
    http_client: hyper::Client,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError>
    {
        let ssl = NativeTlsClient::new()?;
        let connector = HttpsConnector::new(ssl);

        Ok(Client {
               config: config,
               http_client: hyper::Client::with_connector(connector),
           })
    }

    /// Fetch the specified ressource from the server and parse it.
    pub fn get_by_mbid<Res>(&self, mbid: &Mbid) -> Result<Res, ClientError>
        where Res: Resource + FromXmlContained
    {
        use entities::default_musicbrainz_context;

        let url = Res::get_url(mbid);
        let response_body = self.get_body(url.parse()?)?;

        // Parse the response.
        let context = default_musicbrainz_context();
        let reader = XpathStrReader::new(&response_body[..], &context)?;
        check_response_error(&reader)?;
        Ok(Res::from_xml(&reader)?)
    }

    fn get_body(&self, url: Url) -> Result<String, ClientError>
    {
        let mut response =
            self.http_client.get(url).header(UserAgent(self.config.user_agent.clone())).send()?;
        let mut response_body = String::new();
        response.read_to_string(&mut response_body)?;
        Ok(response_body)
    }

    /// Returns a search builder to search for an area.
    pub fn search_area<'cl>(&'cl self) -> AreaSearchBuilder<'cl>
    {
        AreaSearchBuilder::new(self)
    }

    /// Returns a search biulder to search for an artist.
    pub fn search_artist<'cl>(&'cl self) -> ArtistSearchBuilder<'cl>
    {
        ArtistSearchBuilder::new(self)
    }

    /// Returns a search builder to search for a release group.
    pub fn search_release_group<'cl>(&'cl self) -> ReleaseGroupSearchBuilder<'cl>
    {
        ReleaseGroupSearchBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_client() -> Client
    {
        let config = ClientConfig { user_agent: "MusicBrainz-Rust/Testing".to_string() };
        Client::new(config).unwrap()
    }

    #[test]
    fn search_release_group()
    {
        let client = get_client();
        let results = client
            .search_release_group()
            .add(search::fields::release_group::ReleaseGroupName("霊魂消滅".to_owned()))
            .search()
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].score, 100);
        assert_eq!(results[0].entity.mbid,
                   "739de9cd-7e81-4bb0-9fdb-0feb7ea709c7".parse().unwrap());
        assert_eq!(results[0].entity.title, "霊魂消滅".to_string());
    }
}
