use xpath_reader::Context;

pub fn musicbrainz_context<'d>() -> Context<'d> {
    let mut context = Context::default();
    context.set_namespace("mb", "http://musicbrainz.org/ns/mmd-2.0#");
    context
}

#[cfg(test)]
pub mod test_utils {
    use crate::client::{Client, ClientConfig, ClientWaits};
    use crate::entities::{Mbid, Resource};
    use crate::errors::Error;
    use reqwest_mock::GenericClient as HttpClient;
    use xpath_reader::reader::FromXml;

    pub fn fetch_entity<E: Resource + FromXml>(mbid: &Mbid) -> Result<E, Error> {
        let mut client = Client::with_http_client(
            ClientConfig {
                user_agent: "MusicBrainz-Rust/Testing".to_string(),
                max_retries: 5,
                waits: ClientWaits::default(),
            },
            HttpClient::replay_file(format!("replay/test_entities/{}/{}.json", E::NAME, mbid)),
        );
        client.get_by_mbid(mbid)
    }
}

#[cfg(test)]
pub use self::test_utils::*;
