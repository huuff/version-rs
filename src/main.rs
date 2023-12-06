use indoc::indoc;

struct Version(u8, u8, u8);

impl Version {
    fn major(&self) -> u8 { self.0 }
    fn minor(&self) -> u8 { self.1 }
    fn patch(&self) -> u8 { self.2 }
}

fn main() {
    let sample = Version(0, 1, 2);

    println!(indoc! {"
        major: {}
        minor: {}
        patch: {}
    "}, sample.major(), sample.minor(), sample.patch());
}
