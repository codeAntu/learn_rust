# Rust Book Ch 7–12: Practice Exercises

How to use this file:
- Use one Cargo project per chapter, for example `cargo new ch08_exercises`.
- Try each exercise before reading the tip. Tips name the tool or concept, not the answer.
- Run `cargo clippy` and `cargo test` after each exercise.
- Commit to git after each passing exercise.

---

## Ch 7: Packages, Crates, and Modules

**Topics:** `cargo new --lib`, `mod`, `pub`, `use`, `super`, `crate::`, `pub(crate)`, re-exporting with `pub use`, splitting modules into files.

**E7.1** Create a library crate `shapes`. Inside it, make modules `circle` and `square`, each with a public `area` function. Call both from a binary in the same workspace.

**E7.2** Make a struct in a private module with one `pub` field and one private field. Show that you can't access the private field from outside, then add a `pub fn new` constructor to make it usable.

**E7.3** Move the `circle` module into its own file, `src/circle.rs`, and confirm it still compiles with only `mod circle;` in `lib.rs`.

**E7.4** Use `super::` to call a helper function defined in the parent module from a child module.

**E7.5** Use `pub use` to re-export a deeply nested function so users can call `shapes::area` instead of `shapes::circle::area`.

**Tips**
- Items are private by default. Mark them `pub` deliberately.
- `use` brings a path into scope. It doesn't load a file. Only `mod` does that.
- `crate::` starts from the root of your crate. `super::` goes one level up.
- `pub use` is how libraries hide internal structure from their users.

---

## Ch 8: Common Collections

**Topics:** `Vec<T>`, `String` and UTF-8, `HashMap<K, V>`, `entry` API, iterators over collections.

**E8.1** Create a `Vec<i32>`, push five numbers, and print the third element two ways: with indexing, and with `.get()`. Explain in a comment which one is safer and why.

**E8.2** Given `vec![3, 1, 4, 1, 5, 9, 2, 6]`, compute the sum, max, and average (as `f64`) using iterators.

**E8.3** Write `fn mode(nums: &[i32]) -> Option<i32>` that returns the most frequent value using a `HashMap`. Return `None` for an empty slice.

**E8.4** Given a sentence, count the occurrences of each word using `HashMap<String, usize>` and the `entry().or_insert(0)` pattern. Print the results sorted alphabetically.

**E8.5** Convert a string `"héllo"` into bytes and into `char`s. Print the lengths of each. Explain the difference in a comment.

**E8.6** Write `fn pig_latin(word: &str) -> String` (a simple version: move the first letter to the end and add "ay"). Build the result with `push_str`, not repeated `+`.

**Tips**
- `HashMap` lookups return `Option`. Use `get`, `entry`, or `if let`.
- `.entry(key).or_insert(0)` returns a mutable reference. Dereference it with `*` to update.
- `s.len()` counts bytes, not characters. Use `s.chars().count()` for characters.
- You can't index a `String` by integer. Use `.chars()` or byte slices.
- Iterator methods like `.sum()`, `.max()`, `.map()`, `.filter()` will replace most manual loops.

---

## Ch 9: Error Handling

**Topics:** `panic!`, `unwrap`/`expect`, `Result<T, E>`, `?` operator, propagating errors, `Box<dyn Error>`, when to panic vs return `Result`.

**E9.1** Write a function that opens a file path and returns `Result<String, std::io::Error>`. Call it from `main` with `match`, printing a friendly message on error.

**E9.2** Rewrite E9.1 to use `?` instead of `match`. Compare the two versions and note which one reads better.

**E9.3** Define a custom error enum `ConfigError { Missing(String), Invalid(String) }`. Write `fn parse_port(s: Option<&str>) -> Result<u16, ConfigError>` that returns the correct variant for each failure.

**E9.4** Implement `std::fmt::Display` for `ConfigError` so it prints a readable message.

**E9.5** Write a function returning `Result<u32, Box<dyn std::error::Error>>` that parses two numbers from strings and adds them. Use `?` for both parses.

**E9.6** Write a function that deliberately panics on an empty input. Then rewrite it to return `Err` instead. Explain in a comment when panicking is appropriate.

**Tips**
- `?` works only in functions that return `Result` or `Option`. Make `main` return `Result<(), Box<dyn Error>>` if you need it.
- `Box<dyn Error>` is fine for applications. Libraries should use custom error types.
- `expect("message")` is `unwrap` with a message. Use it when failure is a bug, not a user error.
- Implement `From` for your error type when you want `?` to convert automatically.

---

## Ch 10: Generic Types, Traits, and Lifetimes

**Topics:** generic functions and structs, trait definitions and implementations, trait bounds, `impl Trait`, `where` clauses, `#[derive]`, lifetime annotations, `'static`.

