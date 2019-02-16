#[derive(Debug)]
pub enum Error {
  Clap{clap_error: clap::Error},
}

impl From<clap::Error> for Error {
  fn from(clap_error: clap::Error) -> Error {
    Error::Clap{clap_error}
  }
}
