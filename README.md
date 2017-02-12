portunes
========

[![crates.io
version](https://img.shields.io/crates/v/portunes.svg)](https://crates.io/crates/portunes)
[![Build Status](https://travis-ci.org/casey/portunes.svg?branch=master)](https://travis-ci.org/casey/portunes)

Generate passwords with specified bits of entropy

- generate readme from help string

- default to alternating letters in two groups with a symbol separating them, and X bits of entropy

- alphabet: words, ascii, hiragana, katakana, hex, emoji,
  numbers, uppercase, symbols, braile, base36,  left handed
  letters, right handed letters, alternating letters, binary,
  octal, pronounceable english-like gibberish

- calculate bits of entropy from calls to choice
  . use unique items, not number of items

- deterministic rng for testing

- separator: none, space, dash, alphabet

- group size: 1 symbol, two symbols, etc

- bits of entropy

- recommendations for security
  basic online password, important online password, bitcoin wallet
  and some examples of passwords
