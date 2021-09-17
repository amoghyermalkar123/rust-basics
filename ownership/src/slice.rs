// Another data type that does not have ownership is the slice. 
// Slices let you reference a contiguous sequence of elements 
// in a collection rather than the whole collection.

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // byte literal
            return &s[0..i];
        }
    }

    &s[..]
}


pub fn main_two() {
    let s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);
}
