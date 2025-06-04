use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZettariumError {
    #[error("Invalid note type: {0}")]
    InvalidNoteType(String),
}
