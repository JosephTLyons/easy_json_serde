use serde::{de::DeserializeOwned, Serialize};
use serde_json::{ser::PrettyFormatter, Serializer};
use std::error::Error;
use std::{fs::File, io::Read};

pub trait EasyJsonSerialize {
    fn save<T: Serialize>(
        &self,
        serializable_struct: &T,
        indent_string: &str,
    ) -> Result<(), Box<dyn Error>>;
}

impl EasyJsonSerialize for File {
    fn save<T: Serialize>(
        &self,
        serializable_type: &T,
        indent_string: &str,
    ) -> Result<(), Box<dyn Error>> {
        let formatter = PrettyFormatter::with_indent(indent_string.as_bytes());
        let mut serializer = Serializer::with_formatter(self, formatter);

        serializable_type.serialize(&mut serializer)?;

        Ok(())
    }
}

pub trait EasyJsonDeserialize<T: DeserializeOwned> {
    fn load(file: &mut File) -> Result<T, Box<dyn Error>>;
}

impl<T: DeserializeOwned> EasyJsonDeserialize<T> for T {
    fn load(file: &mut File) -> Result<T, Box<dyn Error>> {
        let mut json_string = String::new();
        file.read_to_string(&mut json_string)?;

        Ok(serde_json::from_str(&json_string)?)
    }
}
