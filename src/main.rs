pub mod diamond;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    letter: char,
}

fn main() {
    let args = Args::parse();
    arg_validator(args.letter);
    let result = diamond::build_diamond(args.letter);

    println!("{}", result);
}

fn arg_validator(letter: char) {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let result: usize = alphabet.matches(letter).count();

    if result == 0 {
        println!("Pattern accepted: [A-Z]");
        std::process::exit(0);
    }
}
