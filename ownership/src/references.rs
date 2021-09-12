/*
The ownership of a variable follows the same pattern every time: assigning a value to 
another variable moves it. When a variable that includes data on the heap goes out of 
scope, the value will be cleaned up by drop unless the data has been moved to be owned 
by another variable.

Taking ownership and then returning ownership with every function is a bit tedious. 
What if we want to let a function use a value but not take ownership? It’s quite annoying 
that anything we pass in also needs to be passed back if we want to use it again, in 
addition to any data resulting from the body of the function that we might want 
to return as well.

This is where references come into play.
*/
fn mainer() {
    let my_string = String::from("hello");

    let len = calculate_length(&my_string);

    println!("The length of '{}' is {}.", my_string, len);
}

fn calculate_length(s: &String) -> usize {
    /*
    When functions have references as parameters instead of the actual values, 
    we won’t need to return the values in order to give back ownership, because 
    we never had ownership.

    We call having references as function parameters borrowing. As in real life, 
    if a person owns something, you can borrow it from them. When you’re done, you 
    have to give it back.
    */

    /*
    The &String syntax lets us take a reference that refers to the value of my_string but does not own it. 
    Because it does not own it, 
    the value it points to will not be dropped when the reference goes out of scope.

    remember : values go out of scope when their owner does. 
                owner and their values go hand in hand.
    */
    s.len()
}
// In the above example the reference we take in is an immutable reference,
// meaning we cannot change the value

/*
The Rules of References
    *At any given time, you can have either one mutable reference or any number of immutable references.
    * References must always be valid.
*/
