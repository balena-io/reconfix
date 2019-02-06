use serde_derive::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ValidationError {
    keyword: String,
    #[serde(rename = "schemaPath")]
    schema_path: String,
    #[serde(rename = "dataPath")]
    data_path: String,
    message: String,
}

impl ValidationError {
    pub fn keyword(&self) -> &str {
        &self.keyword
    }

    pub fn schema_path(&self) -> &str {
        &self.schema_path
    }

    pub fn data_path(&self) -> &str {
        &self.data_path
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl ValidationError {
    pub fn new<S1, S2, S3, S4>(keyword: S1, schema_path: S2, data_path: S3, message: S4) -> ValidationError
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        ValidationError {
            keyword: keyword.into(),
            schema_path: schema_path.into(),
            data_path: data_path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "schema path: '{}', data path: '{}', keyword: '{}', message: '{}'",
            self.schema_path, self.data_path, self.keyword, self.message
        )
    }
}

impl std::error::Error for ValidationError {}
