# easy_json_serde

Dead-simple JSON serialization / serialization

[`easy_json_serde`](https://crates.io/crates/easy_json_serde) works in
conjunction with [`serde`](https://crates.io/crates/serde).  Decorate your
`struct`s with `serde`'s `Serialize` and `Deserialize`, bring
`easy_json_serde`'s `EasyJsonSerialize` and `EasyJsonDeserialize` into view,
and easily serialize / deserialize to and from JSON.

```rust
use easy_json_serde::{EasyJsonDeserialize, EasyJsonSerialize};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct Dog {
    name: String,
    age: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "output_file.json";

    {
        let rufus = Dog {
            name: "Rufus".to_string(),
            age: 10,
        };
        let json_file = File::create(file_name)?;
        json_file.save(&rufus, "    ")?;
    }

    let rufus: Dog = {
        let mut json_file = File::open(file_name)?;
        Dog::load(&mut json_file)?
    };

    Ok(())
}

```
