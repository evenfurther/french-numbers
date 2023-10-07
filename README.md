# french-numbers

[![Current Version](https://img.shields.io/crates/v/french-numbers.svg)](https://crates.io/crates/french-numbers)
[![Documentation](https://docs.rs/french-numbers/badge.svg)](https://docs.rs/french-numbers)
[![License: Apache-2.0/MIT](https://img.shields.io/crates/l/french-numbers.svg)](#license)

This crate transforms a number into its French representation.

## Using this crate

In your `Cargo.toml`, add:

``` ini
[dependencies]
french-numbers = "1.2.0"
```

You can then use the `french_number` function from the `french_numbers` crate to format any integer into the French representation:

``` rust
use french_numbers::french_number;

assert_eq!(french_number(&71), "soixante-et-onze");
assert_eq!(french_number(&1001), "mille-un");
assert_eq!(french_number(&-200001), "moins deux-cent-mille-un");
assert_eq!(french_number(&-200000001), "moins deux-cents-millions-un");
assert_eq!(french_number(&-204000001), "moins deux-cent-quatre-millions-un");
```

You can also request the use of the feminine form, or prefer the previous way of writing numbers predating the 1990 orthographic reform:

``` rust
use french_numbers::*;

assert_eq!(french_number_options(&37251061, &POST_REFORM_MASCULINE),
           "trente-sept-millions-deux-cent-cinquante-et-un-mille-soixante-et-un");
assert_eq!(french_number_options(&37251061, &POST_REFORM_FEMININE),
           "trente-sept-millions-deux-cent-cinquante-et-un-mille-soixante-et-une");
assert_eq!(french_number_options(&37251061, &PRE_REFORM_FEMININE),
           "trente-sept millions deux cent cinquante et un mille soixante et une");
assert_eq!(french_number_options(&37251061, &PRE_REFORM_MASCULINE),
           "trente-sept millions deux cent cinquante et un mille soixante et un")
```

An example program can dump particular numbers, with various options combinations:

``` bash
% cargo run --bin french-numbers --features cli -- --help
Represent numbers in French language

Usage: french-numbers [OPTIONS] <LOW> [HIGH]

Arguments:
  <LOW>
          Number (or low bound) to use

          If no high bound is supplied, this will be the only number displayed.

  [HIGH]
          Optional high bound

Options:
  -f, --feminine
          Use the feminine declination

  -p, --prefix
          Prefix output with the numerical representation

  -r, --no-reform
          Use the pre-1990 orthographic reform writing

          By default, all numbers are separated by dashes. Pre-1990, only numbers smaller than 100 were separated by dashes, others words were separated by spaces.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
