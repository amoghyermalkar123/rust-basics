/*
Structs and enums are the building blocks for creating new types in your program’s domain
to take full advantage of Rust’s compile time type checking.
*/

// example 1

struct SomeType {
    name: String,
    age: u32,
}

fn create_literal() -> SomeType {
    let tp = SomeType {
        name: "amogh".to_string(),
        age: 21,
    };

    tp
}

pub fn doer() {
    let tp = create_literal();

    println!("{} {}", tp.name, tp.age);
}

// example 2

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn calculate_area() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    area(&mut rect1); // mutable borrow

    println!("{:?}", rect1);

    let mut s = String::from("amogh");

    strrr(&mut s);

    s.push_str("wow")
}

fn area(rectangle: &mut Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn strrr(s: &mut String) -> &mut String {
    s.push_str("ishan");
    s
}

// METHODS

/*
Methods are similar to functions: they’re declared with the fn keyword and their name,
they can have parameters and a return value, and they contain some code that is run when
they’re called from somewhere else. However, methods are different from functions in that
they’re defined within the context of a struct (or an enum or a trait object, and their
first parameter is always self, which represents the instance of the struct the method
is being called on.
*/

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    // method
    fn area(&self) -> u32 {
        /*
        In the signature for area, we use &self instead of rectangle: &Rectangle 
        because Rust knows the type of self is Rectangle due to this method’s being 
        inside the impl Rectangle context. Note that we still need to use the & before self, 
        just as we did in &Rectangle. Methods can take ownership of self, borrow self 
        immutably as we’ve done here, or borrow self mutably, just as they can any other 
        parameter.
        */
        self.width * self.height
    }

    // associated function
    fn just_print(sq: Square) {
        println!("{}, {}", sq.width, sq.height)
    }
}

pub fn start() {
    let square = Square {
        width:20,
        height:20,
    };

    // method
    println!("{}", square.area());

    // associated function
    Square::just_print(square);
}
