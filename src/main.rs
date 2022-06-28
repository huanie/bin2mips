use std::env;

use parse::Format;

mod parse;
mod types;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        let data = args[1..].join("").replace(' ', "");
        if data.len() != 32 {
            panic!("Type a 32bit binary number, spaces are allowed")
        }
        println!("{}", Format::new(&data));
    }
}
