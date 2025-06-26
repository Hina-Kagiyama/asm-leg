use std::io::{Read, stdin};

use asm_leg::grammar::ProgramParser;

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    match ProgramParser::new().parse(&buf) {
        Ok(x) => x.into_iter().for_each(|x| println!("{x}")),
        Err(e) => println!("{e}"),
    }
}
