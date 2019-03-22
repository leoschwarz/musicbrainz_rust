use backtrace::Backtrace;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Error {
    // TODO: Make it possible to disable backtraces for performance reasons?
    backtrace: Backtrace,
    message: String,
    kind: ErrorKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub(crate) enum ErrorKind {
    /// Something went wrong while parsing a response.
    ParseResponse,

    /// Internal error of other kinds.
    Internal,

    /// Communication failed.
    Communication,

    /// The server returned an error message.
    ServerError,
}

impl ErrorKind {
    /// True if an error of this kind constitutes a bug that should ideally be reported to upstream.
    pub fn is_bug(&self) -> bool {
        match self {
            ErrorKind::ParseResponse | ErrorKind::Internal => true,
            ErrorKind::Communication | ErrorKind::ServerError => false,
        }
    }
}

impl Error {
    pub(crate) fn new<S: Into<String>>(msg: S, kind: ErrorKind) -> Error {
        Error {
            message: msg.into(),
            kind,
            backtrace: Backtrace::new()
        }
    }

    pub(crate) fn parse_error<S: Into<String>>(msg: S) -> Error {
        Error {
            message: msg.into(),
            kind: ErrorKind::ParseResponse,
            backtrace: Backtrace::new(),
        }
    }
}

impl std::error::Error for Error {
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.kind {
            ErrorKind::ParseResponse => {
                writeln!(f, "[parse response error]: {}", self.message)?;
            }
            ErrorKind::Internal => {
                writeln!(f, "[internal error]: {}", self.message)?;
            }
            ErrorKind::Communication => {
                writeln!(f, "[communication error]: {}", self.message)?;
            }
            ErrorKind::ServerError => {
                writeln!(f, "[server error]: {}", self.message)?;
            }
        }
        if self.kind.is_bug() {
            writeln!(f, "This might be a bug that should be reported upstream.")?;
        }
        writeln!(f, "Backtrace: {:?}", self.backtrace)?;
        Ok(())
    }
}

impl From<xpath_reader::Error> for Error {
    fn from(e: xpath_reader::Error) -> Self {
        Error {
            message: format!("xpath_reader error: {}", e),
            kind: ErrorKind::ParseResponse,
            backtrace: Backtrace::new(),
        }
    }
}

impl From<reqwest_mock::Error> for Error {
    fn from(e: reqwest_mock::Error) -> Self {
        Error {
            message: format!("reqwest_mock parse error: {}", e),
            kind: ErrorKind::Internal,
            backtrace: Backtrace::new(),
        }
    }
}

impl From<reqwest_mock::UrlError> for Error {
    fn from(e: reqwest_mock::UrlError) -> Self {
        Error {
            message: format!("reqwest_mock url error: {}", e),
            kind: ErrorKind::Internal,
            backtrace: Backtrace::new()
        }
    }
}
