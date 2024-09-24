#[derive(Debug)]
enum IpAddrKind{
  V4,
  V6,
}

struct IpAddr{
  kind: IpAddrKind,
  address: String,
}

fn main(){
  println!("Enums - Approach 2 !!");

  println!("");

  let home: IpAddr = IpAddr{
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback: IpAddr = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };

  println!("Home Ip Address:");
  println!("Kind {:#?}", home.kind);
  println!("Address {}", home.address);

  println!("");

  println!("LoopBack Ip Adress:");
  println!("Kind {:#?}", loopback.kind);
  println!("Address {}", loopback.address);


}