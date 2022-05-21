use serde::{de::DeserializeOwned, Serialize};
use serde_json::{ser::PrettyFormatter, Serializer};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;

pub trait EasyJsonSerialize {
    fn save<P: AsRef<Path>, T: Serialize>(
        path: P,
        serializable_type: &T,
        indent_string: &str,
    ) -> Result<File, Box<dyn Error>>;
}

impl EasyJsonSerialize for File {
    fn save<P: AsRef<Path>, T: Serialize>(
        path: P,
        serializable_type: &T,
        indent_string: &str,
    ) -> Result<File, Box<dyn Error>> {
        let formatter = PrettyFormatter::with_indent(indent_string.as_bytes());
        let file = OpenOptions::new().write(true).create(true).open(path)?;
        let mut serializer = Serializer::with_formatter(&file, formatter);

        serializable_type.serialize(&mut serializer)?;

        Ok(file)
    }
}

pub trait EasyJsonDeserialize<T> {
    fn load(file: &mut File) -> Result<T, Box<dyn Error>>;
}

impl<T: DeserializeOwned> EasyJsonDeserialize<T> for T {
    fn load(file: &mut File) -> Result<T, Box<dyn Error>> {
        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;

        Ok(serde_json::from_str(&json_string)?)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::{EasyJsonDeserialize, EasyJsonSerialize};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Dog {
        name: String,
        age: u8,
    }

    #[test]
    fn test_serialization_and_deserialization() {
        let rufus_original = Dog {
            name: "Rufus".to_string(),
            age: 10,
        };

        let file_name = "test.json";
        let indentation_string = "    ";
        File::save(file_name, &rufus_original, indentation_string).unwrap();

        let mut json_file = File::open(file_name).unwrap();
        let rufus_deserialized: Dog = Dog::load(&mut json_file).unwrap();

        assert_eq!(rufus_original, rufus_deserialized)
    }
}