**E10.1** Write `fn largest<T: PartialOrd + Copy>(list: &[T]) -> T` and test it with `i32`, `char`, and `f64`.

**E10.2** Define a `Pair<T> { a: T, b: T }`. Implement a method `cmp_display` that prints the larger value, but only for `T: PartialOrd + Display`.

**E10.3** Define a trait `Summary { fn summarize(&self) -> String; }` with a default implementation returning `"(Read more...)"`. Implement it for two structs, one overriding the default and one using it.

**E10.4** Write `fn notify(item: &impl Summary)` and then rewrite it using a generic with a trait bound. Note the difference in a comment.

**E10.5** Write `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`. Then try to call it with one string declared in an inner scope and use the result outside that scope. Read the error and fix it.

**E10.6** Create a struct `Excerpt<'a> { part: &'a str }` holding a reference to a string slice. Write a method that returns the first word of it.

**Tips**
- Start without generics. Write it for `i32`, then generalize. This is how the compiler feedback becomes clearer.
- Trait bounds tell the compiler what operations your generic code can use. Add only what you need.
- Lifetimes don't make references live longer. They describe how long references relate to each other.
- Elision handles most lifetimes automatically. If you need to write `'a`, the compiler probably asked you to.
- `impl Trait` in argument position is sugar for a generic. Use it for simple cases.

---

## Ch 11: Writing Automated Tests

**Topics:** `#[test]`, `assert!`, `assert_eq!`, `assert_ne!`, `#[should_panic]`, `Result` in tests, `cargo test` options, unit vs integration tests.

**E11.1** Write `fn add(a: i32, b: i32) -> i32` and a test module with three tests: positive numbers, negative numbers, and zero.

**E11.2** Write a function that returns `Result<i32, String>` and test both the `Ok` and `Err` cases. Use `assert!(result.is_err())` for the error case.

**E11.3** Write a function that panics when given a negative number. Test it with `#[should_panic(expected = "...")]`.

**E11.4** Write a test that returns `Result<(), String>` and uses `?` inside it.

**E11.5** Put a function in `src/lib.rs` and write an integration test in `tests/integration_test.rs` that calls it from outside the crate.

**E11.6** Write a test that deliberately fails, then fix it. Read the output of `assert_eq!` carefully. It shows both values.

**E11.7** Run `cargo test -- --test-threads=1` and `cargo test some_name` to filter. Note the difference in output.

**Tips**
- `assert_eq!` requires `Debug` on both sides. Add `#[derive(Debug, PartialEq)]` to your types.
- `cargo test -- --nocapture` shows `println!` output from passing tests.
- Put unit tests in the same file as the code, inside `#[cfg(test)] mod tests`.
- Integration tests in `tests/` can only use your crate's public API.

---

## Ch 12: An I/O Project: Building a Command Line Program

**Topics:** reading command-line args (`std::env::args`), reading files, separating logic from `main`, `Config` struct, returning `Result` from `run`, environment variables, writing errors to stderr, test-driven development.

**E12.1** Read two command-line arguments, a query and a file path, and print them. Handle the case where they're missing with a clear error.

**E12.2** Read the file contents and print the whole file. Handle the case where the file doesn't exist.

**E12.3** Move the parsing logic into `Config::build(args: &[String]) -> Result<Config, &'static str>`. Keep `main` small.

**E12.4** Write `fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>` that returns lines containing the query. Write the test first, then the code.

**E12.5** Add a case-insensitive search that's enabled by an environment variable `IGNORE_CASE`. Write a test for it.

**E12.6** Send error messages to stderr with `eprintln!`, not `println!`. Verify with `cargo run > output.txt`.

**Tips**
- `std::env::args()` includes the program name as the first item. Skip it.
- Use `std::env::var("IGNORE_CASE").is_ok()` to check for the variable.
- Return `Box<dyn Error>` from `run`, and let `main` print the error and exit with code 1.
- `to_lowercase()` on both the query and the line gives a simple case-insensitive match.
- This chapter builds `minigrep`. Finish it: it's the first real program that ties everything together.

---

## Bonus: Mini Library System

Combine all six chapters in one workspace:
- A library crate with modules `book` and `member`, using `pub use` for a clean API (ch 7)
- A `HashMap<u32, Book>` for the catalog and a `Vec<Loan>` for active loans (ch 8)
- Functions returning `Result<_, LibraryError>` with a custom error enum (ch 9)
- A generic `fn find<T: Searchable>(items: &[T], query: &str) -> Vec<&T>` (ch 10)
- Unit tests for each function and one integration test for a full checkout-and-return flow (ch 11)
- A small CLI that reads commands from a file, using the `Config` pattern from ch 12

**Tip:** Build the types and the tests first. Once the tests pass, the CLI is just wiring.
