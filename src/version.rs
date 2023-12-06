use std::fmt::Display;
use std::{fmt, error};
use std::str::FromStr;
use regex::Regex;

pub struct Version(pub u8, pub u8, pub u8);

impl Version {
    fn major(&self) -> u8 { self.0 }
    fn minor(&self) -> u8 { self.1 }
    fn patch(&self) -> u8 { self.2 }
}

// TODO: Split parsing and displaying code to a different module?
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseVersionError;

impl Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse version")
    }
}

impl error::Error for ParseVersionError {}

// TODO: Handle more cases such as:
// * optional v
// * optional patch and minor??
impl FromStr for Version {
    type Err = ParseVersionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Cleaner error handling? Maybe implement ?
        // TODO: Once cell
        let RE = match Regex::new(r"v(\d+).(\d+).(\d+)") {
            Ok(it) => it,
            Err(_) => return Err(ParseVersionError),
        };

        let Some((_, parts)) =
            RE.captures(s)
                .map(|caps| caps.extract()) 
                else { return Err(ParseVersionError)}
                ;

        // XXX: I'm unwrapping, but it should never fail since
        // the regex only captures \d+, right?
        let [major, minor, patch] = parts
            .map(|p| p.parse().unwrap());

        Ok(Version(major, minor, patch))
    }
}
