use crate::config::QwerConfig;
use snafu::{Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct ListPackages {}

impl super::command::Command for ListPackages {
    type Error = Error;

    fn apply(self, config: &QwerConfig) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    HttpError,
}
