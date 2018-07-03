#[macro_use]
extern crate clap;
extern crate french_numbers;
extern crate num_bigint;

use clap::*;
use french_numbers::*;
use num_bigint::BigInt;

// List a single number, or numbers between two bounds given on the command line
fn main() {
    let matches = App::new("french-number")
        .about("Print number(s) in French")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .args_from_usage(
            "
            -f, --feminine   'use the feminine declination'
            -p, --prefix     'prefix output with the numerical representation'
            -r, --no-reform  'use the pre-1990 orthographic reform writing'
            <LOW>            'number (or low bound) to use'
            [HIGH]           'optional high bound'",
        )
        .get_matches();
    let options = Options {
        feminine: matches.is_present("feminine"),
        reformed: !matches.is_present("no-reform"),
    };
    let low = matches
        .value_of("LOW")
        .unwrap()
        .parse::<BigInt>()
        .expect("low bound must be an integer");
    let high = matches.value_of("HIGH").map_or(low.clone(), |h| {
        h.parse::<BigInt>().expect("high  bound must be an integer")
    });

    let use_prefix = matches.is_present("prefix");
    let mut i = low;
    while i <= high {
        let repr = french_number_options(&i, &options);
        if use_prefix {
            println!("{} {}", i, repr);
        } else {
            println!("{}", repr);
        }
        i += 1;
    }
}
