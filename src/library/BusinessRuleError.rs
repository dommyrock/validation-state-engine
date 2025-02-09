pub struct BusinessRuleError {
    rule_name: String,
    failure_reason: String, //business_rule",
    message: String, //"A shift that starts within 60 minutes cannot be self-cancelled, please call your Local Office for assistance.",
    tag: String,     //"Any CandidateStatusId",
    success: bool,
    error_code: u16,
}
pub const ERR_CODE: u16 = 403;

impl std::fmt::Display for BusinessRuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Business rule '{}' failed: {} | CODE: {}",
            self.rule_name, self.message,self.error_code
        )
    }
}

impl std::fmt::Debug for BusinessRuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BusinessRuleError")
            .field("rule_name", &self.rule_name)
            .field("failure_reason", &self.failure_reason)
            .field("message", &self.message)
            .field("tag", &self.tag)
            .field("success", &self.success)
            .field("error_code", &self.error_code)
            .finish()
    }
}

impl std::error::Error for BusinessRuleError {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /* Example command to runt this test
    (NOW) cargo test -- --nocapture
    (FUTURE) cargo test --lib BusinessRuleError -- --nocapture (when we move it to --lib )*/

    fn produce_error() -> Result<(), BusinessRuleError> {
        Err(BusinessRuleError {
            rule_name: "ShiftCancellation".to_string(),
            failure_reason: "business_rule".to_string(),
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
        let error = BusinessRuleError {
            rule_name: "ShiftCancellation".to_string(),
            failure_reason: "business_rule".to_string(),
            message: "A shift that starts within 60 minutes cannot be self-cancelled, please call your Local Office for assistance.".to_string(),
            tag: "Missmatch CandidateId".to_string(),
            success: false,
            error_code: ERR_CODE,
        };

        assert_eq!(
            format!("{:?}", error),
            "BusinessRuleError { rule_name: \"ShiftCancellation\", failure_reason: \"business_rule\", message: \"A shift that starts within 60 minutes cannot be self-cancelled, please call your Local Office for assistance.\", tag: \"Missmatch CandidateId\", success: false, error_code: 403 }"
        );
    }
}
