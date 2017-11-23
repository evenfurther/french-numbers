#![cfg(test)]

extern crate french_numbers;
use french_numbers::*;

extern crate num_bigint;
use num_bigint::BigInt;

extern crate num_traits;
use num_traits::*;

#[test]
fn test_french_number() {
    assert_eq!(french_number(&-17_000), "moins dix-sept-mille");
    assert_eq!(french_number(&-17_000_000), "moins dix-sept-millions");
    assert_eq!(french_number(&900), "neuf-cents");
    assert_eq!(french_number(&901), "neuf-cent-un");
    assert_eq!(french_number(&17_000_000), "dix-sept-millions");
    assert_eq!(
        french_number(&220_130_202),
        "deux-cent-vingt-millions-cent-trente-mille-deux-cent-deux"
    );
    let mut large = "un-billion-deux-cent-vingt-milliards-".to_owned();
    large.push_str("quatre-vingts-millions-trois-cent-quatre-vingt-mille-deux-cents");
    assert_eq!(french_number(&1_220_080_380_200u64), large);
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
    assert_eq!(
        french_number_options(&21_001, &options),
        "vingt-et-un-mille-une"
    );
    assert_eq!(
        french_number_options(&1_021_001, &options),
        "un-million-vingt-et-un-mille-une"
    );
    assert_eq!(
        french_number_options(&101_021_001, &options),
        "cent-un-millions-vingt-et-un-mille-une"
    );
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
    assert_eq!(
        french_number_options(&21_001, &options),
        "vingt et un mille un"
    );
    assert_eq!(
        french_number_options(&1_021_001, &options),
        "un million vingt et un mille un"
    );
    assert_eq!(
        french_number_options(&1_027_001, &options),
        "un million vingt-sept mille un"
    );
    assert_eq!(
        french_number_options(&101_021_037, &options),
        "cent un millions vingt et un mille trente-sept"
    );
}

#[test]
fn test_unreformed_web() {
    // From http://www.podcastfrancaisfacile.com/
    let options = Options {
        feminine: false,
        reformed: false,
    };
    assert_eq!(
        french_number_options(&3641, &options),
        "trois mille six cent quarante et un"
    );
    assert_eq!(
        french_number_options(&2984, &options),
        "deux mille neuf cent quatre-vingt-quatre"
    );
    assert_eq!(
        french_number_options(&7129, &options),
        "sept mille cent vingt-neuf"
    );
    assert_eq!(
        french_number_options(&1891, &options),
        "mille huit cent quatre-vingt-onze"
    );
    assert_eq!(
        french_number_options(&2820, &options),
        "deux mille huit cent vingt"
    );
    assert_eq!(
        french_number_options(&1734, &options),
        "mille sept cent trente-quatre"
    );
    assert_eq!(
        french_number_options(&1986, &options),
        "mille neuf cent quatre-vingt-six"
    );
    assert_eq!(french_number_options(&6012, &options), "six mille douze");
    assert_eq!(
        french_number_options(&1930, &options),
        "mille neuf cent trente"
    );
    assert_eq!(
        french_number_options(&9021, &options),
        "neuf mille vingt et un"
    );
    assert_eq!(
        french_number_options(&5555, &options),
        "cinq mille cinq cent cinquante-cinq"
    );
    assert_eq!(
        french_number_options(&8080, &options),
        "huit mille quatre-vingts"
    );
    assert_eq!(
        french_number_options(&6728, &options),
        "six mille sept cent vingt-huit"
    );
    assert_eq!(
        french_number_options(&2773, &options),
        "deux mille sept cent soixante-treize"
    );
    assert_eq!(
        french_number_options(&1839, &options),
        "mille huit cent trente-neuf"
    );
    assert_eq!(
        french_number_options(&5391, &options),
        "cinq mille trois cent quatre-vingt-onze"
    );
    assert_eq!(french_number_options(&3100, &options), "trois mille cent");
    assert_eq!(
        french_number_options(&1193, &options),
        "mille cent quatre-vingt-treize"
    );
    assert_eq!(
        french_number_options(&4722, &options),
        "quatre mille sept cent vingt-deux"
    );
    assert_eq!(
        french_number_options(&6382, &options),
        "six mille trois cent quatre-vingt-deux"
    );
}

fn check_reference(str: &str, options: &Options) {
    for line in str.lines() {
        let mut splitted = line.trim().splitn(2, ' ');
        let n = splitted
            .next()
            .expect("number not found")
            .parse::<u32>()
            .expect("unparsable number");
        let s = splitted.next().expect("french text not found");
        assert_eq!(french_number_options(&n, options), s);
    }
}

#[test]
fn test_reference() {
    check_reference(
        include_str!("files/nombres-francais.txt"),
        &Default::default(),
    );
    check_reference(
        include_str!("files/nombres-francais-pre-reforme.txt"),
        &Options {
            reformed: false,
            ..Default::default()
        },
    );
}
