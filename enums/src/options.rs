/*
Rust does not have nulls, but it does have an enum that can encode the concept of a 
value being present or absent. 

If we use None rather than Some, we need to tell Rust what type of Option<T> we have, 
because the compiler can’t infer the type that the Some variant will hold by looking 
only at a None value.

When we have a Some value, we know that a value is present and the value is held within 
the Some. When we have a None value, in some sense, it means the same thing as null: we 
don’t have a valid value.

you have to convert an Option<T> to a T before you can perform T operations with it. 
Generally, this helps catch one of the most common issues with null: assuming that 
something isn’t null when it actually is.
*/


pub fn test_options(opt : Option<u32>) -> Option<u32> {
    match opt {
        None => None,
        Some(x) => Some(x + 2),
    }
}

/*
Rust knows that we didn’t cover every possible case and even knows which pattern we forgot! 
Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code 
to be valid. Especially in the case of Option<T>, when Rust prevents us from forgetting to 
explicitly handle the None case, it protects us from assuming that we have a value when we 
might have null, thus making the billion-dollar mistake discussed earlier impossible.
*/
