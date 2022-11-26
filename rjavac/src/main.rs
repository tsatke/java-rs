use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
    if args.verbose {
        println!("Running in verbose mode");
    }
}
