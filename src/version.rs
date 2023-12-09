mod format;

#[derive(Debug, Eq, PartialEq)]
pub struct Version(pub u8, pub u8, pub u8);

impl Version {
    fn major(&self) -> u8 { self.0 }
    fn minor(&self) -> u8 { self.1 }
    fn patch(&self) -> u8 { self.2 }
}

