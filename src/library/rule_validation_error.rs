

//NOTE:: THIS IS NOT ACTUALLY USED YET 

#[derive(Debug)]
pub struct RuleValidationError {
    rule_name: String,
    failure_reason: String,
    message: String,
    tag: String,
    success: bool,
    error_code: u16,
}
pub const ERR_CODE: u16 = 403;
pub const FAIL_REASON: &str = "rule_validation";

impl std::fmt::Display for RuleValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Business rule '{}' failed: {} | CODE: {}",
            self.rule_name, self.message, self.error_code
        )
    }
}

impl std::error::Error for RuleValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /* Example command to runt this test
    (NOW) cargo test -- --nocapture
    (FUTURE) cargo test --lib business_rule_error -- --nocapture (when we move it to --lib )*/

    fn produce_error() -> Result<(), RuleValidationError> {
        Err(RuleValidationError {
            rule_name: "ShiftCancellation".to_string(),
            failure_reason: FAIL_REASON.to_string(),
            message: "A shift that starts within 60 minutes cannot be self-cancelled, please call your Local Office for assistance.".to_string(),
            tag: "Missmatch CandidateId".to_string(),
            success: false,
            error_code: ERR_CODE,
        })
    }

    #[test]
    fn test_display() {
        match produce_error() {
            Err(e) => eprintln!("{:?}", e),
            _ => println!("No error"),
        }

        //Wrap it into Err()
        eprintln!("\n\n{:?}", produce_error()); //--nocapture to see the output

        assert_eq!(
            format!("{}", produce_error().unwrap_err()),
            "Business rule 'ShiftCancellation' failed: A shift that starts within 60 minutes cannot be self-cancelled, please call your Local Office for assistance. | CODE: 403"
        );
    }

    #[test]
    fn test_debug() {
        let error = RuleValidationError {
            rule_name: "ShiftCancellation".to_string(),
            failure_reason: FAIL_REASON.to_string(),
            message: "A shift that starts within 60 minutes cannot be self-cancelled, please call your Local Office for assistance.".to_string(),
            tag: "Missmatch CandidateId".to_string(),
            success: false,
            error_code: ERR_CODE,
        };

        assert_eq!(
            format!("{:?}", error),
            "RuleValidationError { rule_name: \"ShiftCancellation\", failure_reason: \"rule_validation\", message: \"A shift that starts within 60 minutes cannot be self-cancelled, please call your Local Office for assistance.\", tag: \"Missmatch CandidateId\", success: false, error_code: 403 }"
        );
    }
}
