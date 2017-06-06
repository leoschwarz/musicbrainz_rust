error_chain! {
    types {
        ParseError, ParseErrorKind, ChainParseErr;
    }

    links {
        XpathReadError(::xpath_reader::XpathError, ::xpath_reader::XpathErrorKind);
    }

    // Automatic conversions between this error chain and errors not defined using error
    // chain.
    foreign_links {
        UuidParseError(::uuid::ParseError);
        ParseIntError(::std::num::ParseIntError);
        ParseDateError(super::entities::ParseDateError);
    }

    // Custom error kinds.
    errors {
        InvalidData(msg: String) {
            description("invalid data")
            display("invalid data: {}", msg)
        }
        /// Somewhere in our code something went wrong, that really shouldn't have.
        /// These are always considered a bug that should reported as an issue.
        InternalError(msg: String) {
            description("internal error")
            display("internal error: {}\nYou should probably report this bug.", msg)
        }
    }
}

error_chain!{
    types {
        ClientError, ClientErrorKind, ChainClientErr;
    }

    links {
        ParseError(ParseError, ParseErrorKind);
        XpathReadError(::xpath_reader::XpathError, ::xpath_reader::XpathErrorKind);
        ReqwestMockError(::reqwest_mock::Error, ::reqwest_mock::error::ErrorKind);
    }

    foreign_links {
        IoError(::std::io::Error);
        Url(::reqwest_mock::UrlError);
    }

    errors {
        /// This is most likely a rate limit, but since the errors aren't coded and there are
        /// also other issues like the server being busy we don't distinguish it except for the
        /// provided message.
        MusicbrainzServerError(msg: String) {
            description("MusicBrainz server error")
            display("MusicBrainz server error: {}", msg)
        }
    }
}
