use degrees::Temp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Foo {
    temp: Temp,
}

#[test]
fn shold_read_and_record_readings() {
    assert_eq!(
        serde_json::to_string(&Foo {
            temp: Temp::C(100.)
        })
        .unwrap(),
        "{\"temp\":{\"C\":100.0}}"
    );
}
