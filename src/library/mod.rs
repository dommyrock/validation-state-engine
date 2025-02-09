use serde::Deserialize;

pub mod BusinessRuleError;
pub mod BusinessRuleService;
pub mod BusinessRuleXMLParser;
pub mod ConfigurationService;

//TO ME THIS READER Seems best at the moment
// https://crates.io/crates/quick-xml (there is also 'xml-rs' crate)

#[derive(Deserialize,Debug, Clone, PartialEq)]
pub enum RuleType { 
   SideJobPrevention,
   //oneother
   ExhaustionPrevention,
   LastMinuteBookingPrevention,
   LastMinuteCancellationPrevention,

}