use xpath_reader::Reader;
use crate::{Error, ErrorKind};

/// Checks if there is an error in the document provided by the reader and
/// returns Ok if there
/// wasn't and Err parsing the MusicBrainz error if the API actually returned
/// an error.
pub fn check_response_error<'d>(reader: &'d Reader<'d>) -> Result<(), Error> {
    let r: Result<Vec<String>, _> = reader.read("//error/text");

    match r {
        Ok(errs) => {
            if errs.len() > 0 {
                let text = errs.join("\n");
                Err(Error::new(text, ErrorKind::ServerError))
            } else {
                Ok(())
            }
        }
        Err(_) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xpath_reader::Reader;

    const XML_ERR: &'static str = r#"<?xml version="1.0" encoding="UTF-8"?>
<error>
  <text>Your requests are exceeding the allowable rate limit. Please see http://wiki.musicbrainz.org/XMLWebService for more information.</text>
  <text>For usage, please see: http://musicbrainz.org/development/mmd</text>
</error>"#;

    const XML_OK: &'static str = r#"<?xml version="1.0" encoding="UTF-8"?><metadata xmlns="http://musicbrainz.org/ns/mmd-2.0#"><release-group type="Album" type-id="f529b476-6e62-324f-b0aa-1f3e33d313fc" id="23c74936-ad4f-45bb-8b6b-527d4aeaaad6"><title>A.I Complex</title><first-release-date>2014-07-26</first-release-date><primary-type id="f529b476-6e62-324f-b0aa-1f3e33d313fc">Album</primary-type></release-group></metadata>"#;

    #[test]
    fn error() {
        let context = crate::util::musicbrainz_context();
        let reader = Reader::from_str(XML_ERR, Some(&context)).unwrap();

        let res = check_response_error(&reader);
        let err = res.err().unwrap();

        assert!(format!("{}", err).starts_with(
            "[server error]: Your requests are exceeding the allowable rate limit. Please see http://wiki.musicbrainz.org/XMLWebService for more information.\nFor usage, please see: http://musicbrainz.org/development/mmd"));
    }

    #[test]
    fn ok() {
        let context = crate::util::musicbrainz_context();
        let reader = Reader::from_str(XML_OK, Some(&context)).unwrap();

        // should not raise error
        check_response_error(&reader).unwrap();
    }
}
