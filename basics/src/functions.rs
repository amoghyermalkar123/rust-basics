pub fn stats_and_exprs() {
    // Statements are instructions that perform some action and do not return a value. 
    // Expressions evaluate to a resulting value.
    // Function definitions are also statements.

    let y = {
        let x = 3;
        // notice that the last line does not have a semicolon
        // If you add a semicolon to the end of an expression, you turn it into a statement, 
        // which will then not return a value. 
        // Keep this in mind as you explore function return values and expressions 
        x + 1

        // statements don’t evaluate to a value, which is expressed by (), an empty tuple
    };

    println!("The value of y is: {}", y);
}

pub fn five() -> i32 {
    // Functions can return values to the code that calls them. 
    // We don’t name return values, but we do declare their type after an arrow (->). 
    // In Rust, the return value of the function is synonymous with the value of the 
    // final expression in the block of the body of a function. You can return early 
    // from a function by using the return keyword and specifying a value, but most 
    // functions return the last expression implicitly. 
    5
}
