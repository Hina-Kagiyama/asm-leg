use std::{
    io::{Read, stdin},
    iter::repeat_n,
};

use asm_leg::grammar::ProgramParser;

const L: usize = 15;
fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    match ProgramParser::new().parse(&buf) {
        Ok(x) => x.into_iter().for_each(|x| {
            println!("{x}{} # {x:?}", {
                let l = x.to_string().len();
                repeat_n(' ', if l < L { L - l } else { 0 }).collect::<String>()
            })
        }),
        Err(e) => println!("{e}"),
    }
}
