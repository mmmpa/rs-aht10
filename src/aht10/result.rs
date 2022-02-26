pub type Aht10Result<T> = Result<T, Aht10Error>;

#[derive(Debug)]
pub enum Aht10Error {
    #[cfg(feature = "std")]
    I2cError(String),
    #[cfg(feature = "embedded")]
    I2cError(&'static str),
}

#[cfg(feature = "std")]
impl std::fmt::Display for Aht10Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Aht10Error {}
