// ===============================
// src/constructs/validator_construct.rs
// ===============================

use crate::invocation::invocation::{Invocation, InvocationResult, InvocationMode};
use crate::invocation::named_construct::NamedConstruct;
use crate::validator::validate_scroll;
use crate::scroll::Scroll;

pub struct Validator;

impl NamedConstruct for Validator {
    fn name(&self) -> &str {
        "Validator"
    }

    fn perform(&self, invocation: &Invocation, scroll: Option<Scroll>) -> Result<InvocationResult, String> {
        match invocation.mode {
            InvocationMode::Validate => {
                if let Some(scroll) = scroll {
                    validate_scroll(&scroll.yaml_metadata)?;
                    Ok(InvocationResult::Success("Validation passed.".into()))
                } else {
                    Err("No scroll provided to validate.".into())
                }
            }
            _ => Err("Validator only supports Validate invocation mode.".into()),
        }
    }
}