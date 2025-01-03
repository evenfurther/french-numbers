//! This crate transforms a number into its French representation

#![deny(missing_docs)]
#![allow(clippy::non_ascii_literal)]
#![doc = include_str!("../README.md")]

use num_integer::Integer;
use num_traits::{CheckedMul, FromPrimitive, ToPrimitive};
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

/// Pre 1990 reform masculine variant.
pub static PRE_REFORM_MASCULINE: Options = Options {
    feminine: false,
    reformed: false,
};

/// Pre 1990 reform feminine variant.
pub static PRE_REFORM_FEMININE: Options = Options {
    feminine: true,
    reformed: false,
};

/// Post 1990 reform masculine variant. This is the default.
pub const POST_REFORM_MASCULINE: Options = Options {
    feminine: false,
    reformed: true,
};

/// Post 1990 reform feminine variant.
pub const POST_REFORM_FEMININE: Options = Options {
    feminine: true,
    reformed: true,
};

#[allow(clippy::derivable_impls)] // Clippy wrongly suggest that this Default trait can be derived
impl Default for Options {
    fn default() -> Self {
        POST_REFORM_MASCULINE
    }
}

impl Options {
    const fn masculinize(&self) -> Self {
        Self {
            feminine: false,
            ..*self
        }
    }
}

fn literal_for(value: usize, options: &Options) -> Option<String> {
    static SMALLS: [&str; 21] = [
        "zéro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf", "dix",
        "onze", "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit",
        "dix-neuf", "vingt",
    ];
    let literal = if value == 1 && options.feminine {
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
    };
    literal.map(String::from)
}

fn add_unit_for(str: &mut String, prefix_count: usize, log1000: usize) -> bool {
    static PREFIXES: [&str; 16] = [
        "m",
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
        "sexdéc",
    ];
    PREFIXES.get(log1000 / 2).is_some_and(|prefix| {
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
        str.truncate(str.len() - 1);
    }
}

fn complete(mut str: String, n: usize, prefix_under_100: bool, options: &Options) -> String {
    if n > 0 {
        unpluralize(&mut str);
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
                str.push('e');
            }
        } else {
            str.push(if options.reformed || (prefix_under_100 && n < 100) {
                '-'
            } else {
                ' '
            });
            str.push_str(&basic(&n, options, false));
        }
    }
    str
}

fn basic<N: Integer + FromPrimitive + ToPrimitive + Display>(
    n: &N,
    options: &Options,
    negative: bool,
) -> String {
    n.to_usize()
        .and_then(|n| {
            literal_for(n, options).or_else(|| match n {
                n if n < 60 => Some(smaller_than_60(n, options)),
                n if n < 80 => Some(base_onto(60, n, options)),
                n if n < 100 => Some(base_onto(80, n, options)),
                n if n < 1000 => Some(smaller_than_1000(n, options)),
                n if n < 2000 => Some(smaller_than_2000(n, options)),
                n if n < 1_000_000 => Some(smaller_than_1000000(n, options)),
                _ => None,
            })
        })
        .map_or_else(
            || over_1000000(n, options, negative),
            |s| add_minus(s, negative),
        )
}

fn smaller_than_60(n: usize, options: &Options) -> String {
    let unit = n % 10;
    complete(
        basic(&(n - unit), &Options::default(), false),
        unit,
        true,
        options,
    )
}

fn base_onto(b: usize, n: usize, options: &Options) -> String {
    complete(literal_for(b, options).unwrap(), n - b, true, options)
}

fn smaller_than_1000(n: usize, options: &Options) -> String {
    let (hundredths, rest) = n.div_rem(&100);
    let result = if hundredths > 1 {
        let mut prefix = literal_for(hundredths, options).unwrap();
        push_space_or_dash(&mut prefix, options);
        prefix.push_str("cents");
        prefix
    } else {
        String::from("cent")
    };
    complete(result, rest, false, options)
}

