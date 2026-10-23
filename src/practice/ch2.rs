// pub fn run() {

//   let  mut name  = String::new();

//   println!("Please input your name.");

//   std::io::stdin()
//       .read_line(&mut name)
//       .expect("Failed to read line");

//   println!("Hello, {}!", name);

// }

pub fn run() {
    let real_number : u32  = rand::random::<u32>() % 100 + 1;

    loop {
        let mut number: String = String::new();

        println!("Please input a number.");

        std::io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number = match number.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number.");
                continue;
            }
        };

        let ans = guess_feedback(real_number, number);

        println!("You guessed: {}, and  the  real number is {}", number, real_number  );
        println!("Feedback: {}", ans);
        if ans == "You win!" {
            break;
        }
    }
}

fn guess_feedback(secret: u32, guess: u32) -> &'static str {
    if guess < secret {
        "Too small!"
    } else if guess > secret {
        "Too big!"
    } else {
        "You win!"
    }
}
