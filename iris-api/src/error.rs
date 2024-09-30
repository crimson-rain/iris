#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Remove After Code Base Matures
    #[error("Generic Error: {0}")]
    Generic(String),

    /// Remove After Code Base Matures
    #[error("Static Error: {0}")]
	Static(&'static str),
}