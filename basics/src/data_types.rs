/// Two types : Scalar and Compound
/// A Scalar type represents a single value
/// A Compound type represents a compound value

pub fn types() {
    let guess :u64 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{}, {} {} {} {} {} {} {}",guess, x, y, t, f, c ,z, heart_eyed_cat)
}

pub fn compound_types() {
    /*
    A tuple is a general way of grouping together a number of values with a variety of types 
    into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    We create a tuple by writing a comma-separated list of values inside parentheses. 
    Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
    */

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}