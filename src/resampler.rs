use sample::*;
use data::value::*;

mod error {
    #[derive(Debug, Fail)]
    pub enum Error {
        #[fail(display = "Invalid Data")]
        InvalidData,
        #[fail(display = "Additional data needed")]
        MoreDataNeeded,
        #[fail(display = "Configuration Incomplete")]
        ConfigurationIncomplete,
        #[fail(display = "Configuration Invalid")]
        ConfigurationInvalid,
        #[fail(display = "Unsupported feature {}", _0)]
        Unsupported(String),
        // TODO add support for dependency-specific errors here
        // Inner(failure::Context)
    }

    pub type Result<T> = ::std::result::Result<T, Error>;
}

use self::error::*;

pub trait Resampler<S: SampleType> {
    fn configure(&mut self) -> Result<()>;
    fn set_option<'a>(&mut self, key: &str, val: Value<'a>) -> Result<()>;
    fn send(&mut self, &[S]) -> Result<usize>;
    fn receive(&mut self, &mut [S]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}
