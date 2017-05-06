use super::*;

/// A label entity in the MusicBrainz database.
/// There is quite some controversy in the music industry what a 'label'
/// constitutes.
///
/// For a complete disambiguation see the `LabelType` enum. The labels in
/// MusicBrainz are mostly
/// imprints.
pub struct Label {
    /// MBID of the entity in the MusicBrainz database.
    pub mbid: Mbid,

    /// The official name of the label.
    pub name: String,

    /// Version of the `name` converted to latin characters for sorting.
    pub sort_name: String,

    /// If there are multiple labels with the same name in the database, a
    /// short disambiguation
    /// comment is provided which allows to differentiate the entities.
    pub disambiguation: Option<String>,

    /// Variants of the name mainly used as search help.
    /// These can be variants, spellings of names, missing titles and common
    /// misspellings.
    pub aliases: Vec<String>,

    /// LC code of the label, as issued by the IFPI.
    pub label_code: Option<String>,

    /// Describes the main activity of the label.
    pub label_type: LabelType,

    /// ISO 3166 country of origin for the label.
    pub country: Option<String>,

    /// Identifying number of the label as assigned by the CISAC database.
    pub ipi_code: Option<String>,

    /// ISNI code of the label.
    pub isni_code: Option<String>,

    /// The date when this label was founded.
    /// (Consult the MusicBrainz manual for disclaimers about the significance
    /// of these
    /// informations.)
    pub begin_date: Option<Date>,

    /// The date when this label ceased to exist or its last release ever was
    /// released.
    pub end_date: Option<Date>,
}

impl Resource for Label {
    fn get_url(mbid: &Mbid) -> String
    {
        format!("https://musicbrainz.org/ws/2/label/{}?inc=aliases", mbid)
    }

    fn base_url() -> &'static str
    {
        "https://musicbrainz.org/ws/2/label/"
    }
}

impl FromXml for Label {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Label, XpathError>
        where R: XpathReader<'d>
    {
        Ok(Label {
               mbid: reader.read(".//mb:label/@id")?,
               name: reader.read(".//mb:label/mb:name/text()")?,
               sort_name: reader.read(".//mb:label/mb:sort-name/text()")?,
               disambiguation: reader.read_option(".//mb:label/mb:disambiguation/text()")?,
               aliases: reader.read_vec(".//mb:label/mb:alias-list/mb:alias/text()")?,
               label_code: reader.read_option(".//mb:label/mb:label-code/text()")?,
               label_type: reader.read(".//mb:label/@type")?,
               country: reader.read_option(".//mb:label/mb:country/text()")?,
               ipi_code: None, // TODO
               isni_code: None, // TODO
               begin_date: reader.read_option(".//mb:label/mb:life-span/mb:begin/text()")?,
               end_date: reader.read_option(".//mb:label/mb:life-span/mb:end/text()")?,
           })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum LabelType {
    /// The main `LabelType` in the MusicBrainz database.
    /// That is a brand (and trademark) associated with the marketing of a
    /// release.
    Imprint,

    /// Production company producing entirely new releases.
    ProductionOriginal,
    /// Known bootleg production companies, not sanctioned by the rights owners
    /// of the released
    /// work.
    ProductionBootleg,
    /// Companies specialized in catalog reissues.
    ProductionReissue,

    /// Companies mainly distributing other labels production, often in a
    /// specfic region of the
    /// world.
    Distribution,
    /// Holdings, conglomerates or other financial entities that don't mainly
    /// produce records but
    /// manage a large set of recording labels owned by them.
    Holding,
    /// An organization which collects royalties on behalf of the artists.
    RightsSociety,
}

impl FromXmlElement for LabelType {}
impl FromXml for LabelType {
    fn from_xml<'d, R>(reader: &'d R) -> Result<Self, XpathError>
        where R: XpathReader<'d>
    {
        let s = String::from_xml(reader)?;
        match s.as_str() {
            "Imprint" => Ok(LabelType::Imprint),
            "Original Production" => Ok(LabelType::ProductionOriginal),
            "Bootleg Production" => Ok(LabelType::ProductionBootleg),
            "Reissue Production" => Ok(LabelType::ProductionReissue),
            "Distribution" => Ok(LabelType::Distribution),
            "Holding" => Ok(LabelType::Holding),
            "RightsSociety" => Ok(LabelType::RightsSociety),
            s => Err(format!("Invalid `LabelType`: '{}'", s).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_read_xml1()
    {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><label id="c029628b-6633-439e-bcee-ed02e8a338f7" type="Original Production" type-id="7aaa37fe-2def-3476-b359-80245850062d"><name>EMI</name><sort-name>EMI</sort-name><disambiguation>EMI Records, since 1972</disambiguation><label-code>542</label-code><country>GB</country><area id="8a754a16-0027-3a29-b6d7-2b40ea0481ed"><name>United Kingdom</name><sort-name>United Kingdom</sort-name><iso-3166-1-code-list><iso-3166-1-code>GB</iso-3166-1-code></iso-3166-1-code-list></area><life-span><begin>1972</begin></life-span></label></metadata>"#;
        let context = default_musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let label = Label::from_xml(&reader).unwrap();

        assert_eq!(label.mbid,
                   Mbid::from_str("c029628b-6633-439e-bcee-ed02e8a338f7").unwrap());
        assert_eq!(label.name, "EMI".to_string());
        assert_eq!(label.sort_name, "EMI".to_string());
        assert_eq!(label.disambiguation,
                   Some("EMI Records, since 1972".to_string()));
        assert_eq!(label.aliases, Vec::<String>::new());
        assert_eq!(label.label_code, Some("542".to_string()));
        assert_eq!(label.label_type, LabelType::ProductionOriginal);
        assert_eq!(label.country, Some("GB".to_string()));
        assert_eq!(label.ipi_code, None);
        assert_eq!(label.isni_code, None);
        assert_eq!(label.begin_date, Some(Date::Year { year: 1972 }));
        assert_eq!(label.end_date, None);
    }

    #[test]
    fn read_aliases()
    {
        // url: https://musicbrainz.
        // org/ws/2/label/168f48c8-057e-4974-9600-aa9956d21e1a?inc=aliases
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><label type-id="7aaa37fe-2def-3476-b359-80245850062d" id="168f48c8-057e-4974-9600-aa9956d21e1a" type="Original Production"><name>avex trax</name><sort-name>avex trax</sort-name><country>JP</country><area id="2db42837-c832-3c27-b4a3-08198f75693c"><name>Japan</name><sort-name>Japan</sort-name><iso-3166-1-code-list><iso-3166-1-code>JP</iso-3166-1-code></iso-3166-1-code-list></area><life-span><begin>1990-09</begin></life-span><alias-list count="2"><alias sort-name="Avex Trax Japan">Avex Trax Japan</alias><alias sort-name="エイベックス・トラックス">エイベックス・トラックス</alias></alias-list></label></metadata>"#;
        let context = default_musicbrainz_context();
        let reader = XpathStrReader::new(xml, &context).unwrap();
        let label = Label::from_xml(&reader).unwrap();

        let mut expected = vec![
            "Avex Trax Japan".to_string(),
            "エイベックス・トラックス".to_string(),
        ];
        expected.sort();
        let mut actual = label.aliases.clone();
        actual.sort();

        assert_eq!(actual, expected);
    }
}
