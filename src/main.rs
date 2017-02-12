extern crate clap;

#[macro_use]
extern crate error_chain;

use clap::{App, Arg, AppSettings};

mod error {
  error_chain!{
    foreign_links {
      Clap(::clap::Error);
    }
  }
}

use error::*;

struct Alphabet {
  symbols: Vec<char>
}

impl Alphabet {
  fn new(symbols: &str) -> Alphabet{
    Alphabet {
      symbols: symbols.chars().collect()
    }
  }
}

fn run<I, T>(args: I) -> Result<String>
  where I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
{
  let matches = App::new(env!("CARGO_PKG_NAME"))
    .version(concat!("v", env!("CARGO_PKG_VERSION")))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(concat!(env!("CARGO_PKG_DESCRIPTION"),
                   " - ",
                   env!("CARGO_PKG_HOMEPAGE")))
    .setting(AppSettings::ColoredHelp)
    .arg(Arg::with_name("alphabet")
         .short("a")
         .long("alphabet")
         .takes_value(true)
         .possible_values(&["left"]))
    .arg(Arg::with_name("separator")
         .short("s")
         .long("separator")
         .takes_value(true))
    .arg(Arg::with_name("group size")
         .short("g")
         .long("group")
         .takes_value(true))
    .arg(Arg::with_name("bits of entropy")
         .short("b")
         .long("bits")
         .takes_value(true))
    .arg(Arg::with_name("ruin everything")
         .help("Generate deterministic passwords for testing")
         .long("ruin-everything"))
    .get_matches_from_safe(args)?;

  Ok("hello".to_string())
}

fn main() {
  match run(std::env::args_os()) {
    Ok(password) => println!("{}", password),
    Err(ref e) => {
      if let Error(ErrorKind::Clap(ref clap_error), _) = *e {
        use clap::ErrorKind::{HelpDisplayed, VersionDisplayed};
        match clap_error.kind {
          HelpDisplayed | VersionDisplayed => return,
          _ => std::process::exit(1),
        }
      }

      println!("error: {}", e);

      for e in e.iter().skip(1) {
        println!("caused by: {}", e);
      }

      if let Some(backtrace) = e.backtrace() {
        println!("backtrace: {:?}", backtrace);
      }

      std::process::exit(1);
    }
  }
}

#[cfg(test)]
mod tests;