fn smaller_than_2000(n: usize, options: &Options) -> String {
    complete(String::from("mille"), n - 1000, false, options)
}

fn push_space_or_dash(str: &mut String, options: &Options) {
    str.push(if options.reformed { '-' } else { ' ' });
}

fn smaller_than_1000000(n: usize, options: &Options) -> String {
    let (thousands, rest) = n.div_rem(&1000);
    let prefix = if thousands > 1 {
        let mut thousands = basic(&thousands, &options.masculinize(), false);
        unpluralize(&mut thousands);
        push_space_or_dash(&mut thousands, options);
        thousands.push_str("mille");
        thousands
    } else {
        String::from("mille")
    };
    complete(prefix, rest, false, options)
}

fn over_1000000<N: Integer + FromPrimitive + ToPrimitive + Display>(
    n: &N,
    options: &Options,
    negative: bool,
) -> String {
    let thousand = N::from_u32(1000).unwrap();
    let (mut num, small) = n.div_rem(&N::from_u32(1_000_000).unwrap());
    let mut base = if small == N::zero() {
        None
    } else {
        Some(basic(&small, options, false))
    };
    let mut log1000 = 0;
    while num != N::zero() {
        let (rest, prefix) = num.div_rem(&thousand);
        let prefix = prefix.to_usize().unwrap();
        if prefix > 0 {
            let mut str = basic(&prefix, &options.masculinize(), false);
            push_space_or_dash(&mut str, options);
            if !add_unit_for(&mut str, prefix, log1000) {
                return add_minus_digits(n, negative);
            }
            if let Some(base) = base {
                push_space_or_dash(&mut str, options);
                str.push_str(&base);
            }
            base = Some(str);
        }
        log1000 += 1;
        num = rest;
    }
    base.map_or_else(|| add_minus_digits(n, negative), |s| add_minus(s, negative))
}

fn add_minus(s: String, negative: bool) -> String {
    if negative {
        format!("moins {s}")
    } else {
        s
    }
}

fn add_minus_digits<N>(n: N, negative: bool) -> String
where
    N: Display,
{
    if negative {
        format!("-{n}")
    } else {
        n.to_string()
    }
}

/// Compute the French language representation of the given number.
///
/// If the number is too large (greater than 10^103), then its numerical
/// representation is returned with a leading minus sign if needed.
///
/// Also, the smallest number of a bounded signed numerical type will be
/// returned as a numerical representation because the opposite value
/// cannot be computed. For example, `-128u8` will be shown as `-128`.
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
pub fn french_number<N: Integer + FromPrimitive + ToPrimitive + Display + CheckedMul>(
    n: &N,
) -> String {
    french_number_options(n, &Options::default())
}

/// Compute the French language representation of the given number with
/// the given formatting options.
///
/// If the number is too large (greater than 10^103), then its numerical
/// representation is returned with a leading minus sign if needed.
///
/// Also, the smallest number of a bounded signed numerical type will be
/// returned as a numerical representation because the opposite value
/// cannot be computed. For example, `-128u8` will be shown as `-128`.
///
/// # Example
///
/// ```
/// use french_numbers::*;
///
/// assert_eq!(french_number_options(&37251061, &POST_REFORM_MASCULINE),
///            "trente-sept-millions-deux-cent-cinquante-et-un-mille-soixante-et-un");
/// assert_eq!(french_number_options(&37251061, &POST_REFORM_FEMININE),
///            "trente-sept-millions-deux-cent-cinquante-et-un-mille-soixante-et-une");
/// assert_eq!(french_number_options(&37251061, &PRE_REFORM_FEMININE),
///            "trente-sept millions deux cent cinquante et un mille soixante et une");
/// assert_eq!(french_number_options(&37251061, &PRE_REFORM_MASCULINE),
///            "trente-sept millions deux cent cinquante et un mille soixante et un")
/// ```
pub fn french_number_options<N: Integer + FromPrimitive + ToPrimitive + Display + CheckedMul>(
    n: &N,
    options: &Options,
) -> String {
    if *n < N::zero() {
        // Take the absolute value of n without consuming it. Since n is negative, we know that
        // we can build the -1 constant. However, the positive value may not be properly
        // representable with this type.
        N::from_i8(-1)
            .and_then(|m1| m1.checked_mul(n))
            .map_or_else(|| n.to_string(), |n| basic(&n, options, true))
    } else {
        basic(n, options, false)
    }
}

