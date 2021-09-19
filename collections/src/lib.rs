/*
Vectors allow you to store more than one value in a single data structure
that puts all the values next to each other in memory. Vectors can only store
values of the same type.
*/

pub fn do_stuff_with_vectors() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    println!("{}", vec.len());

    let vec_two = vec![1, 2, 3, 4];
    let vec_three = vec![String::from("amogh"), String::from("amogh")];
    println!("{:?}, {:?}", vec_two, vec_three);

    let get_vec = &vec_three[0];

    println!("{}", get_vec);

}
// When the vector gets dropped, all of its contents are also dropped,
// meaning those integers it holds will be cleaned up.

pub fn something() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

pub fn iterate_over_vecs() {
    let mut v = vec![1, 2, 3, 4, 5];

    for item in &mut v {
        // mutable borrow each item, change it, print it
        *item = 1;
        println!("{}", item)
    }
}
