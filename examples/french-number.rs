use clap::*;
use french_numbers::*;
use num_bigint::BigInt;

// List a single number, or numbers between two bounds given on the command line
fn main() {
    let matches = app_from_crate!()
        .arg(arg!(-f --feminine "Use the feminine declination"))
        .arg(arg!(-p --prefix "Prefix output with the numerical representation"))
        .arg(arg!(-r --"no-reform" "Use the pre-1990 orthographic reform writing"))
        .arg(arg!(<LOW> "Number (or low bound) to use"))
        .arg(arg!([HIGH] "Optional high bound"))
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
