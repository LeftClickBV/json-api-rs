//! The `Error` struct, the `Result` alias, and other tools to handle failure.

use std::io::Error as IoError;
use std::str::Utf8Error;

use error_chain::error_chain;
use http::status::InvalidStatusCode as InvalidStatusCodeError;
use http::uri::InvalidUri as InvalidUriError;
use serde_json::Error as JsonError;
use serde_qs::Error as QueryError;

error_chain! {
    foreign_links {
        InvalidStatusCode(InvalidStatusCodeError);
        InvalidUri(InvalidUriError);
        Io(IoError);
        Json(JsonError);
        Query(QueryError);
        Utf8(Utf8Error);
    }

    errors {
        InvalidMemberName(name: String) {
            description("TODO")
            display("TODO")
        }

        MissingField(name: String) {
            description("A struct was built without a required field.")
            display(r#"missing required field "{}""#, name)
        }

        PayloadTooLarge(size: u64) {
            description("The payload is too large")
            display(r#"payload exceeds limit of {} B"#, size)
        }

        UnsupportedVersion(version: String) {
            description("The specified version of is not \
                         supported by this implementation.")
            display(r#"Version "{}" is not yet supported by \
                       this implementation."#, version)
        }
    }
}

impl Error {
    pub fn missing_field(name: &str) -> Self {
        Self::from(ErrorKind::MissingField(name.to_owned()))
    }

    pub fn payload_too_large(size: u64) -> Self {
        Self::from(ErrorKind::PayloadTooLarge(size))
    }

    pub fn unsupported_version(version: &str) -> Self {
        Self::from(ErrorKind::UnsupportedVersion(version.to_owned()))
    }
}
