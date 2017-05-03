//! This crate transforms a number into its French representation

#![deny(missing_docs)]

extern crate num_integer;
extern crate num_traits;

use num_integer::*;
use num_traits::*;
use std::fmt::Display;

/// Options for French number representation
#[derive(Debug)]
pub struct Options {
    /// Set to `true` to get a feminine declination (default `false`).
    /// This only affects numbers ending in 1.
    pub feminine: bool,
    /// Set to `false` to prevent hyphens from being inserted between
    /// literals greater than 100 (default `true`). This corresponds
    /// to the way of writing predating the 1990 orthographic reform.
    pub reformed: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            feminine: false,
            reformed: true,
        }
    }
}

fn literal_for(value: usize, options: &Options) -> Option<&'static str> {
    static SMALLS: [&'static str; 21] = ["zéro", "un", "deux", "trois", "quatre", "cinq", "six",
                                         "sept", "huit", "neuf", "dix", "onze", "douze", "treize",
                                         "quatorze", "quinze", "seize", "dix-sept", "dix-huit",
                                         "dix-neuf", "vingt"];
    if value == 1 && options.feminine {
        Some("une")
    } else if value <= 20 {
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
        Some(if options.reformed {
                 "soixante-et-onze"
             } else {
                 "soixante et onze"
             })
    } else if value == 80 {
        Some("quatre-vingts")
    } else if value == 81 {
        Some(if options.feminine {
                 "quatre-vingt-une"
             } else {
                 "quatre-vingt-un"
             })
    } else if value == 100 {
        Some("cent")
    } else if value == 1000 {
        Some("mille")
    } else {
        None
    }
}

fn add_unit_for(str: &mut String, prefix_count: usize, log1000: usize) -> bool {
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
        .map_or(false, |prefix| {
            str.push_str(prefix);
            if log1000 % 2 == 0 {
                str.push_str("illion");
            } else {
                str.push_str("illiard");
            }
            if prefix_count > 1 {
                str.push('s');
            }
            true
        })
}

fn unpluralize(str: &mut String) {
    if str.ends_with("ts") {
        let len = str.len();
        str.truncate(len - 1);
    }
}

fn complete(mut str: String, n: usize, prefix_under_100: bool, options: &Options) -> String {
    if n != 0 {
        unpluralize(&mut str);
    }
    if n == 1 {
        if prefix_under_100 && options.reformed {
            str.push_str("-et-un");
        } else if prefix_under_100 {
            str.push_str(" et un");
        } else if options.reformed {
            str.push_str("-un");
        } else {
            str.push_str(" un");
        }
        if options.feminine {
            str.push('e')
        }
    } else if n > 0 {
        str.push(if options.reformed || (prefix_under_100 && n < 100) {
                     '-'
                 } else {
                     ' '
                 });
        str.push_str(&basic(&n, options).unwrap());
    }
    str
}

fn basic<N: Integer + FromPrimitive + ToPrimitive>(n: &N, options: &Options) -> Option<String> {
    if let Some(n) = n.to_usize() {
        if let Some(literal) = literal_for(n, options) {
            return Some(literal.to_owned());
        } else if n < 60 {
            return Some(smaller_than_60(n, options));
        } else if n < 80 {
            return Some(base_onto(60, n, options));
        } else if n < 100 {
            return Some(base_onto(80, n, options));
        } else if n < 1000 {
            return Some(smaller_than_1000(n, options));
        } else if n < 2000 {
            return Some(smaller_than_2000(n, options));
        } else if n < 1000000 {
            return Some(smaller_than_1000000(n, options));
        }
    }
    over_1000000(n, options)
}

fn smaller_than_60(n: usize, options: &Options) -> String {
    let unit = n % 10;
    complete(basic(&(n - unit), &Default::default()).unwrap(),
             unit,
             true,
             options)
}

fn base_onto(b: usize, n: usize, options: &Options) -> String {
    complete(literal_for(b, options).unwrap().to_owned(),
             n - b,
             true,
             options)
}

fn smaller_than_1000(n: usize, options: &Options) -> String {
    let (hundredths, rest) = n.div_rem(&100);
    let result = if hundredths > 1 {
        let mut prefix = literal_for(hundredths, options).unwrap().to_owned();
        push_space_or_dash(&mut prefix, options);
        prefix.push_str("cents");
        prefix
    } else {
        "cent".to_owned()
    };
    complete(result, rest, false, options)
}

fn smaller_than_2000(n: usize, options: &Options) -> String {
    complete("mille".to_owned(), n - 1000, false, options)
}

fn push_space_or_dash(str: &mut String, options: &Options) {
    str.push(if options.reformed { '-' } else { ' ' });
}

fn smaller_than_1000000(n: usize, options: &Options) -> String {
    let (thousands, rest) = n.div_rem(&1000);
    let prefix = if thousands > 1 {
        let mut thousands = basic(&thousands,
                                  &Options {
                                       feminine: false,
                                       ..*options
                                   })
                .unwrap();
        unpluralize(&mut thousands);
        push_space_or_dash(&mut thousands, options);
        thousands.push_str("mille");
        thousands
    } else {
        "mille".to_owned()
    };
    complete(prefix, rest, false, options)
}

