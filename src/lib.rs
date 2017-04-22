//! This crate transforms a number into its French representation

#![warn(missing_docs)]

extern crate num_integer;
extern crate num_traits;

use num_integer::*;
use num_traits::*;
use std::fmt::Display;

fn literal_for(value: usize) -> Option<&'static str> {
    static SMALLS: [&'static str; 21] = ["zéro", "un", "deux", "trois", "quatre", "cinq", "six",
                                         "sept", "huit", "neuf", "dix", "onze", "douze", "treize",
                                         "quatorze", "quinze", "seize", "dix-sept", "dix-huit",
                                         "dix-neuf", "vingt"];
    if value <= 20 {
        Some(SMALLS[value])
    } else if value == 30 {
        Some("trente")
    } else if value == 40 {
        Some("quarante")
    } else if value == 50 {
        Some("cinquante")
    } else if value == 60 {
        Some("soixante")
    } else if value == 71 {
        Some("soixante-et-onze")
    } else if value == 80 {
        Some("quatre-vingts")
    } else if value == 81 {
        Some("quatre-vingt-un")
    } else if value == 100 {
        Some("cent")
    } else if value == 1000 {
        Some("mille")
    } else {
        None
    }
}

fn unit_for(log1000: usize) -> Option<String> {
    static PREFIXES: [&'static str; 16] = ["m",
                                           "b",
                                           "tr",
                                           "quadr",
                                           "quint",
                                           "sext",
                                           "sept",
                                           "oct",
                                           "non",
                                           "déc",
                                           "unodéc",
                                           "duodéc",
                                           "trédéc",
                                           "quattuordéc",
                                           "quindéc",
                                           "sexdéc"];
    PREFIXES
        .get(log1000 / 2)
        .map(|unit| {
            let mut unit = (*unit).to_owned();
            if log1000 % 2 == 0 {
                unit.push_str("illion");
            } else {
                unit.push_str("illiard");
            }
            unit
        })
}

fn unpluralize(str: &mut String) {
    if str.ends_with("ts") {
        let len = str.len();
        str.truncate(len - 1);
    }
}

fn complete(mut str: String, n: usize, et: bool) -> String {
    if n == 1 {
        str.push_str(if et { "-et-un" } else { "-un" });
    } else if n > 0 {
        unpluralize(&mut str);
        str.push('-');
        str.push_str(&basic(&n).unwrap());
    }
    str
}

fn basic<N: Integer + FromPrimitive + ToPrimitive>(n: &N) -> Option<String> {
    if let Some(n) = n.to_usize() {
        if let Some(literal) = literal_for(n) {
            return Some(literal.to_owned());
        } else if n < 60 {
            return Some(smaller_than_60(n));
        } else if n < 80 {
            return Some(base_onto(60, n));
        } else if n < 100 {
            return Some(base_onto(80, n));
        } else if n < 1000 {
            return Some(smaller_than_1000(n));
        } else if n < 2000 {
            return Some(smaller_than_2000(n));
        } else if n < 1000000 {
            return Some(smaller_than_1000000(n));
        }
    }
    over_1000000(n)
}

fn smaller_than_60(n: usize) -> String {
    let unit = n % 10;
    complete(basic(&(n - unit)).unwrap(), unit, true)
}

fn base_onto(b: usize, n: usize) -> String {
    complete(literal_for(b).unwrap().to_owned(), n - b, true)
}

fn smaller_than_1000(n: usize) -> String {
    let (hundredths, rest) = n.div_rem(&100);
    let result = if hundredths > 1 {
        let mut prefix = literal_for(hundredths).unwrap().to_owned();
        prefix.push_str("-cents");
        prefix
    } else {
        "cent".to_owned()
    };
    complete(result, rest, false)
}

fn smaller_than_2000(n: usize) -> String {
    complete("mille".to_owned(), n - 1000, false)
}

fn smaller_than_1000000(n: usize) -> String {
    let (thousands, rest) = n.div_rem(&1000);
    let prefix = if thousands > 1 {
        let mut thousands = basic(&thousands).unwrap();
        unpluralize(&mut thousands);
        thousands.push_str("-mille");
        thousands
    } else {
        "mille".to_owned()
    };
    complete(prefix, rest, false)
}

