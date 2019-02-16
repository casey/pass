use std::collections::HashSet;

use crate::alphabet::{Alphabet, ALPHABETS};

#[test]
fn empty_alphabets() {
  for alphabet in ALPHABETS {
    assert!(alphabet.symbols().len() > 1);
  }
}

#[test]
fn repeated_symbols() {
  for alphabet in ALPHABETS {
    let symbols = alphabet.symbols();
    let unique = symbols.iter().collect::<HashSet<_>>().len();
    if symbols.len() != unique {
      panic!("alphabet {:?} contains repeated symbols", alphabet);
    }
  }
}

#[test]
fn alphabets_survive_round_trip() {
  for alphabet in ALPHABETS {
    assert_eq!(*alphabet, Alphabet::from_name(alphabet.name()).unwrap());
  }
}
