
#[derive(Debug)]
enum IpAddrKind{
  V4,
  V6,
}

fn main(){
  println!("Enums - Approach 1 !!");

  let four: IpAddrKind = IpAddrKind::V4;
  let six: IpAddrKind = IpAddrKind::V6;

  println!("Ip V4: {:#?}, Ip V6: {:#?}", four, six);
}