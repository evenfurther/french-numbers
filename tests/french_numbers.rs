#![cfg(test)]

use french_numbers::*;
use num_bigint::BigInt;
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
        big *= 10;
    }
    big -= 1;
    assert_eq!(french_number(&big).len(), 1527);
    assert_eq!(french_number(&-big).len(), 1533);
}

#[test]
fn test_too_big_french_number() {
    let mut big = BigInt::one();
    for _ in 1..103 {
        big *= 10;
    }
    assert_eq!(french_number(&big), big.to_string());
    assert_eq!(french_number(&-big.clone()), (-big).to_string());
}

#[test]
fn test_feminine() {
    assert_eq!(french_number_options(&1, &POST_REFORM_FEMININE), "une");
    assert_eq!(
        french_number_options(&21, &POST_REFORM_FEMININE),
        "vingt-et-une"
    );
    assert_eq!(
        french_number_options(&71, &POST_REFORM_FEMININE),
        "soixante-et-onze"
    );
    assert_eq!(
        french_number_options(&81, &POST_REFORM_FEMININE),
        "quatre-vingt-une"
    );
    assert_eq!(
        french_number_options(&21_001, &POST_REFORM_FEMININE),
        "vingt-et-un-mille-une"
    );
    assert_eq!(
        french_number_options(&1_021_001, &POST_REFORM_FEMININE),
        "un-million-vingt-et-un-mille-une"
    );
    assert_eq!(
        french_number_options(&101_021_001, &POST_REFORM_FEMININE),
        "cent-un-millions-vingt-et-un-mille-une"
    );
}

#[test]
fn test_unreformed() {
    assert_eq!(french_number_options(&1, &PRE_REFORM_MASCULINE), "un");
    assert_eq!(
        french_number_options(&21, &PRE_REFORM_MASCULINE),
        "vingt et un"
    );
    assert_eq!(
        french_number_options(&71, &PRE_REFORM_MASCULINE),
        "soixante et onze"
    );
    assert_eq!(
        french_number_options(&21_001, &PRE_REFORM_MASCULINE),
        "vingt et un mille un"
    );
    assert_eq!(
        french_number_options(&1_021_001, &PRE_REFORM_MASCULINE),
        "un million vingt et un mille un"
    );
    assert_eq!(
        french_number_options(&1_027_001, &PRE_REFORM_MASCULINE),
        "un million vingt-sept mille un"
    );
    assert_eq!(
        french_number_options(&101_021_037, &PRE_REFORM_MASCULINE),
        "cent un millions vingt et un mille trente-sept"
    );
}

