mod version;

use version::Version;

fn main() {
    let sample = Version(0, 1, 2);

    println!("version: {}", sample.to_string());
}
