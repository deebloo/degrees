# degrees

Easily handle temperature values in rust. The goal is to work with temps without needing to think about about units at all. You can compare and combine temps safely without manually converting to and from different units.
For example you can subtract 10degC from 86degF. The crate will handle the conversions internally and give you a result in the initial unit.

```rust
use degrees:Temp;

let value = Temp::F(86.) - Temp::C(10.); // Temp::F(36.0)
```

This also means you can safely compare temperatures in two different units

```rust
use degrees:Temp;

let value = Temp::F(86.) == Temp::C(30.); // true
```

To enable serde serialization.

```toml
[dependencies]
degrees = { version = "0.3", features = ["serde"]}
```
