# easy_json_serde

Dead-simple JSON serialization / deserialization

[`easy_json_serde`](https://crates.io/crates/easy_json_serde) works in
conjunction with [`serde`](https://crates.io/crates/serde).  Decorate your
`struct`s with `serde`'s `Serialize` and `Deserialize`, bring
`easy_json_serde`'s `EasyJsonSerialize` and `EasyJsonDeserialize` into view,
and easily serialize / deserialize to and from JSON.

```rust
use std::fs::File;

use easy_json_serde::{EasyJsonDeserialize, EasyJsonSerialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Dog {
    name: String,
    age: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rufus_original = Dog {
        name: "Rufus".to_string(),
        age: 10,
    };

    let indentation_string = "    ";
    File::save(file_name, &rufus_original, indentation_string)?;
    let file_name = "dog.json";

    let mut json_file = File::open(file_name)?;
    let _rufus_deserialized: Dog = Dog::load(&mut json_file)?;

    Ok(())
}
```
