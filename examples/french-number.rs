use clap::Parser;
use french_numbers::*;
use num_bigint::BigInt;

#[derive(Parser)]
#[clap(version, author, about)]
struct Args {
    #[clap(short, long)]
    /// Use the feminine declination
    feminine: bool,
    #[clap(short, long)]
    /// Prefix output with the numerical representation
    prefix: bool,
    #[clap(short('r'), long)]
    /// Use the pre-1990 orthographic reform writing
    ///
    /// By default, all numbers are separated by dashes. Pre-1990, only
    /// numbers smaller than 100 were separated by dashes, others words
    /// were separated by spaces.
    no_reform: bool,
    /// Number (or low bound) to use
    ///
    /// If no high bound is supplied, this will be the only number displayed.
    low: BigInt,
    /// Optional high bound
    high: Option<BigInt>,
}

// List a single number, or numbers between two bounds given on the command line
fn main() {
    let args = Args::parse();
    let options = Options {
        feminine: args.feminine,
        reformed: !args.no_reform,
    };
    let high = args.high.unwrap_or_else(|| args.low.clone());
    let mut i = args.low;
    while i <= high {
        let repr = french_number_options(&i, &options);
        if args.prefix {
            println!("{} {}", i, repr);
        } else {
            println!("{}", repr);
        }
        i += 1;
    }
}
