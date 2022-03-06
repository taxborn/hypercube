use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The file name
    #[clap(short, long)]
    file: String,

    /// The side length of the hypercube. This shouldn't change to adhere to
    /// the spec, but might be interesting to play around with.
    #[clap(short, long, default_value_t = 8)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    let mut file = File::open(args.file).expect("Open failed.");
    let mut source = String::new();

    file.read_to_string(&mut source).expect("Read to string failed.");

    println!(source);

}

