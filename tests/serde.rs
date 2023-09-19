use degrees::Temp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Foo {
    temp: Temp,
}

#[test]
fn should_deserialize() {
    assert_eq!(
        serde_json::to_string(&Foo {
            temp: Temp::C(100.)
        })
        .unwrap(),
        "{\"temp\":{\"C\":100.0}}"
    );
}

#[test]
fn should_serialize() {
    assert_eq!(
        serde_json::from_str::<Foo>("{\"temp\":{\"C\":100.0}}").unwrap(),
        Foo {
            temp: Temp::C(100.)
        }
    );
}
