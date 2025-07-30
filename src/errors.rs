use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CustomSheepstorError {
    details: String
}

impl CustomSheepstorError {
    pub fn new(msg: &str) -> CustomSheepstorError {
        CustomSheepstorError {details: msg.to_string()}
    }
}

impl fmt::Display for CustomSheepstorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CustomSheepstorError {
    fn description(&self) -> &str {
        &self.details
    }
}
