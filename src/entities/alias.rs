use xpath_reader::{FromXml, FromXmlOptional, Reader};
use crate::entities::Language;

enum_mb_xml_optional!(
    pub enum AliasType {
        var SearchHint = "Search hint",
        var ArtistName = "Artist name",
        var LegalName = "Legal name",
    }
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alias {
    pub(crate) alias_type: Option<AliasType>,
    // TODO: I think SearchHint does not have a sort_name, at least according to the MB web
    // interface. -> But check if the API does always return one anyways and remove Option in that
    // case.
    pub(crate) sort_name: Option<String>,
    pub(crate) name: String,
    pub(crate) locale: Option<Language>,
    pub(crate) primary: bool,
}

impl FromXml for Alias {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Self, xpath_reader::Error> {
        let primary: Option<String> = reader.read(".//@primary")?;

        let lang: Option<String> = reader.read(".//@locale")?;
        let locale = match lang {
            Some(l) => Some(
                Language::from_639_1(l.as_ref()).map_err(|e| xpath_reader::Error::custom_err(e))?,
            ),
            None => None,
        };

        Ok(Alias {
            alias_type: reader.read(".//@type")?,
            sort_name: reader.read(".//@sort-name")?,
            name: reader.read(".//text()")?,
            locale,
            primary: primary == Some("primary".into()),
        })
    }
}

impl Alias {
    pub fn alias_type(&self) -> Option<AliasType> {
        self.alias_type
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn sort_name(&self) -> Option<&String> {
        self.sort_name.as_ref()
    }

    pub fn locale(&self) -> Option<&Language> {
        self.locale.as_ref()
    }

    pub fn primary(&self) -> bool {
        self.primary
    }
}
