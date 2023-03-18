use std::fmt::{Display, Formatter, Result};
use error_stack::Context;

#[derive(Debug)]
pub struct InvalidCommandError {
    pub message: String,
}

impl Display for InvalidCommandError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let full_msg = format!("Invalid command: {}", self.message);
        formatter.write_str(full_msg.as_str())
    }
}

impl Context for InvalidCommandError { }