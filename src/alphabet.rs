use Alphabet::*;

const DIGITS: &str = "0123456789";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const PUNCTUATION: &str = r#"~`!@#$%^&*()-_=+[{]}\|;:'",<.>/?"#;
const WORDS: &str = include_str!("words.txt");

fn chars(s: &str) -> Vec<String> {
  s.chars().map(|c| c.to_string()).collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Alphabet {
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

pub static ALPHABETS: &'static [Alphabet] = &[
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

impl Alphabet {
  pub fn symbols(self) -> Vec<String> {
    match self {
      Digits => chars(DIGITS),
      Letters => chars(DIGITS),
      Lowercase => chars(LOWERCASE),
      Uppercase => chars(UPPERCASE),
      Fast => chars("asdfjkl;"),
      Hex => chars("0123456789abcdefABCDEF"),
      HexLowercase => chars("0123456789abcdef"),
      HexUppercase => chars("0123456789ABCDEF"),
      Left => chars("qwertasdfgzxcvb"),
      Octal => chars("01234567"),
      Right => chars("yuiophjkl;nm,./"),
      Punctuation => chars(PUNCTUATION),
      Graphical => chars(DIGITS)
        .into_iter()
        .chain(chars(UPPERCASE).into_iter())
        .chain(chars(LOWERCASE).into_iter())
        .chain(chars(PUNCTUATION).into_iter())
        .collect(),
      Alphanumeric => chars(DIGITS)
        .into_iter()
        .chain(chars(UPPERCASE).into_iter())
        .chain(chars(LOWERCASE).into_iter())
        .collect(),
      Words => WORDS
        .split_whitespace()
        .map(str::to_string)
        .collect(),
    }
  }

  pub fn name(self) -> &'static str {
    match self {
      Alphanumeric => "alphanumeric",
      Digits => "digits",
      Fast => "fast",
      Graphical => "graphical",
      Hex => "hex",
      HexLowercase => "hex-lowercase",
      HexUppercase => "hex-uppercase",
      Left => "left",
      Letters => "letters",
      Lowercase => "lowercase",
      Octal => "octal",
      Punctuation => "punctuation",
      Right => "right",
      Uppercase => "uppercase",
      Words => "words",
    }
  }

  pub fn from_name(name: &str) -> Option<Alphabet> {
    match name {
      "alphanumeric" => Some(Alphanumeric),
      "digits" => Some(Digits),
      "fast" => Some(Fast),
      "graphical" => Some(Graphical),
      "hex" => Some(Hex),
      "hex-lowercase" => Some(HexLowercase),
      "hex-uppercase" => Some(HexUppercase),
      "left" => Some(Left),
      "letters" => Some(Letters),
      "lowercase" => Some(Lowercase),
      "octal" => Some(Octal),
      "punctuation" => Some(Punctuation),
      "right" => Some(Right),
      "uppercase" => Some(Uppercase),
      "words" => Some(Words),
      _ => None,
    }
  }
}
