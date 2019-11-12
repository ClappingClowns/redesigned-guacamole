#[derive(Debug)]
pub enum WalpurgisError {
    GGEZ(ggez::error::GameError),
    IO(std::io::Error),
    Ron(ron::de::Error),
    Generic(String),
}
pub type WalpurgisResult<T = ()> = Result<T, WalpurgisError>;

impl std::convert::From<ggez::error::GameError> for WalpurgisError {
    fn from(e: ggez::error::GameError) -> WalpurgisError {
        WalpurgisError::GGEZ(e)
    }
}

impl std::convert::From<String> for WalpurgisError {
    fn from(e: String) -> WalpurgisError {
        WalpurgisError::Generic(e)
    }
}

impl std::convert::From<std::io::Error> for WalpurgisError {
    fn from(e: std::io::Error) -> WalpurgisError {
        WalpurgisError::IO(e)
    }
}

impl std::convert::From<ron::de::Error> for WalpurgisError {
    fn from(e: ron::de::Error) -> WalpurgisError {
        WalpurgisError::Ron(e)
    }
}
