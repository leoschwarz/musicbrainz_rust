use xpath_reader::Context;

pub fn musicbrainz_context<'d>() -> Context<'d>
{
    let mut context = Context::default();
    context.set_namespace("mb", "http://musicbrainz.org/ns/mmd-2.0#");
    context
}

#[cfg(test)]
pub mod test_utils {
    use client::{Client, ClientConfig};
    use entities::{Mbid, Resource};
    use errors::ClientError;
    use reqwest_mock::GenericClient as HttpClient;
    use xpath_reader::reader::FromXmlContained;

    pub fn fetch_entity<E: Resource + FromXmlContained>(mbid: &Mbid) -> Result<E, ClientError>
    {
        let mut client = Client::with_http_client(
            ClientConfig {
                user_agent: "MusicBrainz-Rust/Testing".to_string(),
                max_retries: 5,
            },
            HttpClient::replay_file(format!(
                "replay/test_entities/{}/{}.json",
                E::get_name(),
                mbid
            )),
        );
        client.get_by_mbid(mbid)
    }
}

#[cfg(test)]
pub use self::test_utils::*;