fn over_1000000<N: Integer + FromPrimitive + ToPrimitive>(n: &N,
                                                          options: &Options)
                                                          -> Option<String> {
    let thousand = N::from_u32(1000).unwrap();
    let (mut n, small) = n.div_rem(&N::from_u32(1000000).unwrap());
    let mut base = if small != N::zero() {
        basic(&small, options).unwrap()
    } else {
        String::new()
    };
    let mut log1000 = 0;
    while n != N::zero() {
        let (rest, prefix) = n.div_rem(&thousand);
        let prefix = prefix.to_usize().unwrap();
        if prefix > 0 {
            let mut str = basic(&prefix,
                                &Options {
                                     feminine: false,
                                     ..*options
                                 })
                    .unwrap();
            push_space_or_dash(&mut str, options);
            if !add_unit_for(&mut str, prefix, log1000) {
                return None;
            }
            if !base.is_empty() {
                push_space_or_dash(&mut str, options);
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
/// By default, the masculine declination is used, as well as the preferred
/// orthographic form introduced in the 1990 reform (use hyphens everywhere).
/// See `french_number_options` if you wish to change either of those options.
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
    french_number_options(n, &Default::default())
}

/// Compute the French language representation of the given number.
///
/// If the number is too large (greater than 10^103), then its numerical
/// representation is returned with a leading minus sign if needed.
///
/// # Example
///
/// ```
/// use french_numbers::*;
///
/// assert_eq!(french_number_options(&37251061,
///                                  &Options { feminine: false, reformed: true}),
///            "trente-sept-millions-deux-cent-cinquante-et-un-mille-soixante-et-un");
/// assert_eq!(french_number_options(&37251061,
///                                  &Options { feminine: true, reformed: true}),
///            "trente-sept-millions-deux-cent-cinquante-et-un-mille-soixante-et-une");
/// assert_eq!(french_number_options(&37251061,
///                                  &Options { feminine: true, reformed: false }),
///            "trente-sept millions deux cent cinquante et un mille soixante et une");
/// assert_eq!(french_number_options(&37251061,
///                                  &Options { feminine: false, reformed: false }),
///            "trente-sept millions deux cent cinquante et un mille soixante et un")
/// ```
pub fn french_number_options<N: Integer + FromPrimitive + ToPrimitive + Display>(n: &N,
                                                                                 options: &Options)
-> String{
    if n < &N::zero() {
        // Take the absolute value of n without consuming it. Since n is
        // negative, we know that we can build the -1 constant.
        let n = n.div_floor(&N::from_isize(-1).unwrap());
        if let Some(str) = basic(&n, options) {
            let mut result = "moins ".to_owned();
            result.push_str(&str);
            return result;
        }
    } else if let Some(result) = basic(n, options) {
        return result;
    }
    n.to_string()
}

#[cfg(test)]
mod tests {

    use ::*;

    #[test]
    fn test_literal_for() {
        assert_eq!(literal_for(30, &Default::default()), Some("trente"));
        assert_eq!(literal_for(31, &Default::default()), None);
    }

    #[test]
    fn test_add_unit_for() {
        let mut str = String::new();
        assert_eq!(add_unit_for(&mut str, 1, 0), true);
        assert_eq!(str, "million");
        str.clear();
        assert_eq!(add_unit_for(&mut str, 2, 0), true);
        assert_eq!(str, "millions");
        str.clear();
        assert_eq!(add_unit_for(&mut str, 1, 3), true);
        assert_eq!(str, "billiard");
        assert_eq!(add_unit_for(&mut str, 1, 97), false);
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
        assert_eq!(basic(&0, &Default::default()).unwrap(), "zéro");
        assert_eq!(basic(&21, &Default::default()).unwrap(), "vingt-et-un");
        assert_eq!(basic(&54, &Default::default()).unwrap(), "cinquante-quatre");
        assert_eq!(basic(&64, &Default::default()).unwrap(), "soixante-quatre");
        assert_eq!(basic(&71, &Default::default()).unwrap(), "soixante-et-onze");
        assert_eq!(basic(&72, &Default::default()).unwrap(), "soixante-douze");
        assert_eq!(basic(&80, &Default::default()).unwrap(), "quatre-vingts");
        assert_eq!(basic(&81, &Default::default()).unwrap(), "quatre-vingt-un");
        assert_eq!(basic(&91, &Default::default()).unwrap(),
                   "quatre-vingt-onze");
        assert_eq!(basic(&101, &Default::default()).unwrap(), "cent-un");
        assert_eq!(basic(&800, &Default::default()).unwrap(), "huit-cents");
        assert_eq!(basic(&803, &Default::default()).unwrap(), "huit-cent-trois");
        assert_eq!(basic(&872, &Default::default()).unwrap(),
                   "huit-cent-soixante-douze");
        assert_eq!(basic(&880, &Default::default()).unwrap(),
                   "huit-cent-quatre-vingts");
        assert_eq!(basic(&882, &Default::default()).unwrap(),
                   "huit-cent-quatre-vingt-deux");
        assert_eq!(basic(&1001, &Default::default()).unwrap(), "mille-un");
        assert_eq!(basic(&1882, &Default::default()).unwrap(),
                   "mille-huit-cent-quatre-vingt-deux");
        assert_eq!(basic(&2001, &Default::default()).unwrap(), "deux-mille-un");
        assert_eq!(basic(&300001, &Default::default()).unwrap(),
                   "trois-cent-mille-un");
        assert_eq!(basic(&180203, &Default::default()).unwrap(),
                   "cent-quatre-vingt-mille-deux-cent-trois");
        assert_eq!(basic(&180203, &Default::default()).unwrap(),
                   "cent-quatre-vingt-mille-deux-cent-trois");
        assert_eq!(basic(&17180203, &Default::default()).unwrap(),
                   "dix-sept-millions-cent-quatre-vingt-mille-deux-cent-trois");
    }
}