fn over_1000000<N: Integer + FromPrimitive + ToPrimitive>(n: &N) -> Option<String> {
    let thousand = N::from_u32(1000).unwrap();
    let (mut n, small) = n.div_rem(&N::from_u32(1000000).unwrap());
    let mut base = if small != N::zero() {
        basic(&small).unwrap()
    } else {
        String::new()
    };
    let mut log1000 = 0;
    while n != N::zero() {
        let (rest, prefix) = n.div_rem(&thousand);
        let prefix = prefix.to_usize().unwrap();
        if prefix > 0 {
            let mut str = basic(&prefix).unwrap();
            str.push('-');
            if let Some(unit) = unit_for(log1000) {
                str.push_str(&unit);
            } else {
                return None;
            }
            if prefix > 1 && !str.ends_with('s') {
                str.push('s');
            }
            if !base.is_empty() {
                str.push('-');
                str.push_str(&base);
            }
            base = str;
        }
        log1000 += 1;
        n = rest;
    }
    Some(base)
}

/// Compute the French language representation of the given number.
///
/// If the number is too large (greater than 10^103), then its numerical
/// representation is returned with a leading minus sign if needed.
///
/// # Example
///
/// ```
/// use french_numbers::french_number;
///
/// assert_eq!(french_number(&71), "soixante-et-onze");
/// assert_eq!(french_number(&1001), "mille-un");
/// assert_eq!(french_number(&-200001), "moins deux-cent-mille-un");
/// assert_eq!(french_number(&-200000001), "moins deux-cents-millions-un");
/// assert_eq!(french_number(&-204000001), "moins deux-cent-quatre-millions-un");
/// ```
pub fn french_number<N: Integer + FromPrimitive + ToPrimitive + Display>(n: &N) -> String {
    if n < &N::zero() {
        // Take the absolute value of n without consuming it. Since n is
        // negative, we know that we can build the -1 constant.
        let n = n.div_floor(&N::from_isize(-1).unwrap());
        if let Some(str) = basic(&n) {
            let mut result = "moins ".to_owned();
            result.push_str(&str);
            return result;
        }
    } else if let Some(result) = basic(n) {
        return result;
    }
    n.to_string()
}

#[cfg(test)]
mod tests {

    extern crate num_bigint;
    use self::num_traits::FromPrimitive;
    use self::num_bigint::BigInt;

    use ::*;

    #[test]
    fn test_literal_for() {
        assert_eq!(literal_for(30), Some("trente"));
        assert_eq!(literal_for(31), None);
    }

    #[test]
    fn test_unit_for() {
        assert_eq!(unit_for(0).unwrap(), "million");
        assert_eq!(unit_for(3).unwrap(), "billiard");
        assert_eq!(unit_for(97), None);
    }

    #[test]
    fn test_unpluralize() {
        let mut s = "quatre-cents".to_owned();
        unpluralize(&mut s);
        assert_eq!(s, "quatre-cent");
        let mut s = "cent".to_owned();
        unpluralize(&mut s);
        assert_eq!(s, "cent");
    }

    #[test]
    fn test_basic() {
        assert_eq!(basic(&0).unwrap(), "zéro");
        assert_eq!(basic(&21).unwrap(), "vingt-et-un");
        assert_eq!(basic(&54).unwrap(), "cinquante-quatre");
        assert_eq!(basic(&64).unwrap(), "soixante-quatre");
        assert_eq!(basic(&71).unwrap(), "soixante-et-onze");
        assert_eq!(basic(&72).unwrap(), "soixante-douze");
        assert_eq!(basic(&80).unwrap(), "quatre-vingts");
        assert_eq!(basic(&81).unwrap(), "quatre-vingt-un");
        assert_eq!(basic(&91).unwrap(), "quatre-vingt-onze");
        assert_eq!(basic(&101).unwrap(), "cent-un");
        assert_eq!(basic(&800).unwrap(), "huit-cents");
        assert_eq!(basic(&803).unwrap(), "huit-cent-trois");
        assert_eq!(basic(&872).unwrap(), "huit-cent-soixante-douze");
        assert_eq!(basic(&880).unwrap(), "huit-cent-quatre-vingts");
        assert_eq!(basic(&882).unwrap(), "huit-cent-quatre-vingt-deux");
        assert_eq!(basic(&1001).unwrap(), "mille-un");
        assert_eq!(basic(&1882).unwrap(), "mille-huit-cent-quatre-vingt-deux");
        assert_eq!(basic(&2001).unwrap(), "deux-mille-un");
        assert_eq!(basic(&300001).unwrap(), "trois-cent-mille-un");
        assert_eq!(basic(&180203).unwrap(),
                   "cent-quatre-vingt-mille-deux-cent-trois");
        assert_eq!(basic(&180203).unwrap(),
                   "cent-quatre-vingt-mille-deux-cent-trois");
        assert_eq!(basic(&17180203).unwrap(),
                   "dix-sept-millions-cent-quatre-vingt-mille-deux-cent-trois");
    }

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
}
