use xpath_reader::Context;

pub fn musicbrainz_context<'d>() -> Context<'d>
{
    let mut context = Context::default();
    context.set_namespace("mb", "http://musicbrainz.org/ns/mmd-2.0#");
    context
}
