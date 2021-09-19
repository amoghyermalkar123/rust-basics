mod options;

enum RustTypes {
    Int, 
    String,
    SmallStr,
    Boolean,
    Generic,
    IntVariants
}

pub enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IPs {
    address: IpAddr,
    address_two : IpAddr2,
}

pub fn play_with_enums() {
    let int64 = RustTypes::IntVariants;

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let home2 = IpAddr2::V4(127, 0, 0, 1);

    let loopback2 = IpAddr2::V6(String::from("::1"));

    let addr = IPs {
        address : home,
        address_two: home2,
    };
}

pub fn sort_addresses(addrs: IpAddr) {
    match addrs {
        IpAddr::V4(addr) => {
            println!("got an IPv4 address! {}", addr)
        },
        IpAddr::V6(addr) => {
            println!("got an IPv6 address! {}", addr)
        },
    }
}

pub fn test_options() {
    let a = options::test_options(Some(3));

    let b = options::test_options(None);

    println!("{:?}, {:?}", a, b)
}
