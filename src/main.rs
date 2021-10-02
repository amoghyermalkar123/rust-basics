
fn main() {
    let ipv4 =  enums::IpAddr::V4(String::from("127.0.0.1"));

    let ipv6 =  enums::IpAddr::V6(String::from("2402:e280:3e10:49:4d8c:40d3:72ec:8e83"));

    enums::sort_addresses(ipv4);
    enums::sort_addresses(ipv6);
    enums::test_options();

    // error_handling::err_handling();

    let g = generics::generic_function(11212);
    println!("{}", g)
}
