mod cmdline_utils;
mod version_provider;

fn main() {
    let v = version_provider::Version::new("9.3.1");

    // println!("{:?}", v);
}
