use xpath_reader::Context;

pub fn musicbrainz_context<'d>() -> Context<'d>
{
    let mut context = Context::default();
    context.set_namespace("mb", "http://musicbrainz.org/ns/mmd-2.0#");
    context
}

#[cfg(test)]
mod test_utils {
    use entities::Resource;
    use xpath_reader::XpathStrReader;
    use xpath_reader::reader::FromXmlContained;

    // TODO right now this just saves us a couple lines of code per test,
    // however in the future this should use `ReplayClient`.
    pub fn extract_entity<E: Resource + FromXmlContained>(source: &str) -> E
    {
        let context = super::musicbrainz_context();
        let reader = XpathStrReader::new(source, &context).unwrap();
        E::from_xml(&reader).unwrap()
    }
}

#[cfg(test)]
pub use self::test_utils::*;
