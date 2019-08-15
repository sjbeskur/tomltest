#[macro_use]
extern crate serde_derive;
extern crate serde;

mod creds;
use toml;

fn main() {
    if let Ok(contents) = std::fs::read_to_string("creds.toml"){
        println!("{}",contents);
        let c: creds::Creds = toml::from_str(&contents).unwrap();
        assert_eq!(c.default.key, "entersomekey");
        assert_eq!(c.default.secret, "entersomesecret");
    }
}
