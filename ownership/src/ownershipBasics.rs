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
 */

pub fn invalidation() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);
}
