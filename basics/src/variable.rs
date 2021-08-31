const I_AM_A_CONST : u32 = 100_000;

pub fn variables() {
    // this is an immutable variable
    let x = 5;
    println!("{}", x);

    // hence we cannot change it's value i.e. re-assign it
    // x = 6;
    // println!("The value of x is: {}", x);
}

fn playing_with_mutability() {
    let x = "amogh";
    // the below assignment is not allowed in rust
    // since 'x' is a string type whose values are not known at compile time
    // let y = *x;
}

pub fn constants() {
    println!("{}", I_AM_A_CONST)
}


// SHADOWING

pub fn shadowing() {
    /*
    Shadowing is different from marking a variable as mut, because we’ll get a compile-time error 
    if we accidentally try to reassign to this variable without using the let keyword. 
    By using let, we can perform a few transformations on a value but have the variable be 
    immutable after those transformations have been completed.
    The other difference between mut and shadowing is that because we’re effectively creating 
    a new variable when we use the let keyword again, we can change 
    the type of the value but reuse the same name.
    */
    let x = 5;

    let x = x + 5;

    println!("{}", x)
}