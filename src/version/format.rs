use super::*;
use std::{fmt, error, str};
use regex::Regex;
use once_cell::sync::Lazy;

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseVersionError;

impl fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse version")
    }
}

impl error::Error for ParseVersionError {}

impl str::FromStr for Version {
    type Err = ParseVersionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"v?(\d+).(\d+).(\d+)").unwrap());

        let (_, parts) =
            RE.captures(s)
                .map(|caps| caps.extract()) 
                .ok_or(ParseVersionError)?
                ;

        // XXX: I'm unwrapping, but it should never fail since
        // the regex only captures \d+, right?
        let [major, minor, patch] = parts
            .map(|p| p.parse().unwrap());

        Ok(Version(major, minor, patch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_formats() {
        let v = Version(0, 1, 2);
        assert_eq!("v0.1.2", v.to_string());
    }

    #[test]
    fn correctly_parses() {
        let v = "v1.0.2";
        assert_eq!(Version(1, 0, 2), v.parse().unwrap());
    }

    #[test]
    fn fails_parsing() {
        let v = "v1";
        assert_eq!(Err(ParseVersionError), v.parse::<Version>())
    }

    #[test]
    fn parses_without_v() {
        let v = "1.0.2";
        assert_eq!(Version(1, 0, 2), v.parse().unwrap())
    }
}
