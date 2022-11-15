mod clippy;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(long, env = "FOO")]
    foo: String,

    #[arg(short, long)]
    max: Option<u8>,
}

fn main() {
    let args = Args::parse();

    if args.max == Some(0) {
        println!("zero");
    }

    println!("{:?}", args);
}
