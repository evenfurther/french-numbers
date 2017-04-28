extern crate getopts;
extern crate french_numbers;
extern crate num_bigint;
extern crate num_traits;

use french_numbers::*;
use num_bigint::BigInt;
use num_traits::*;
use std::process::exit;
use std::io::Write;

fn print_usage<W: Write>(program: &str, opts: &getopts::Options, output: &mut W) {
    let brief = format!("Usage: {} FILE [options] low [high]", program);
    write!(output, "{}", opts.usage(&brief)).unwrap();
}

// List a single number, or numbers between two bounds given on the command line
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();
    opts.optflag("f", "feminine", "use the feminine declination");
    opts.optflag("p", "prefix", "prefix with the numerical representation");
    opts.optflag("r",
                 "no-reform",
                 "use the pre-1990 orthographic reform writing");
    opts.optflag("h", "help", "this help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            writeln!(&mut std::io::stderr(), "{}", f.to_string()).unwrap();
            print_usage(&program, &opts, &mut std::io::stderr());
            exit(1);
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, &opts, &mut std::io::stdout());
        return;
    }
    let options = Options {
        feminine: matches.opt_present("f"),
        reformed: !matches.opt_present("r"),
    };
    if matches.free.len() != 1 && matches.free.len() != 2 {
        print_usage(&program, &opts, &mut std::io::stderr());
        exit(1);
    }
    let low = matches.free[0]
        .parse::<BigInt>()
        .expect("low bound must be an integer");
    let high = if matches.free.len() == 2 {
        matches.free[1]
            .parse::<BigInt>()
            .expect("high  bound must be an integer")
    } else {
        low.clone()
    };

    let use_prefix = matches.opt_present("p");
    let mut i = low;
    while i <= high {
        let repr = french_number_options(&i, &options);
        if use_prefix {
            println!("{} {}", i, repr);
        } else {
            println!("{}", repr);
        }
        i = i + BigInt::one();
    }
}
