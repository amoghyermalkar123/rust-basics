pub struct Data {
    name: String,
    age: i64,
}

pub struct Drawing {
    diagram: String,
}

pub trait Display {
    fn show(&self);
}

pub trait Clear {
    fn delete(&mut self);
}

impl Clear for Data {
    fn delete(&mut self) {
        self.name.clear();
        self.age = 0;
    }
} 

impl Clear for Drawing {
    fn delete(&mut self) {
        self.diagram.clear();
    }
}

impl Display for Data {
    fn show(&self) {
        println!("name is {} and age is {}", self.name, self.age);
    }
}

impl Display for Drawing {
    fn show(&self) {
        println!("here is the drawing \n {}", self.diagram);
    }
}

// this function signature follows a trait bound syntax
pub fn trait_bound_func<T: Display>(data : &T) {
    data.show()
}

// enforcing multiple trait object parameters to be the same type
pub fn trait_bound_same_type_fund<T: Display> (item1: &T, item2: &T){
    // item1 and item2 have the Display trait as well as are of the same type
    // here using trait bound syntax is useful
    item1.show();
    item2.show();
}

// allowing multiple trait object parameters to be of different type
pub fn trait_objects_func(item1 : &impl Display, item2: &impl Display){
    // item1 and item2 have the Display trait and can also be of different types
    // so in these scenarios we dont need to use the trait bound syntax 
    item1.show();
    item2.show();
}

// multiple trait bounds 
pub fn delete_data<T: Display + Clear>(data : &mut T){
    data.delete()
}

// syntax sugar for multiple trait bounds
pub fn delete_stuff<T>(data: &mut T)
where T: Display + Clear
{
    data.delete()
}

// conditional method implementation for generic struct types with trait bounds
struct SomeData<T> {
    x: T,
    y: T,
}

impl<T> SomeData<T> {
    fn new(x:T, y:T) -> Self {
        Self {x,y}
    }   
}

impl<T: Display + PartialOrd> SomeData<T> {
    fn print_things(&self){
        println!("hello")
    }
}

// conditional trait implementation based on trait bounds
pub trait Enlarge {
    fn enlarge_canvas(&self) -> String;
}

// the below block translates to
// give and implement the trait Enlarge for those types that have the trait Display
impl<T:Display> Enlarge for T {
    // this is how we can use trait bounds for conditional additional trait implementation
    fn enlarge_canvas(&self) -> String {
        String::from("canvas enlarged")
    }
}
