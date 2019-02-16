#[cfg(test)]
mod tests;
mod error;
mod alphabet;
mod run;
mod common;

use crate::common::*;

use run::run;

fn main() {
  match run(std::env::args_os()) {
    Ok(password) => println!("{}", password),
    Err(e) => {
      match e {
        Error::Clap{clap_error} => {
          use clap::ErrorKind::{HelpDisplayed, VersionDisplayed};
          match clap_error.kind {
            HelpDisplayed | VersionDisplayed => return,
            _ => std::process::exit(1),
          }
        },
      }
    }
  }
}
