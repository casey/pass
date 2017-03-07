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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Alphabet {
  Alphanumeric,
  Digits,
  Letters,
  Lowercase,
  Uppercase,
  Fast,
  Hex,
  HexLowercase,
  HexUppercase,
  Left,
  Octal,
  Graphical,
  Punctuation,
  Right,
  Words,
}

use Alphabet::*;

static DIGITS:      STR = "0123456789";
static LOWERCASE:   STR = "abcdefghijklmnopqrstuvwxyz";
static UPPERCASE:   STR = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static PUNCTUATION: STR = r#"~`!@#$%^&*()-_=+[{]}\|;:'",<.>/?"#;

impl Alphabet {
  fn symbols(self) -> Vec<String> {
    match self {
      Digits         => chars(DIGITS),
      Letters        => chars(DIGITS),
      Lowercase      => chars(LOWERCASE),
      Uppercase      => chars(UPPERCASE),
      Fast           => chars("asdfjkl;"),
      Hex            => chars("0123456789abcdefABCDEF"),
      HexLowercase   => chars("0123456789abcdef"),
      HexUppercase   => chars("0123456789ABCDEF"),
      Left           => chars("qwertasdfgzxcvb"),
      Octal          => chars("01234567"),
      Right          => chars("yuiophjkl;nm,./"),
      Punctuation    => chars(PUNCTUATION),
      Graphical      => 
        chars(DIGITS).into_iter()
        .chain(chars(UPPERCASE).into_iter())
        .chain(chars(LOWERCASE).into_iter())
        .chain(chars(PUNCTUATION).into_iter())
        .collect(),
      Alphanumeric   => 
        chars(DIGITS).into_iter()
        .chain(chars(UPPERCASE).into_iter())
        .chain(chars(LOWERCASE).into_iter())
        .collect(),
      Words          => include_str!("words.txt")
        .split_whitespace()
        .map(str::to_string)
        .collect(),
    }
  }

  fn name(self) -> STR {
    match self {
      Alphanumeric   => "alphanumeric",
      Digits         => "digits",
      Fast           => "fast",
      Graphical      => "graphical",
      Hex            => "hex",
      HexLowercase   => "hex-lowercase",
      HexUppercase   => "hex-uppercase",
      Left           => "left",
      Letters        => "letters",
      Lowercase      => "lowercase",
      Octal          => "octal",
      Punctuation    => "punctuation",
      Right          => "right",
      Uppercase      => "uppercase",
      Words          => "words",
    }
  }

  fn from_name(name: &str) -> Option<Alphabet> {
    match name {
      "alphanumeric"  => Some(Alphanumeric),
      "digits"        => Some(Digits),
      "fast"          => Some(Fast),
      "graphical"     => Some(Graphical),
      "hex"           => Some(Hex),
      "hex-lowercase" => Some(HexLowercase),
      "hex-uppercase" => Some(HexUppercase),
      "left"          => Some(Left),
      "letters"       => Some(Letters),
      "lowercase"     => Some(Lowercase),
      "octal"         => Some(Octal),
      "punctuation"   => Some(Punctuation),
      "right"         => Some(Right),
      "uppercase"     => Some(Uppercase),
      "words"         => Some(Words),
      _               => None,
    }
  }
}

static ALPHABETS: &'static [Alphabet] = &[
  Alphanumeric,
  Digits,
  Letters,
  Lowercase,
  Uppercase,
  Fast,
  Hex,
  HexLowercase,
  HexUppercase,
  Left,
  Octal,
  Graphical,
  Punctuation,
  Right,
  Words,
];

fn chars(s: &str) -> Vec<String> {
  s.chars().map(|c| c.to_string()).collect()
}

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
  let alphabet = Alphabet::from_name(matches.value_of("alphabet").unwrap_or("hex-lower")).unwrap();
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
