//! Test that deserializer lifetimes of `from_str()` are more general than those
//! for `from_reader()`. This enables more use-cases.
//!
//! See the [serde documentation] on that topic and have a look at the same
//! [example using `serde_json`][playground].
//!
//! [serde documentation]: https://serde.rs/lifetimes.html
//! [playground]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=fd61dbe68200a8f40ba9fb41381f043c
#![cfg(feature = "serialize")]

extern crate quick_xml;
extern crate serde;

use serde::Deserialize;

#[derive(Deserialize)]
struct User<'a> {
    id: u32,
    name: &'a str,
    screen_name: &'a str,
    location: &'a str,
}

#[test]
fn deserializer_lifetime() {
    const XML: &str = r#"<User id="1" name="tafia" screen_name="Johann" location="Hong Kong"/>"#;
    let user: User = quick_xml::de::from_str(XML).expect("Could not deserialize");
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "tafia");
    assert_eq!(user.screen_name, "Johann");
    assert_eq!(user.location, "Hong Kong");
}
