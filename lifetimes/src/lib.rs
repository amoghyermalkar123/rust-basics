/*
Another kind of generic that we’ve already been using is called lifetimes. 
Rather than ensuring that a type has the behavior we want, lifetimes ensure 
that references are valid as long as we need them to be. Let’s look at how 
lifetimes do that.
*/

/*
every reference in rust has a lifetime
a simple thing to also remember is just as when we need to annotate 
types when multiple of them are possible, we annotate lifetimes of references 
when they could be related in a few different ways.
*/


/*
Rust requires us to annotate the relationships using generic lifetime 
parameters to ensure the actual references used at runtime will 
definitely be valid.
*/

/*
Lifetimes on function or method parameters are called input lifetimes, 
and lifetimes on return values are called output lifetimes.
*/

// preventing dangling references with lifetimes

pub fn dangles() {
    // what are dangling references?
    // they cause a program to reference data other than the data 
    // it’s intended to reference.
    // here is an example describing the problem
    let r;

    {
        let x = 5;
        r = &x;
    }

    // trying to use a reference after it has been dropped is what causes
    // dangling reference problem
    // see above the reference to x is dropped since it goes out of scope.
    println!("r : {}", r);
    
    // If Rust allowed this code to work, r would be referencing memory that was 
    // deallocated when x went out of scope, and anything we tried to do with r 
    // wouldn’t work correctly. So how does Rust determine that this code is 
    // invalid? It uses a borrow checker.
}


pub fn annotated_dangles() {
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+

    // Here, we’ve annotated the lifetime of r with 'a and the lifetime of x 
    // with 'b. As you can see, the inner 'b block is much smaller than the 
    // outer 'a lifetime block. At compile time, Rust compares the size of the 
    // two lifetimes and sees that r has a lifetime of 'a but that it refers to 
    // memory with a lifetime of 'b. The program is rejected because 'b is 
    // shorter than 'a: the subject of the reference doesn’t live as long as 
    // the reference.
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// The constraint we want to express in this signature is that all the 
// references in the parameters and the return value must have the same lifetime. 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
Ultimately, lifetime syntax is about connecting the lifetimes of various 
parameters and return values of functions. Once they’re connected, Rust 
has enough information to allow memory-safe operations and disallow operations 
that would create dangling pointers or otherwise violate memory safety.
*/

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main_two() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

/*
This struct has one field, part, that holds a string slice, which is a reference. 
As with generic data types, we declare the name of the generic lifetime parameter 
inside angle brackets after the name of the struct so we can use the lifetime 
parameter in the body of the struct definition. This annotation means an instance 
of ImportantExcerpt can’t outlive the reference it holds in its part field.

The main function here creates an instance of the ImportantExcerpt struct that 
holds a reference to the first sentence of the String owned by the variable novel. 
The data in novel exists before the ImportantExcerpt instance is created. 
In addition, novel doesn’t go out of scope until after the ImportantExcerpt 
goes out of scope, so the reference in the ImportantExcerpt instance is valid.
*/