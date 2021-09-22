enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));

}
