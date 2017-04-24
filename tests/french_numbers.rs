#![cfg(test)]

extern crate french_numbers;
use french_numbers::*;

extern crate num_bigint;
use num_bigint::BigInt;

extern crate num_traits;
use num_traits::*;

#[test]
fn test_french_number() {
    assert_eq!(french_number(&-17000), "moins dix-sept-mille");
    assert_eq!(french_number(&-17000000), "moins dix-sept-millions");
    assert_eq!(french_number(&17000000), "dix-sept-millions");
    assert_eq!(french_number(&220130202),
    "deux-cent-vingt-millions-cent-trente-mille-deux-cent-deux");
    let mut large = "un-billion-deux-cent-vingt-milliards-".to_owned();
    large.push_str("quatre-vingts-millions-trois-cent-quatre-vingt-mille-deux-cents");
    assert_eq!(french_number(&1220080380200u64), large);
}

#[test]
fn test_big_french_number() {
    let mut big = BigInt::one();
    for _ in 1..103 {
        big = big * BigInt::from_u64(10).unwrap();
    }
    big = big - BigInt::one();
    assert_eq!(french_number(&big).len(), 1527);
    assert_eq!(french_number(&-big).len(), 1533);
}

#[test]
fn test_too_big_french_number() {
    let mut big = BigInt::one();
    for _ in 1..103 {
        big = big * BigInt::from_u64(10).unwrap();
    }
    assert_eq!(french_number(&big), big.to_string());
    assert_eq!(french_number(&-big.clone()), (-big).to_string());
}

#[test]
fn test_feminine() {
    let options = Options {
        feminine: true,
        reformed: true,
    };
    assert_eq!(french_number_options(&1, &options), "une");
    assert_eq!(french_number_options(&21, &options), "vingt-et-une");
    assert_eq!(french_number_options(&71, &options), "soixante-et-onze");
    assert_eq!(french_number_options(&81, &options), "quatre-vingt-une");
    assert_eq!(french_number_options(&21001, &options),
    "vingt-et-un-mille-une");
    assert_eq!(french_number_options(&1021001, &options),
    "un-million-vingt-et-un-mille-une");
    assert_eq!(french_number_options(&101021001, &options),
    "cent-un-millions-vingt-et-un-mille-une");
}

#[test]
fn test_unreformed() {
    let options = Options {
        feminine: false,
        reformed: false,
    };
    assert_eq!(french_number_options(&1, &options), "un");
    assert_eq!(french_number_options(&21, &options), "vingt et un");
    assert_eq!(french_number_options(&71, &options), "soixante et onze");
    assert_eq!(french_number_options(&21001, &options),
    "vingt et un mille un");
    assert_eq!(french_number_options(&1021001, &options),
    "un million vingt et un mille un");
    assert_eq!(french_number_options(&1027001, &options),
    "un million vingt-sept mille un");
    assert_eq!(french_number_options(&101021037, &options),
    "cent un millions vingt et un mille trente-sept");
}
