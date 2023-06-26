#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    FactorioBlueprint(#[from] factorio_blueprint::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;
