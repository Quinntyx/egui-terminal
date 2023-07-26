use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct TermConversionError;

impl Display for TermConversionError {
    // YES IK THIS IS BAD
    // @todo fix this later
    fn fmt (&self, _f: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}

impl std::error::Error for TermConversionError { }

