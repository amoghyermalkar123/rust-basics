pub fn mainer() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// mutable references have one big restriction: you can have only one mutable reference
// to a particular piece of data in a particular scope.
// This code will fail:

/// fn main() {
///     let mut s = String::from("hello");
///
///     let r1 = &mut s;
///     let r2 = &mut s;
///
///     println!("{}, {}", r1, r2);
/// }

/*
The benefit of having this restriction is that Rust can prevent data races at compile time.
A data race is similar to a race condition and happens when these three behaviors occur:
    * Two or more pointers access the same data at the same time.
    * At least one of the pointers is being used to write to the data.
    * There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when
you’re trying to track them down at runtime; Rust prevents this problem from happening
because it won’t even compile code with data races!
*/

pub fn combine_types_of_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// fn main() {
//    let mut s = String::from("hello");

//    let r1 = &s; // no problem
//    let r2 = &s; // no problem
//    let r3 = &mut s; // BIG PROBLEM

//    println!("{}, {}, and {}", r1, r2, r3);
// }

// GIST -> YOU CAN ONLY HAVE ONE MUTABLE REFERENCE AT ONE TIME IN A GIVEN SCOPE
// YOU CAN HAVE ANY NUMBER OF IMMUTABLE REFERENCES BUT ONLY ONE MUTABLE REFERENCE WHEN
// THE USE OF IMMUTABLE REFERENCES IS DONE BEFORE USING THE MUTABLE REFERENCE.
