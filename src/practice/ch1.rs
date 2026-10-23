pub fn run() {
    let name = String::from("Ananta Karmakar");

    println!("Hello, My name is {}!", name);

    for j in 1..=5 {
        for i in 1..=10 {
            println!("{} x {} = {}", j, i, i * j);
        }
    }
}
