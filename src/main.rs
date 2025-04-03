use clap::Parser;
use unit_converter::convert;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// original number
    #[arg(required = true)]
    number: f64,
    /// Unit to convert
    #[arg(required = true)]
    unit: String,

    /// conversion output
    #[arg(long = "output", short = 'o', required = false, default_value = "all")]
    output_unit: String,
}

fn main() {
    let args = Args::parse();
    let output = convert(args.number, &args.unit.to_owned(), &args.output_unit);

    match output {
        Ok(out) => println!("{}", out),
        Err(err) => eprintln!("{}", err),
    }
}
