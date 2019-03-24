use xpath_reader::{FromXml, Reader};
use crate::entities::Language;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alias {
    type_: String,
    sort_name: String,
    name: String,
    locale: Option<Language>,
    primary: bool,
}

impl FromXml for Alias {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
        let primary: Option<String> = reader.read(".//@primary")?;
        Ok(Alias {
            type_: reader.read(".//@type")?,
            sort_name: reader.read(".//@sort-name")?,
            name: reader.read(".//text()")?,
            locale: reader.read(".//@locale")?,
            primary: primary == Some("primary".into()),
        })
    }
}
