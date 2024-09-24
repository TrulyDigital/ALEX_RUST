/**
 * 
 * TITLE: Structs
 * 
 * Struct and its implementation
 * 
 * Clone is a trait that it needs to be
 * implemented for clone structs.
 * 
 */

#[derive(Debug)]
struct User {
  active: bool,
  user_name: String,
  email: String,
  sign_in_count: u64
}

impl Clone for User {
  fn clone(&self) -> Self {
    Self {
      active: self.active.clone(),
      user_name: self.user_name.clone(),
      email: self.email.clone(),
      sign_in_count: self.sign_in_count,
    }
  }
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

/**
 * 
 * TITLE: Tuples
 * 
 */

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

/**
 * 
 * TITLE: Unit-Like Structs
 * 
 */
#[derive(Debug)]
struct AlwaysEqual;

/**
 * 
 * Functions
 * 
 */
fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}



/**
 * 
 * Entry point
 * 
 */
fn main() {
  let user1: User = User {
    active: true,
    user_name: String::from("alex.daniel.m"),
    email: String::from("alex.daniel.m@trulydigitl.tech"),
    sign_in_count: 1
  };

  let user2: User = user1.clone();
  let user2: User = User {
    email: String::from("another.email@example.com"),
    ..user2
  };

  println!("User 1 - struct: {:#?}", user1);
  println!("User 2 - struct: {:#?}", user2);

  let black: Color = Color(0, 0, 0);
  let origin: Point = Point(0, 0, 0);

  println!("Tuples");
  println!("Color {black:?}");
  println!("Point {origin:?}");
  println!("Color Black {}, {}, {}", black.0, black.1, black.2);
  println!("Point Origin {}, {}, {}", origin.0, origin.1, origin.2);

  let subject: AlwaysEqual = AlwaysEqual;
  println!("Unit-Like Struct");
  println!("{:#?}", subject);


  let rect1: Rectangle = Rectangle {
    width: 30,
    height: 50
  };

  let rect2: Rectangle = Rectangle {
    width: 10,
    height: 40
  };

  let rect3: Rectangle = Rectangle {
    width: 60,
    height: 45
  };

  println!("Struct Rectangle {rect1:?}");
  println!("Area {} from external function", area(&rect1));
  println!("Area {} from Struct implementation", rect1.area());

  println!("Hold Rentangles");
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

  println!("Change something");
}