#[test]
fn test_podcastfrancaisfacile() {
    // From http://www.podcastfrancaisfacile.com/
    assert_eq!(
        french_number_options(&3641, &PRE_REFORM_MASCULINE),
        "trois mille six cent quarante et un"
    );
    assert_eq!(
        french_number_options(&2984, &PRE_REFORM_MASCULINE),
        "deux mille neuf cent quatre-vingt-quatre"
    );
    assert_eq!(
        french_number_options(&7129, &PRE_REFORM_MASCULINE),
        "sept mille cent vingt-neuf"
    );
    assert_eq!(
        french_number_options(&1891, &PRE_REFORM_MASCULINE),
        "mille huit cent quatre-vingt-onze"
    );
    assert_eq!(
        french_number_options(&2820, &PRE_REFORM_MASCULINE),
        "deux mille huit cent vingt"
    );
    assert_eq!(
        french_number_options(&1734, &PRE_REFORM_MASCULINE),
        "mille sept cent trente-quatre"
    );
    assert_eq!(
        french_number_options(&1986, &PRE_REFORM_MASCULINE),
        "mille neuf cent quatre-vingt-six"
    );
    assert_eq!(
        french_number_options(&6012, &PRE_REFORM_MASCULINE),
        "six mille douze"
    );
    assert_eq!(
        french_number_options(&1930, &PRE_REFORM_MASCULINE),
        "mille neuf cent trente"
    );
    assert_eq!(
        french_number_options(&9021, &PRE_REFORM_MASCULINE),
        "neuf mille vingt et un"
    );
    assert_eq!(
        french_number_options(&5555, &PRE_REFORM_MASCULINE),
        "cinq mille cinq cent cinquante-cinq"
    );
    assert_eq!(
        french_number_options(&8080, &PRE_REFORM_MASCULINE),
        "huit mille quatre-vingts"
    );
    assert_eq!(
        french_number_options(&6728, &PRE_REFORM_MASCULINE),
        "six mille sept cent vingt-huit"
    );
    assert_eq!(
        french_number_options(&2773, &PRE_REFORM_MASCULINE),
        "deux mille sept cent soixante-treize"
    );
    assert_eq!(
        french_number_options(&1839, &PRE_REFORM_MASCULINE),
        "mille huit cent trente-neuf"
    );
    assert_eq!(
        french_number_options(&5391, &PRE_REFORM_MASCULINE),
        "cinq mille trois cent quatre-vingt-onze"
    );
    assert_eq!(
        french_number_options(&3100, &PRE_REFORM_MASCULINE),
        "trois mille cent"
    );
    assert_eq!(
        french_number_options(&1193, &PRE_REFORM_MASCULINE),
        "mille cent quatre-vingt-treize"
    );
    assert_eq!(
        french_number_options(&4722, &PRE_REFORM_MASCULINE),
        "quatre mille sept cent vingt-deux"
    );
    assert_eq!(
        french_number_options(&6382, &PRE_REFORM_MASCULINE),
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
        &POST_REFORM_MASCULINE,
    );
    check_reference(
        include_str!("files/nombres-francais-pre-reforme.txt"),
        &PRE_REFORM_MASCULINE,
    );
}

#[test]
fn test_educastream() {
    // From http://www.educastream.com/ecrire-grands-nombres-cm2
    assert_eq!(
        french_number_options(&1_236_458, &PRE_REFORM_MASCULINE),
        "un million deux cent trente-six mille quatre cent cinquante-huit"
    );
    // vingt-et-un is not correct here before the 1990 reform, fixed to vingt et un
    assert_eq!(
        french_number_options(&74_521_890, &PRE_REFORM_MASCULINE),
        "soixante-quatorze millions cinq cent vingt et un mille huit cent quatre-vingt-dix"
    );
    assert_eq!(
        french_number_options(&2_530_647_918u64, &PRE_REFORM_MASCULINE),
        "deux milliards cinq cent trente millions six cent quarante-sept mille neuf cent dix-huit"
    );
    assert_eq!(
        french_number_options(&1_234_569, &PRE_REFORM_MASCULINE),
        "un million deux cent trente-quatre mille cinq cent soixante-neuf"
    );
    assert_eq!(
        french_number_options(&20_263_400, &PRE_REFORM_MASCULINE),
        "vingt millions deux cent soixante-trois mille quatre cents"
    );
}

#[test]
fn test_termiumplus() {
    // From https://www.btb.termiumplus.gc.ca/tpv2guides/guides/clefsfp/index-fra.html?lang=fra&lettr=indx_catlog_m&page=9-nI6-pQZOTM.html
    assert_eq!(
        french_number_options(&1283, &PRE_REFORM_MASCULINE),
        "mille deux cent quatre-vingt-trois"
    );
    assert_eq!(
        french_number_options(&10_300_000_000_u64, &PRE_REFORM_MASCULINE),
        "dix milliards trois cents millions"
    );
    assert_eq!(
        french_number_options(&10_350_000_000_u64, &PRE_REFORM_MASCULINE),
        "dix milliards trois cent cinquante millions"
    );
    assert_eq!(
        french_number_options(&1283, &POST_REFORM_MASCULINE),
        "mille-deux-cent-quatre-vingt-trois"
    );
    assert_eq!(
        french_number_options(&10_300_000_000_u64, &POST_REFORM_MASCULINE),
        "dix-milliards-trois-cents-millions"
    );
    assert_eq!(
        french_number_options(&10_350_000_000_u64, &POST_REFORM_MASCULINE),
        "dix-milliards-trois-cent-cinquante-millions"
    );
}