#[cfg(test)]
mod tests {

    use crate::{add_unit_for, basic, literal_for, unpluralize};

    #[test]
    fn test_literal_for() {
        assert_eq!(
            literal_for(30, &Default::default()),
            Some(String::from("trente"))
        );
        assert_eq!(literal_for(31, &Default::default()), None);
    }

    #[test]
    fn test_add_unit_for() {
        let mut str = String::new();
        assert!(add_unit_for(&mut str, 1, 0));
        assert_eq!(str, "million");
        str.clear();
        assert!(add_unit_for(&mut str, 2, 0));
        assert_eq!(str, "millions");
        str.clear();
        assert!(add_unit_for(&mut str, 1, 3));
        assert_eq!(str, "billiard");
        assert!(!add_unit_for(&mut str, 1, 97));
    }

    #[test]
    fn test_unpluralize() {
        let mut s = String::from("quatre-cents");
        unpluralize(&mut s);
        assert_eq!(s, "quatre-cent");
        let mut s = String::from("cent");
        unpluralize(&mut s);
        assert_eq!(s, "cent");
    }

    #[test]
    fn test_basic() {
        assert_eq!(basic(&0, &Default::default(), false), "zéro");
        assert_eq!(basic(&21, &Default::default(), false), "vingt-et-un");
        assert_eq!(basic(&54, &Default::default(), false), "cinquante-quatre");
        assert_eq!(basic(&64, &Default::default(), false), "soixante-quatre");
        assert_eq!(basic(&71, &Default::default(), false), "soixante-et-onze");
        assert_eq!(basic(&72, &Default::default(), false), "soixante-douze");
        assert_eq!(basic(&80, &Default::default(), false), "quatre-vingts");
        assert_eq!(basic(&81, &Default::default(), false), "quatre-vingt-un");
        assert_eq!(basic(&91, &Default::default(), false), "quatre-vingt-onze");
        assert_eq!(basic(&101, &Default::default(), false), "cent-un");
        assert_eq!(basic(&800, &Default::default(), false), "huit-cents");
        assert_eq!(basic(&803, &Default::default(), false), "huit-cent-trois");
        assert_eq!(
            basic(&872, &Default::default(), false),
            "huit-cent-soixante-douze"
        );
        assert_eq!(
            basic(&880, &Default::default(), false),
            "huit-cent-quatre-vingts"
        );
        assert_eq!(
            basic(&882, &Default::default(), false),
            "huit-cent-quatre-vingt-deux"
        );
        assert_eq!(basic(&1001, &Default::default(), false), "mille-un");
        assert_eq!(
            basic(&1882, &Default::default(), false),
            "mille-huit-cent-quatre-vingt-deux"
        );
        assert_eq!(basic(&2001, &Default::default(), false), "deux-mille-un");
        assert_eq!(
            basic(&300_001, &Default::default(), false),
            "trois-cent-mille-un"
        );
        assert_eq!(
            basic(&180_203, &Default::default(), false),
            "cent-quatre-vingt-mille-deux-cent-trois"
        );
        assert_eq!(
            basic(&180_203, &Default::default(), false),
            "cent-quatre-vingt-mille-deux-cent-trois"
        );
        assert_eq!(
            basic(&17_180_203, &Default::default(), false),
            "dix-sept-millions-cent-quatre-vingt-mille-deux-cent-trois"
        );
    }
}
