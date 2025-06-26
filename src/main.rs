use std::{
    io::{Read, stdin},
    // iter::repeat_n,
};

use asm_leg::grammar::ProgramParser;

// const L: usize = 15;
fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    match ProgramParser::new().parse(&buf) {
        Ok(x) => x.iter().for_each(|x| println!("{x}")),
        Err(e) => println!("{e}"),
    }
}
