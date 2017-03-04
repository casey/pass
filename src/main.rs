extern crate clap;
extern crate rand;

#[cfg(test)]
mod tests;

mod prelude {
  pub type STR = &'static str;
}

use prelude::*;
use rand::Rng;

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

// abcdefghijklmnopqrstuvwxyz
// ABCDEFGHIJKLMNOPQRSTUVWXYZ
// !@#$%^&*()
// 0123456789
// qwerasdfzxcv

#[derive(Clone, Copy)]
enum Alphabet {
  Fast,
  Left,
  Right,
}

use Alphabet::*;

impl Alphabet {
  // fn new(symbols: &str) -> Alphabet{
  //   Alphabet {
  //     symbols: symbols.chars().collect()
  //   }
  // }
  fn symbols(self) -> Vec<String> {
    match self {
      Fast  => "asdfjkl;".chars().map(|c| c.to_string()).collect(),
      Left  => "qwertasdfgzxcvb".chars().map(|c| c.to_string()).collect(),
      Right => "yuiophjkl;nm,./".chars().map(|c| c.to_string()).collect(),
    }
  }

  fn name(self) -> STR {
    match self {
      Fast  => "fast",
      Left  => "left",
      Right => "right",
    }
  }

  fn from_name(name: &str) -> Option<Alphabet> {
    match name {
      "fast"  => Some(Fast),
      "left"  => Some(Left),
      "right" => Some(Right),
      _       => None,
    }
  }
}

static ALPHABETS: &'static [Alphabet] = &[
  Fast,
  Left,
];

fn run<I, T>(args: I) -> Result<String>
  where I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
{
  let alphabet_names = ALPHABETS.iter().cloned().map(Alphabet::name).collect::<Vec<_>>();

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
         .possible_values(&alphabet_names))
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

  let required_entropy: f64 = matches.value_of("bits").unwrap_or("128").parse().unwrap();
  let alphabet = Alphabet::from_name(matches.value_of("alphabet").unwrap_or("fast")).unwrap();
  let separator = matches.value_of("separator").unwrap_or("");
  let symbols = alphabet.symbols();
  let entropy_per_choice = (symbols.len() as f64).log2();
  let choices = (required_entropy / entropy_per_choice).ceil() as u64;

  let mut rng: Box<Rng> = if matches.is_present("ruin everything") {
    Box::new(rand::chacha::ChaChaRng::new_unseeded())
  } else {
    Box::new(rand::OsRng::new().unwrap())
  };

  Ok((0..choices)
    .map(|_| rng.choose(&symbols).unwrap().as_str())
    .collect::<Vec<_>>()
    .join(separator))
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
