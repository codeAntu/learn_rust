// pub fn run() {
//     let mut my_vector: Vec<i32> = Vec::new();

//     my_vector.push(10);
//     my_vector.push(20);
//     my_vector.push(30);
//     my_vector.push(40);
//     my_vector.push(50);

//     let third_element = my_vector[2];

//     println!("The third element is: {}", third_element);

//     if let Some(third) = my_vector.get(2) {
//         println!("The third element is: {}", third);
//     } else {
//         println!("There is no third element.");
//     }
// }

// pub fn run() {
//     let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
//     let mode = mode(&v);

//     let sum: i32 = v.iter().sum();

//     let max = v.iter().max().unwrap_or(&0);

//     let avg = sum as f64 / v.len() as f64;

//     println!("Sum: {}", sum);
//     println!("Max: {}", max);
//     println!("Average: {}", avg);
// }

// fn mode(nums: &[i32]) -> Option<i32> {
//     let mut occurrences = std::collections::HashMap::new();

//     for &value in nums {
//         *occurrences.entry(value).or_insert(0) += 1;
//     }

//     occurrences
//         .into_iter()
//         .max_by_key(|&(_, count)| count)
//         .map(|(num, _)| num)
// }

// pub fn map_words_and_print(s: &str) -> Vec<&str> {
//     let mut h = std::collections::HashMap::new();

//     for word in s.split_whitespace() {
//         let count = h.entry(word).or_insert(0);

//         *count += 1
//     }

//     let mut word_counts: Vec<(&str, i32)> = h.into_iter().collect();

//     word_counts.sort_by(|a, b| a.0.cmp(b.0));

//     for (word, count) in &word_counts {
//         println!("{}: {}", word, count);
//     }

//     word_counts.into_iter().map(|(word, _)| word).collect()
// }

// E8.5 Convert a string "héllo" into bytes and into chars.
// Print the lengths of each. Explain the difference in a comment.

pub fn run() {
    let s = "héllo";

    let bytes = s.as_bytes();

    let chars = s.chars();

    println!("Length of bytes: {}", bytes.len());
    println!("Length of chars: {}", chars.clone().count());

    // The difference in length is due to the fact that the character 'é' is represented by two bytes in UTF-8 encoding,
    // while it is considered a single character. Therefore, the byte length is greater than the character count.
}

fn pig_latin(word: &str) -> String {
    let mut chars = word.chars();

    if let Some(first_char) = chars.next() {
        let mut result = String::new();

        result.push_str(chars.as_str());

        result.push(first_char);

        result.push_str("ay");

        result
    } else {
        String::new()
    }
}
