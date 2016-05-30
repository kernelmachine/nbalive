
use std::error;
use std::fmt;
use url::ParseError;
use rustc_serialize::json::{EncoderError, DecoderError};
use hyper::Error as hyperError;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
/// Represents potential errors that may occur when profiling
pub enum NBAError {
    HeaderMissingError(&'static str),
    PostFailedError,
    URLParseError(ParseError),
    JSONEncoderError(EncoderError),
    JSONDecoderError(DecoderError),
    HyperError(hyperError),
    JsonError(SerdeJsonError),
    MissingField(&'static str),
    ArrayError,
    ObjectError,
    RegexError,
}

impl fmt::Display for NBAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NBAError::HeaderMissingError(ref elem) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[Header missing error at {} -- please file a bug. In \
                        bug report, please include the original query.",
                       elem)
            }
            NBAError::PostFailedError => {
                write!(f,
                       "\x1b[1;31merror: \x1b[Post Failed. Exiting now.",
                       )
            }
            NBAError::URLParseError(ref err) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[URL {}. Failed to parse. Try again.",
                       err)
            }
            NBAError::JSONEncoderError(ref err) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[JSON {} failed to encode. Try again.",
                       err)
            }
            NBAError::JSONDecoderError(ref err) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[JSON {} failed to decode. Try again.",
                       err)
            }
            NBAError::HyperError(ref err) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[Hyper failed at {}. Try again.",
                       err)
            }
            NBAError::JsonError(ref err) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[Json failed at {}. Try again.",
                       err)
            }
            NBAError::MissingField(ref key) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[Key {} does not exist. Submit bug with query.",
                       key)
            }
            NBAError::ArrayError => write!(f, "\x1b[1;31merror: \x1b[ArrayError. Try again."),
            NBAError::ObjectError => write!(f, "\x1b[1;31merror: \x1b[ObjectError. Try again."),
            NBAError::RegexError => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mRegex error -- please file a bug. In bug report, \
                        please include the original query.")
            }

        }
    }
}

impl error::Error for NBAError {
    fn description(&self) -> &str {
        match *self {
            NBAError::HeaderMissingError(_) => "Header missing. file bug.",
            NBAError::URLParseError(_) => "Parse failed. Try again.",
            NBAError::JSONEncoderError(_) => "Encoder failed. Try again.",
            NBAError::JSONDecoderError(_) => "Decoder failed. Try again.",
            NBAError::HyperError(_) => "Hyper failed. Try again.",
            NBAError::JsonError(_) => "JSON failed. Try again.",
            NBAError::MissingField(_) => "Missing field. Submit bug.",
            NBAError::ObjectError => "Could not convert JSON to an object.",
            NBAError::ArrayError => "Could not convert JSON to an array.",
            NBAError::PostFailedError => "Post failed.",
            NBAError::RegexError => "Regex error. file bug.",

        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            NBAError::HeaderMissingError(_) => None,
            NBAError::URLParseError(_) => None,
            NBAError::JSONEncoderError(_) => None,
            NBAError::JSONDecoderError(_) => None,
            NBAError::HyperError(_) => None,
            NBAError::JsonError(_) => None,
            NBAError::MissingField(_) => None,
            NBAError::ArrayError => None,
            NBAError::ObjectError => None,
            NBAError::PostFailedError => None,
            NBAError::RegexError => None,

        }
    }
}



impl From<ParseError> for NBAError {
    fn from(err: ParseError) -> NBAError {
        NBAError::URLParseError(err)
    }
}

impl From<EncoderError> for NBAError {
    fn from(err: EncoderError) -> NBAError {
        NBAError::JSONEncoderError(err)
    }
}

impl From<DecoderError> for NBAError {
    fn from(err: DecoderError) -> NBAError {
        NBAError::JSONDecoderError(err)
    }
}

impl From<hyperError> for NBAError {
    fn from(err: hyperError) -> NBAError {
        NBAError::HyperError(err)
    }
}

impl From<SerdeJsonError> for NBAError {
    fn from(err: SerdeJsonError) -> NBAError {
        NBAError::JsonError(err)
    }
}
