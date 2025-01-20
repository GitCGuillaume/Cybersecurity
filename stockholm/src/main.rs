mod tools;
mod parse;
mod tests;
use tools::Flags;

fn main() {
    let args: std::iter::Skip<std::env::Args> = std::env::args().skip(1);
    let mut list: Flags;

    for i in args {

    }
}
