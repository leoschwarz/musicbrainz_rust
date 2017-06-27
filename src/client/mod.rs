use errors::{ClientError, ClientErrorKind};
use entities::{Mbid, Resource};

use reqwest_mock::Client as MockClient;
use reqwest_mock::GenericClient as HttpClient;
use reqwest_mock::Url;
use reqwest_mock::header::UserAgent;
use xpath_reader::reader::{FromXmlContained, XpathStrReader};

use std::time::{Duration, Instant};
use std::thread::sleep;

pub mod search;
use self::search::{AreaSearchBuilder, ArtistSearchBuilder, ReleaseGroupSearchBuilder};
pub use self::search::SearchBuilder;

mod error;
use self::error::check_response_error;

/// Helper extracting the number of milliseconds from a `Duration`.
fn as_millis(duration: &Duration) -> f64
{
    (duration.as_secs() as f64) + (duration.subsec_nanos() as f64) * 1e6
}

/// Returns an `Instant` at least 1000 seconds ago.
fn past_instant() -> Instant
{
    Instant::now() - Duration::new(1000, 0)
}

/// Configuration for the client.
#[derive(Clone, Debug)]
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
///
/// Please create only one instance and use it troughout your application
/// as it will ensure appropriate wait times between requests to prevent
/// being blocked for making to many requests.
pub struct Client {
    http_client: HttpClient,
    config: ClientConfig,

    /// The time the last request was made.
    /// According to the documentation we have to wait at least one second
    /// between any two requests
    /// to the MusicBrainz API.
    last_request: Instant,
}

impl Client {
    /// Create a new Client instance.
    pub fn new(config: ClientConfig) -> Result<Self, ClientError>
    {
        Ok(Client {
            config: config,
            http_client: HttpClient::direct(),
            last_request: past_instant(),
        })
    }
}

impl Client {
    fn wait_if_needed(&mut self)
    {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_request);
        if as_millis(&elapsed) < 1000. {
            // We have to wait a bit.
            sleep(Duration::new(1, 0) - elapsed);
        }
        self.last_request = now;
    }

    /// Fetch the specified ressource from the server and parse it.
    pub fn get_by_mbid<Res>(&mut self, mbid: &Mbid) -> Result<Res, ClientError>
    where
        Res: Resource + FromXmlContained,
    {
        let url = Res::get_url(mbid);
        let response_body = self.get_body(url.parse()?)?;

        // Parse the response.
        let context = ::util::musicbrainz_context();
        let reader = XpathStrReader::new(&response_body[..], &context)?;
        check_response_error(&reader)?;
        Ok(Res::from_xml(&reader)?)
    }

    fn get_body(&mut self, url: Url) -> Result<String, ClientError>
    {
        self.wait_if_needed();

        let response =
            self.http_client.get(url).header(UserAgent(self.config.user_agent.clone())).send()?;
        let response_body = response.body_to_utf8()?;
        Ok(response_body)
    }

    /// Returns a search builder to search for an area.
    pub fn search_area<'cl>(&'cl mut self) -> AreaSearchBuilder<'cl>
    {
        AreaSearchBuilder::new(self)
    }

    /// Returns a search biulder to search for an artist.
    pub fn search_artist<'cl>(&'cl mut self) -> ArtistSearchBuilder<'cl>
    {
        ArtistSearchBuilder::new(self)
    }

    /// Returns a search builder to search for a release group.
    pub fn search_release_group<'cl>(&'cl mut self) -> ReleaseGroupSearchBuilder<'cl>
    {
        ReleaseGroupSearchBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest_mock::ReplayClient;

    fn get_client(testname: &str) -> Client
    {
        Client {
            config: ClientConfig { user_agent: "MusicBrainz-Rust/Testing".to_string() },
            http_client: HttpClient::replay_file(format!("replay/src/client/mod/{}.json", testname)),
            last_request: past_instant(),
        }
    }

    #[test]
    fn search_release_group()
    {
        let mut client = get_client("search_release_group");
        let results = client
            .search_release_group()
            .add(search::fields::release_group::ReleaseGroupName(
                "霊魂消滅".to_owned(),
            ))
            .search()
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].score, 100);
        assert_eq!(
            results[0].entity.mbid,
            "739de9cd-7e81-4bb0-9fdb-0feb7ea709c7".parse().unwrap()
        );
        assert_eq!(results[0].entity.title, "霊魂消滅".to_string());
    }
}
