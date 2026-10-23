pub fn run() {
    let mut x = 5;
    println!("this is a text {x}");

    const DONT_CHANGE_ME: u32 = 10;

    x = 10;

    println!("this is a text {x}");
    println!("this is a const value  {DONT_CHANGE_ME}");
}