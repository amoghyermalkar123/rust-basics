/*
A match is a primitive block. 
syntax - `match expression`

the syntax for match is in itself an expression. This allows for nested match blocks and 
complex pattern matching. There are also some built-in closure methods/functions that help
developers make complex nested match statements very easier to read and maintain.

match expression {

}

basically means match the expression with all it's listed arms, or match any of the mentioned 
arms and ignore the arms named so `_`. Matches in Rust are exhaustive in nature and the compiler
will let you know in some cases when you aren't handling all of them

A few varied examples of match below might make all the above explanations clear to you.
*/


// EXAMPLE 1
use std::fs::File;
use std::io::ErrorKind;

pub fn err_handling_two() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

// EXAMPLE 2
pub fn test_options(opt : Option<u32>) -> Option<u32> {
    match opt {
        None => None,
        Some(x) => Some(x + 2),
    }
}