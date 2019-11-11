pub struct WalpurgisError(pub String);
pub type WalpurgisResult<T = ()> = Result<T, WalpurgisError>;

impl std::convert::From<ggez::error::GameError> for WalpurgisError {
    fn from(e: ggez::error::GameError) -> WalpurgisError {
        WalpurgisError(e.to_string())
    }
}

impl std::convert::From<String> for WalpurgisError {
    fn from(e: String) -> WalpurgisError {
        WalpurgisError(e)
    }
}

impl std::convert::From<std::io::Error> for WalpurgisError {
    fn from(e: std::io::Error) -> WalpurgisError {
        WalpurgisError(e.to_string())
    }
}

impl std::convert::From<ron::de::Error> for WalpurgisError {
    fn from(e: ron::de::Error) -> WalpurgisError {
        WalpurgisError(e.to_string())
    }
}
