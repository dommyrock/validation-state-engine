pub struct BusinessRuleError {
    rule_id: String, 
    failure_reason: String, //business_rule",
    message: String, //"A shift that starts within 60 minutes cannot be self-cancelled, please call your Local Office for assistance.",
    tag: String,     //"Any CandidateStatusId",
    success: bool,
}

impl std::fmt::Display for BusinessRuleError {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Business rule '{}' failed: {}", self.rule_id, self.message)
   }
}

impl std::fmt::Debug for BusinessRuleError {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      f.debug_struct("BusinessRuleError")
         .field("rule_id", &self.rule_id)
         .field("failure_reason", &self.failure_reason)
         .field("message", &self.message)
         .field("tag", &self.tag)
         .field("success", &self.success)
         .finish()
   }
}

impl std::error::Error for BusinessRuleError {}