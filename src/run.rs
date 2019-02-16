use crate::common::*;

use crate::alphabet::{Alphabet::*, ALPHABETS};
use clap::{App, AppSettings, Arg};

pub fn run<I, T>(args: I) -> Result<String, Error>
where
  I: IntoIterator<Item = T>,
  T: Into<std::ffi::OsString> + Clone,
{
  let alphabet_names = ALPHABETS
    .iter()
    .cloned()
    .map(Alphabet::name)
    .collect::<Vec<&str>>();

  let matches = App::new(env!("CARGO_PKG_NAME"))
    .version(concat!("v", env!("CARGO_PKG_VERSION")))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(concat!(
      env!("CARGO_PKG_DESCRIPTION"),
      " - ",
      env!("CARGO_PKG_HOMEPAGE")
    ))
    .setting(AppSettings::ColoredHelp)
    .arg(
      Arg::with_name("alphabet")
        .short("a")
        .long("alphabet")
        .takes_value(true)
        .default_value(Alphanumeric.name())
        .possible_values(&alphabet_names),
    )
    .arg(
      Arg::with_name("separator")
        .short("s")
        .long("separator")
        .takes_value(true)
        .default_value(""),
    )
    .arg(
      Arg::with_name("bits of entropy")
        .short("b")
        .long("bits")
        .takes_value(true)
        .default_value("128"),
    )
    .arg(
      Arg::with_name("ruin everything")
        .help("Generate deterministic passwords for testing")
        .long("ruin-everything"),
    )
    .get_matches_from_safe(args)?;

  let required_entropy: f64 = matches
    .value_of("bits of entropy")
    .unwrap()
    .parse()
    .unwrap();

  let alphabet = Alphabet::from_name(matches.value_of("alphabet").unwrap()).unwrap();
  let separator = matches.value_of("separator").unwrap();
  let symbols = alphabet.symbols();
  let entropy_per_choice = (symbols.len() as f64).log2();
  let choices = (required_entropy / entropy_per_choice).ceil() as u64;

  let mut rng: Box<Rng> = if matches.is_present("ruin everything") {
    Box::new(rand::chacha::ChaChaRng::new_unseeded())
  } else {
    Box::new(rand::OsRng::new().unwrap())
  };

  Ok(
    (0..choices)
      .map(|_| rng.choose(&symbols).unwrap().as_str())
      .collect::<Vec<_>>()
      .join(separator),
  )
}
