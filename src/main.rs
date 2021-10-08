
fn main() {
    let ipv4 =  enums::IpAddr::V4(String::from("127.0.0.1"));

    let ipv6 =  enums::IpAddr::V6(String::from("2402:e280:3e10:49:4d8c:40d3:72ec:8e83"));

    enums::sort_addresses(ipv4);
    enums::sort_addresses(ipv6);
    enums::test_options();

    // error_handling::err_handling();

    generics::generic_function(11212);
    traits::use_summary();
    traits::default::use_default();

    let tweet = traits::Tweet{
        username: String::from("amogh"),
        content: String::from("this is content"),
        reply: false,
        retweet: false,
    };

    traits::take_any_type_that_has_summary_trait(&tweet);
}
