use std::fmt;

struct Version(u8, u8, u8);

impl Version {
    fn major(&self) -> u8 { self.0 }
    fn minor(&self) -> u8 { self.1 }
    fn patch(&self) -> u8 { self.2 }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

fn main() {
    let sample = Version(0, 1, 2);

    println!("version: {}", sample.to_string());
}
