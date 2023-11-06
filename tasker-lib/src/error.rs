use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskerError {
    #[error("failed to determine project directories")]
    ProjectDirectoryError(#[from] std::io::Error),

    #[error("failed to deserialize file: `{0}`")]
    DeserializationError(#[from] ron::error::SpannedError),

    #[error("failed to serialize file: `{0}`")]
    SerializationError(#[from] ron::error::Error),

    #[error("failed to deserialize config file: `{0}`")]
    ConfigDeserializationError(#[from] toml::de::Error),
}
