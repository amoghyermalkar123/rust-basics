pub fn generic_function<T>(data: T) -> T {
    data
}

fn err_generic_function<T>(data: &mut T) -> T {
    // *data // why is this an error
    /*
        Whenever you dereference a pointer, you move ownership out of where the 
        pointer points to, into your local scope.
        
        is that so? i thought i was getting the value out of it and then copying 
        into the return variable. so potentially having 2 values of the same data.
        
        You can not copy any type automatically. So if you have a generic T that 
        should work for any type, it does not allow you to do that.

        You could require either that the type has to be copyable with the Copy trait, 
        or that it implements Clone which you would have to call explicitly

        but because you can not copy it, the only other thing it could possibly do 
        is take ownership, which would have to invalidate the source location, 
        hence the compiler time error

        if compiler had not thrown the error,
        dereferencing this variable could lead to: 2 mutable references to the same 
        data, hence memory-unsafe bugs !

        it's amazing how meticulous programming really is and how it's hidden by
        other high-level programming languages, this puts in perspective about
        what we are actually trying to learn is not so simple :) 
    */
}

// same generic type for x and y
struct Point<T> {
    x: T,
    y: T,
}

/*
Note that we have to declare T just after impl so we can use it to
specify that we’re implementing methods on the type Point<T>. By
declaring T as a generic type after impl, Rust can identify that the
type in the angle brackets in Point is a generic type rather than a
concrete type.
*/
impl<T> Point<T> {
    // here we implemented for Point<T> but we can be specific and
    // implement for other concrete types as well, i.e. impl Point<i32> ...
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }

}

pub fn point() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

// different generic types for x and y
struct PointTwo<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointTwo<T, U> {
    fn mixup<V, W>(self, other: PointTwo<V, W>) -> PointTwo<T, W> {
        PointTwo {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn point_two() {
    let _integer = PointTwo { x: 5, y: 10 };
    let _float = PointTwo { x: 1.0, y: 4.0 };
}
