/**
the context:
    we can’t put a blob of memory into the binary for each piece of text whose size is 
    unknown at compile time and whose size might change while running the program.
    in order to support a mutable, growable piece of text, we need to allocate an amount of memory 
    on the heap, unknown at compile time, to hold the contents. This means:
    The memory must be requested from the memory allocator at runtime.
    We need a way of returning this memory to the allocator when we’re done with the piece of memory
the problem:
    In languages with a garbage collector (GC), the GC keeps track and cleans up memory 
    that isn’t being used anymore, and we don’t need to think about it. Without a GC, 
    it’s our responsibility to identify when memory is no longer being used and call code 
    to explicitly return it, just as we did to request it. Doing this correctly has historically 
    been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, 
    we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly 
    one allocate with exactly one free.
the solution:
    Rust takes a different path: the memory is automatically returned once the variable that 
    owns it goes out of scope.
    NOTE :: none of the ownership features slow down your program while it’s running.
 */

pub fn invalidation() {

    // listing 1
    let s1 = String::from("hello");
    let s2 = s1;
    // s1 is now invalidated and is moved into s2
    // try using s1 to see error message

    println!("{}, world!", s2);

    // listing 2
    let s3 = "amogh";
    let s4 = s3; 

    println!("{},{}",s3,s4)

    // why is it that we can use s3 from listing 2 in print but in
    // listing 1 we can do the same with s1? 
    // as it is evident, String::from() allocates on heap and returns a reference.
    // and in listing 2 we create a string that resides completely on stack
    // so rust does not move s3 into s4 but rather just copies it
}

// continued ..

pub fn copying() {
    /*
    If we do want to deeply copy the heap data of the String, 
    not just the stack data, we can use a common method called clone.
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // NOW ...

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    /*
    But this code seems to contradict what we just learned: we don’t have a call to clone, 
    but x is still valid and wasn’t moved into y.
    The reason is that types such as integers that have a known size at compile time are 
    stored entirely on the stack, so copies of the actual values are quick to make. 
    That means there’s no reason we would want to prevent x from being valid after we create 
    the variable y. In other words, there’s no difference between deep and shallow copying here, 
    so calling clone wouldn’t do anything different from the usual shallow copying and we can 
    leave it out.
    
    Rust has a special annotation called the Copy trait that we can place on types like 
    integers that are stored on the stack (we’ll talk more about traits in Chapter 10). 
    If a type implements the Copy trait, an older variable is still usable after assignment. 
    Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, 
    has implemented the Drop trait. If the type needs something special to happen when the 
    value goes out of scope and we add the Copy annotation to that type, we’ll get a 
    compile-time error. 
    */
}

pub fn ownership() {
    let s = String::from("amogh yermalkar");
    takes_ownership(s);

    let y = 1;

    makes_copy(y);
}

pub fn takes_ownership(some_string : String) {
    println!("{}", some_string)
}

pub fn makes_copy(number : u8) {
    println!("{}", number)
}

fn give_and_take_main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
