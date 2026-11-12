// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn turn_left(direction: Direction) -> Direction {
//     match direction {
//         Direction::East => Direction::North,
//         Direction::West => Direction::South,
//         Direction::North => Direction::West,
//         Direction::South => Direction::East,
//     }
// }

// pub fn run() {
//     let current = Direction::East;
//     let next = turn_left(current);

//     println!("Current direction: {:?}", current);
//     println!("Next direction after turning left: {:?}", next);
// }

// q2. **E6.2** Define `enum Message { Quit, Move { x: i32, y: i32 }, Write(String), ChangeColor(u8, u8, u8) }`.
//  Write `fn describe(msg: &Message) -> String` that handles all four variants.

// enum Message {
//     quit,
//     Move { x: i32, y: i32 },
//     write(String),
//     ChangeColor(u8, u8, u8),
// }

// fn describe(msg: &Message) -> String {
//     match msg {
//         Message::quit => "Quit".to_string(),
//         Message::Move { x, y } => format!("Move to ({}, {})", x, y),
//         Message::write(s) => format!("Write: {}", s),
//         Message::ChangeColor(r, g, b) => format!("Change color to RGB({}, {}, {})", r, g, b),
//     }
// }

// pub fn run() {
//     let msg1 = Message::quit;
//     let msg2 = Message::Move { x: 10, y: 20 };
//     let msg3: Message = Message::write("Hello, world!".to_string());
//     let msg4 = Message::ChangeColor(255, 0, 0);

//     println!("{}", describe(&msg1));
//     println!("{}", describe(&msg2));
//     println!("{}", describe(&msg3));
//     println!("{}", describe(&msg4));
// }

// q3.

// fn divide(a: f64, b: f64) -> Option<f64> {
//     if b == 0.0 { None } else { Some(a / b) }
// }

// pub fn run() {
//     let result1 = divide(10.0, 2.0);
//     let result2 = divide(10.0, 0.0);

//     match result1 {
//         Some(value) => println!("Result of division: {}", value),
//         None => println!("Cannot divide by zero"),
//     }

//     match result2 {
//         Some(value) => println!("Result of division: {}", value),
//         None => println!("Cannot divide by zero"),
//     }
// }

// q4. **E6.5** Given `let v = vec![1, 2, 3];`,
// use `while let Some(x) = stack.pop()` on a `Vec` to pop and print all elements.

// pub fn run() {
//     let mut  v = vec![1, 2, 3];

//     while let Some(x) = v.pop() {
//         println!("{}", x);
//     }
// }
pub fn run() {
    let num: i32 = -5;

    match num {
        n if n < 0 => println!("negative"),
        0 => println!("zero"),
        1..=9 => println!("small"),
        _ => println!("large"),
    }
}
