use failure::Fail;

#[derive(Debug,Fail)]
pub enum TransactionError{
    #[fail(display="Couldnot load file {}",0)]
    LoadError(std::io::Error),
    #[fail(display="Couldnot parse file {}",0)]
    ParseError(serde_json::Error),
    #[fail(display="Error {}",0)]
    Mess(&'static str),
}

impl From<std::io::Error> for TransactionError{
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError{
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

impl From<&'static str> for TransactionError{
    fn from(e: &'static str) -> Self {
        TransactionError::Mess(e)
    }
}