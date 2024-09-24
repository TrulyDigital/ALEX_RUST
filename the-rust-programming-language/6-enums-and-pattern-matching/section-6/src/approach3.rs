#[derive(Debug)]
struct Move{
  x: i32,
  y: i32,
}

#[derive(Debug)]
enum MessageKind{
  Quit,
  Move(Move),
  Write(String),
  ChangeColor(i32, i32, i32),
}

/**
 * 
 * Destruct kind of MessageKind
 * 
 */
impl MessageKind{
  fn call(&self){
    match self {
      MessageKind::Move(m) => {
        println!("X: {}, Y: {}", m.x, m.y);
      }
      MessageKind::Write(text) => {
        println!("Write: {}", text);
      }
      MessageKind::Quit => {
        println!("Quite");
      }
      MessageKind::ChangeColor(r, g, b) => {
        println!("R: {}, G: {}, B: {}", r, g, b);
      }
    }
  }
}

fn main(){
  let message_write: MessageKind = MessageKind::Write(String::from("right"));

  let move_data: Move = Move {
    x: 5,
    y: 6,
  };
  let message_move: MessageKind = MessageKind::Move(move_data);

  println!("{:#?}", message_move.call());
  println!("{:#?}", message_write.call());
}