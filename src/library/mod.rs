pub mod configuration_service;
pub mod rule_validation_error;
pub mod rule_validation_service;
pub mod rule_types;

mod error;

//Flatten the module structure
pub use error::{Error, Result};
