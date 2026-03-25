use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SMTError {
    #[error("Key {0} already exists")]
    KeyAlreadyExist(String),
    #[error("Key {0} does not exist")]
    KeyDoesNotExist(String),
    #[error("Parameter {name} must be a {expected_type}")]
    InvalidParameterType { name: String, expected_type: String },
    #[error("Invalid sibling index")]
    InvalidSiblingIndex,
}
