// pub fn run() {
//     let name = String::from("Alice");

//     print(&name);

//     println!("The value of name is: {}", name);
// }

// fn print( str : &String) {
//     println!("The value of things is: {:#?}", str);
// }

// pub fn run() {
//     let my_sentence = String::from("Hello world!  I am Ananta karmakar");

//     let longest_word = get_longest_word(&my_sentence);
//     println!("The longest word in the sentence is: {}", longest_word);

//     //  clear the memory of my_sentence
//     // drop(my_sentence);

//     println!("The longest word in the sentence is: {}", longest_word);
// }

// fn get_longest_word(s: &str) -> &str {
//     let mut longest_word = "";

//     for word in s.split_whitespace() {
//         if word.len() > longest_word.len() {
//             longest_word = word;
//         }
//     }

//     longest_word
// }

// pub fn run() {
//     let mut my_string = String::from("Hello, world");

//     append_exclamation(&mut my_string);

//     println!("The value of my_string is: {}", my_string);
// }

// fn append_exclamation(s: &mut String) {
//     s.push('!');
// }

// pub fn run() {
//     let mut my_string = String::from("Hello, world");

//     let my_ref =  &my_string;

//     let my_ref2 = &my_string; // This will cause a compile-time error because we already have a mutable reference to my_string

//     println!("The value of my_string is: {}", my_ref);

//     println!("The value of my_string is: {}", my_ref2);
// }

// pub fn run() {
//     let mut my_string = String::from("my name is  Ananta karmakar");

//     let first_space_index = get_first_space(&my_string);

//     let first_word = &my_string[..first_space_index];

//     println!("The first word in the string is: {}", first_word);

//     my_string.clear(); // This will clear the string, making it empty

//     println!("The value of my_string is: {}", my_string);
//     println!("The first word in the string is: {}", first_word); // This will cause a compile-time error because first_word is a reference to my_string, which has been cleared
// }

// fn get_first_space(s: &str) -> usize {
//     for (i, &b) in s.as_bytes().iter().enumerate() {
//         if b == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

pub fn run() {
    let a = [1, 2, 3, 4, 5];

    let sum_of_slice = sum(&a[1..4]);

    println!("The sum of the slice is: {}", sum_of_slice);
}

fn sum(slice: &[i32]) -> i32 {
    let mut total = 0;
    for &item in slice {
        total += item;
    }
    total
}
