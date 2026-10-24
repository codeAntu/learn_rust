// pub fn run() {
//     let x = celsius_to_fahrenheit(37.0);

//     // we  cant  change that value

//     let mut x_mut = celsius_to_fahrenheit(37.0);

//     x_mut = celsius_to_fahrenheit(38.0);
//     // we can change that value

//     println!("The  value of x is: {}", x);
//     println!("The  value of x_mut is: {}", x_mut);
// }

// fn celsius_to_fahrenheit(celsius: f64) -> f64 {
//     (celsius * 9.0 / 5.0) + 32.0
// }

// pub fn run() {
//     let my_arr = [10, 20, 30, 40, 50];

//     for i in my_arr.iter() {
//         println!("The value is: {}", i);
//     }

//     for (index, value) in my_arr.iter().enumerate() {
//         println!("The value at index {} is: {}", index, value);
//     }
// }

pub fn run() {
    let mut number = String::new();

    println!("Please input a number: ");

    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = match number.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a valid number.");
            return;
        }
    };

    let result = fib(number as u32);

    println!("The {}th Fibonacci number is: {}", number, result);
}

fn fib(n: u32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }

    a
}
