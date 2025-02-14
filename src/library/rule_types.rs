use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum RuleType {
    SideJobPrevention,
    IndecisivePrevention,
    ExhaustionPrevention,
    LastMinuteActionPreventionForBooking,
    LastMinuteActionPreventionForCanceling,
}

impl FromStr for RuleType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SideJobPrevention" => Ok(RuleType::SideJobPrevention),
            "IndecisivePrevention" => Ok(RuleType::IndecisivePrevention),
            "ExhaustionPrevention" => Ok(RuleType::ExhaustionPrevention),
            "LastMinuteActionPreventionForBooking" => Ok(RuleType::LastMinuteActionPreventionForBooking),
            "LastMinuteActionPreventionForCanceling" => Ok(RuleType::LastMinuteActionPreventionForCanceling),
            _ => Err(()),
        }
    }
}

impl<'de> Deserialize<'de> for RuleType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        RuleType::from_str(&s).map_err(|_| D::Error::custom(format!("Unsupported RuleType: {}", s)))
    }
}
