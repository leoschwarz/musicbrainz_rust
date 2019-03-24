use xpath_reader::{FromXml, FromXmlOptional, Error, Reader};

use crate::entities::{Mbid, ResourceOld};
use crate::entities::date::PartialDate;

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
    pub label_type: Option<LabelType>,

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
    pub begin_date: Option<PartialDate>,

    /// The date when this label ceased to exist or its last release ever was
    /// released.
    pub end_date: Option<PartialDate>,
}

impl ResourceOld for Label {
    const NAME: &'static str = "label";
    const INCL: &'static str = "aliases";
}

impl FromXml for Label {
    fn from_xml<'d>(reader: &'d Reader<'d>) -> Result<Label, Error> {
        Ok(Label {
            mbid: reader.read(".//mb:label/@id")?,
            name: reader.read(".//mb:label/mb:name/text()")?,
            sort_name: reader.read(".//mb:label/mb:sort-name/text()")?,
            disambiguation: reader.read(".//mb:label/mb:disambiguation/text()")?,
            aliases: reader.read(".//mb:label/mb:alias-list/mb:alias/text()")?,
            label_code: reader.read(".//mb:label/mb:label-code/text()")?,
            label_type: reader.read(".//mb:label/@type")?,
            country: reader.read(".//mb:label/mb:country/text()")?,
            ipi_code: reader.read(".//mb:label/mb:ipi/text()")?,
            isni_code: reader.read(".//mb:label/mb:isni-list/mb-isni/text()")?,
            begin_date: reader.read(".//mb:label/mb:life-span/mb:begin/text()")?,
            end_date: reader.read(".//mb:label/mb:life-span/mb:end/text()")?,
        })
    }
}

enum_mb_xml_optional! {
    pub enum LabelType {
        /// The main `LabelType` in the MusicBrainz database.
        /// That is a brand (and trademark) associated with the marketing of a
        /// release.
        var Imprint = "Imprint",

        var Production = "Production",
        var Publisher = "Publisher",

        /// Production company producing entirely new releases.
        var ProductionOriginal = "Original Production",
        /// Known bootleg production companies, not sanctioned by the rights owners
        /// of the released
        /// work.
        var ProductionBootleg = "Bootleg Production",
        /// Companies specialized in catalog reissues.
        var ProductionReissue = "Reissue Production",

        /// Companies mainly distributing other labels production, often in a
        /// specfic region of the
        /// world.
        var Distribution = "Distribution",
        /// Holdings, conglomerates or other financial entities that don't mainly
        /// produce records but
        /// manage a large set of recording labels owned by them.
        var Holding = "Holding",
        /// An organization which collects royalties on behalf of the artists.
        var RightsSociety = "RightsSociety",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn label_read_xml1() {
        let mbid = Mbid::from_str("c029628b-6633-439e-bcee-ed02e8a338f7").unwrap();
        let label: Label = crate::util::test_utils::fetch_entity(&mbid).unwrap();

        assert_eq!(label.mbid, mbid);
        assert_eq!(label.name, "EMI".to_string());
        assert_eq!(label.sort_name, "EMI".to_string());
        assert_eq!(
            label.disambiguation,
            Some("EMI Records, since 1972".to_string())
        );
        assert_eq!(
            label.aliases,
            vec![
                "EMI".to_string(),
                "EMI Records (UK)".to_string(),
                "EMI Records Ltd".to_string(),
                "EMI UK".to_string(),
            ]
        );
        assert_eq!(label.label_code, Some("542".to_string()));
        assert_eq!(label.label_type, Some(LabelType::ProductionOriginal));
        assert_eq!(label.country, Some("GB".to_string()));
        assert_eq!(label.ipi_code, None);
        assert_eq!(label.isni_code, None);
        assert_eq!(
            label.begin_date,
            Some(PartialDate::from_str("1972").unwrap())
        );
        assert_eq!(label.end_date, None);
    }

    #[test]
    fn read_aliases() {
        let mbid = Mbid::from_str("168f48c8-057e-4974-9600-aa9956d21e1a").unwrap();
        let label: Label = crate::util::test_utils::fetch_entity(&mbid).unwrap();

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
