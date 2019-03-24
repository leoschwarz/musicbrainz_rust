//! Contains the types and functions to communicate with the MusicBrainz API.

use crate::error::{Error, ErrorKind};
use crate::entities::{Mbid, ResourceOld, Resource};

use reqwest_mock::Client as MockClient;
use reqwest_mock::GenericClient as HttpClient;
use reqwest_mock::{StatusCode, Url};
use reqwest_mock::header::UserAgent;
use xpath_reader::reader::{FromXml, Reader};

use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::search::{ReleaseGroupSearchBuilder, SearchBuilder};

mod error;
pub(crate) use self::error::check_response_error;

/// Helper extracting the number of milliseconds from a `Duration`.
fn as_millis(duration: &Duration) -> u64 {
    ((duration.as_secs() as f64) + (duration.subsec_nanos() as f64) * 1e6) as u64
}

/// Returns an `Instant` at least 1000 seconds ago.
fn past_instant() -> Instant {
    Instant::now() - Duration::new(1000, 0)
}

/// Configuration for the client.
#[derive(Clone, Debug)]
pub struct ClientConfig {
    /// The user-agent to be sent with every request to the API.
    ///
    /// Provide a meaningful one as it will be used by MusicBrainz to identify
    /// your application and without a user agent sever throttling will be
    /// undertaken. The official suggestion is to use either one of the
    /// following two options:
    ///
    /// * `Application name/<version> ( contact-url )`
    /// * `Application name/<version> ( contact-email )`
    ///
    /// For more information see:
    /// https://musicbrainz.org/doc/XML_Web_Service/Rate_Limiting
    pub user_agent: String,

    /// How many times to retry requests where MusicBrainz returned 503 because
    /// too many requests were being made.
    pub max_retries: u8,

    /// Specifies amounts of time to wait between certain actions.
    pub waits: ClientWaits,
}

/// Specification of the wait time between requests.
///
/// Times are specified in miliseconds.
#[derive(Clone, Debug)]
pub struct ClientWaits {
    /// Initial wait time after a ServiceUnavailable to use for the exponential
    /// backoff strategy.
    pub backoff_init: u64,

    // TODO: Make this configurable if and only if a custom server instance is used,
    //       to make abuse of the main servers harder.
    /// Minimal time between requests
    requests: u64,
}

impl Default for ClientWaits {
    fn default() -> Self {
        ClientWaits {
            backoff_init: 400,
            requests: 1000,
        }
    }
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

/// A request to be performed on the client.
///
/// Note: You most likely won't have to use it directly, it's public for trait visibility
///       reasons.
#[derive(Clone, Debug)]
pub struct Request {
    pub name: String,
    pub include: String,
}

impl Client {
    /// Create a new `Client` instance.
    pub fn new(config: ClientConfig) -> Self {
        Client {
            config: config,
            http_client: HttpClient::direct(),
            last_request: past_instant(),
        }
    }

    /// Create a new `Client` instance with the specified `HttpClient`.
    ///
    /// This is useful for testing purposes where you can inject a different
    /// `HttpClient`, i. e. one replaying requests to save API calls or one
    /// providing explicit stubbing.
    pub fn with_http_client(config: ClientConfig, client: HttpClient) -> Self {
        Client {
            config: config,
            http_client: client,
            last_request: past_instant(),
        }
    }

    /// Waits until we are allowed to make the next request to the MusicBrainz
    /// API.
    fn wait_if_needed(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_request);
        if as_millis(&elapsed) < self.config.waits.requests {
            sleep(Duration::from_millis(self.config.waits.requests) - elapsed);
        }
        self.last_request = now;
    }

    pub fn get_by_mbid<Res, Resp, Opt>(&mut self, mbid: &Mbid, options: Opt) -> Result<Res, Error>
    where
        Res: Resource<Options = Opt, Response = Resp>,
        Resp: FromXml,
    {
        let request = Res::request(&options);
        let url = request.get_by_mbid_url(mbid);
        let response_body = self.get_body(url.parse()?)?;
        let context = crate::util::musicbrainz_context();
        let reader = Reader::from_str(response_body.as_str(), Some(&context))?;
        check_response_error(&reader)?;

        let response = Resp::from_xml(&reader)?;

        Ok(Res::from_response(response, options))
    }

    /// Fetch the specified resource from the server and parse it.
    pub fn get_by_mbid_old<Res>(&mut self, mbid: &Mbid) -> Result<Res, Error>
    where
        Res: ResourceOld + FromXml,
    {
        let url = Res::get_url(mbid);
        let response_body = self.get_body(url.parse()?)?;

        // Parse the response.
        let context = crate::util::musicbrainz_context();
        let reader = Reader::from_str(&response_body[..], Some(&context))?;
        check_response_error(&reader)?;
        Ok(Res::from_xml(&reader)?)
    }

    pub(crate) fn get_body(&mut self, url: Url) -> Result<String, Error> {
        self.wait_if_needed();

        let mut attempts = 0;
        let mut backoff = self.config.waits.backoff_init;

        while attempts < self.config.max_retries {
            let response = self
                .http_client
                .get(url.clone())
                .header(UserAgent::new(self.config.user_agent.clone()))
                .send()?;
            if response.status == StatusCode::ServiceUnavailable {
                sleep(Duration::from_millis(backoff));
                attempts += 1;
                backoff *= 2;
                // If we are in testing we want to avoid always failing.
                self.http_client.force_record_next();
            } else {
                let response_body = response.body_to_utf8()?;
                return Ok(response_body);
            }
        }
        Err(Error::new(
            "MusicBrainz returned 503 (ServiceUnavailable) too many times.",
            ErrorKind::Communication,
        ))
    }
    /*
    /// Returns a search builder to search for an area.
    pub fn search_area<'cl>(&'cl mut self) -> AreaSearchBuilder<'cl> {
        AreaSearchBuilder::new(self)
    }

    /// Returns a search biulder to search for an artist.
    pub fn search_artist<'cl>(&'cl mut self) -> ArtistSearchBuilder<'cl> {
        ArtistSearchBuilder::new(self)
    }*/

    /// Returns a search builder to search for a release group.
    pub fn search_release_group<'cl>(&'cl mut self) -> ReleaseGroupSearchBuilder<'cl> {
        ReleaseGroupSearchBuilder::new(self)
    }
}

impl Request {
    /// Returns the url where one can get a resource in the valid format for
    /// parsing from.
    fn get_by_mbid_url(&self, mbid: &Mbid) -> String {
        format!(
            "https://musicbrainz.org/ws/2/{}/{}?inc={}",
            self.name, mbid, self.include
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_client(testname: &str) -> Client {
        Client::with_http_client(
            ClientConfig {
                user_agent: "MusicBrainz-Rust/Testing".to_string(),
                max_retries: 5,
                waits: ClientWaits::default(),
            },
            HttpClient::replay_file(format!("replay/test_client/search/{}.json", testname)),
        )
    }

    #[test]
    fn search_release_group() {
        let mut client = get_client("release_group_01");
        let results = client
            .search_release_group()
            .add(crate::search::fields::release_group::ReleaseGroupName(
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
