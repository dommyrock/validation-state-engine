use std::str::FromStr;
use derive_more::From;
use quick_xml::DeError;
//derive_more enables us to use From without requiring us to implement Display on each error type.

use super::rule_types::RuleType;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(From, Debug)] //Serialize ,serde_as (crate) if we want automatic serialization for "Pretty" web error messages
pub enum Error {
    ValidationError(RuleType),

    //Other Module errors ...
    //#[from]
    //ConfigurationServiceError(crate::configuration_service::Error),

    // Externals
    #[from]
    StdError(Box<dyn std::error::Error>),
    #[from]
    IoError(std::io::Error),
    #[from]
    XMLError(quick_xml::Error),
    #[from]
    SerdeError(DeError),
    #[from]
    TokioError(tokio::time::error::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Error {
        Self::ValidationError(RuleType::from_str(s).expect(">> Invalid RuleType <<"))
    }
}
/* TOOD: 
We now get info about speciffic RuleType but, we still need some form of fine grained details From RuleValidationError


--->     pub struct RuleValidationError {
            rule_name: String,
            failure_reason: String,
            message: String,
            tag: String,
            success: bool,
            error_code: usize,
        }   

IF needed i can convert into my error type
impl From<io::Error> for AppError { //AppError = Error in my case
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
} */
impl std::error::Error for Error {}

//cargo test test_errors -- --nocapture
#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_errors() {
        let err = Error::ValidationError(RuleType::SideJobPrevention);
        let err2 = Error::ValidationError(RuleType::ExhaustionPrevention);
        let err3 = Error::ValidationError(RuleType::SideJobPrevention);
        let err4 = Error::ValidationError(RuleType::LastMinuteActionPreventionForBooking);
        let err5 = Error::ValidationError(RuleType::LastMinuteActionPreventionForCanceling);

        let new_err = Error::from("ExhaustionPrevention");
        dbg!(new_err);

        assert_eq!(format!("{}", err), "ValidationError(SideJobPrevention)");
        assert_eq!(format!("{}", err2), "ValidationError(ExhaustionPrevention)");
        assert_eq!(format!("{}", err3), "ValidationError(SideJobPrevention)");
        assert_eq!(
            format!("{}", err4),
            "ValidationError(LastMinuteActionPreventionForBooking)"
        );
        assert_eq!(
            format!("{}", err5),
            "ValidationError(LastMinuteActionPreventionForCanceling)"
        );
    }

    #[test]
    fn test_std_error() {
        let err = Error::StdError(Box::new(std::io::Error::new(
            std::io::ErrorKind::Deadlock,
            "'Deadlock' Error",
        )));
        assert_eq!(
            format!("{}", err),
            "StdError(Custom { kind: Deadlock, error: \"'Deadlock' Error\" })"
        );
    }

    #[test]
    fn test_io_error() {
        let err = Error::IoError(std::io::Error::new(
            std::io::ErrorKind::NotADirectory,
            "'NotADirectory' Error",
        ));
        assert_eq!(
            format!("{}", err),
            "IoError(Custom { kind: NotADirectory, error: \"'NotADirectory' Error\" })"
        );
    }

    #[test]
    fn test_xml_error() {
        let err = Error::XMLError(quick_xml::Error::Io(std::sync::Arc::new(
            std::io::Error::new(std::io::ErrorKind::NotFound, "'NotFound' Error"),
        )));
        assert_eq!(
            format!("{}", err),
            "XMLError(Io(Custom { kind: NotFound, error: \"'NotFound' Error\" }))"
        );
    }

    #[test]
    fn test_serde_error() {
        let err = Error::SerdeError(quick_xml::DeError::Custom("Custom Error".into()));
        assert_eq!(format!("{}", err), "SerdeError(Custom(\"Custom Error\"))");
    }
}
