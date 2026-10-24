# Rust Book Ch 1–6: Practice Exercises

How to use this file:

- Make one Cargo project per chapter: `cargo new ch03_exercises`, and so on.
- Try each exercise before reading the tip. Tips name the tool or concept, not the answer.
- Run `cargo clippy` after each exercise. It teaches idiomatic Rust faster than anything else.
- Commit to git after each passing exercise.

---

## Ch 1: Getting Started

**Topics:** `rustup`, `cargo new/build/run/check`, `println!`, project layout.

**E1.1** Create a project called `greeter`. Print `Hello, <your name>!` using a variable for the name.

**E1.2** Run `cargo check` and `cargo build` on a project with a deliberate type error. Then fix it. Write down the difference between the two commands in one sentence.

**E1.3** Print a simple multiplication table from 1 to 5 using only `println!`. Use `{}` placeholders, not string concatenation.

**Tips**

- `cargo check` only type-checks and is faster. Use it while iterating.
- `println!` is a macro (note the `!`). Format arguments go in `{}`.
- `{:?}` prints debug output. You'll use it a lot.

---

## Ch 2: Guessing Game

**Topics:** `std::io::stdin`, `Result`, `expect`, `match` on `Ordering`, `loop`, `rand` crate.

**E2.1** Read a line from stdin and print it back in uppercase. Handle the `Result` with `expect`.

**E2.2** Read a number from stdin. If parsing fails, print `"Not a number"` and ask again, without crashing. Use `match` on the `Result` from `parse()`, not `expect`.

**E2.3** Write a function `guess_feedback(secret: u32, guess: u32) -> &'static str` that returns `"Too small"`, `"Too big"`, or `"Correct"`. Test it with `#[test]`.

**E2.4** Wrap E2.3 in a game loop that stops on a correct guess and counts attempts.

**Tips**

- `cmp` returns an `Ordering`. Match on `Ordering::Less`, `Greater`, `Equal`.
- `trim()` removes the newline from `read_line`. Forgetting it is a classic bug.
- `parse::<u32>()` returns a `Result`. The turbofish `::<>` tells it the target type.
- For random numbers, add `rand` with `cargo add rand`, then `rand::thread_rng().gen_range(1..=100)`.

---

## Ch 3: Common Programming Concepts

**Topics:** `let` vs `mut`, shadowing, scalar and compound types, tuples, arrays, functions, `if`, `loop`, `while`, `for`, ranges.

**E3.1** Declare an immutable `x = 5`, then try to reassign it. Read the compiler error. Fix it by shadowing, then by `mut`. Explain the difference in a comment.

**E3.2** Write `fn celsius_to_fahrenheit(c: f64) -> f64`. Use it in `main` for three values.

**E3.3** Given the array `[10, 20, 30, 40, 50]`, print each element with its index using `for` and `.iter().enumerate()`.

**E3.4** Write `fn fib(n: u32) -> u64` that returns the nth Fibonacci number, using a `loop` or `for`, not recursion.

**E3.5** Use a labeled `break` to find the first pair `(i, j)` with `i` and `j` from 1 to 10 where `i * j == 42`. Print it.

**E3.	6** Destructure a tuple `(i32, f64, char)` into three variables and print them.

**Tips**

- `for x in 0..n` excludes `n`. `0..=n` includes it.
- Shadowing lets you change a type: `let spaces = "   "; let spaces = spaces.len();`.
- Integer overflow panics in debug builds. Try `255u8 + 1` to see it.
- Use `u64` for Fibonacci. It overflows at `u32` earlier than you'd expect.
- `loop` can return a value: `let x = loop { break 5; };`.

---

## Ch 4: Understanding Ownership

**Topics:** stack vs heap, move semantics, `clone`, `Copy`, references `&`, mutable references `&mut`, borrowing rules, slices, `String` vs `&str`.

**E4.1** Create a `String`, move it into a function that prints it, then try to use it again. Read the error. Fix it two ways: by cloning, and by passing a reference.

**E4.2** Write `fn longest_word(s: &str) -> &str` that returns the longest word in a sentence. Make sure the signature allows returning a slice of the input.

**E4.3** Write `fn append_exclamation(s: &mut String)` that adds `!` to the end. Call it from `main` with a `mut` variable.

**E4.4** Try to create two `&mut` references to the same value in one scope. Read the error. Then rewrite the code so the borrows don't overlap. Explain why in a comment.

**E4.5** Write `fn first_word_index(s: &str) -> usize` that returns the index of the first space. Then use it to slice the string. Notice how the index can go stale if the string changes. This is the motivation for borrow rules.

**E4.6** Given `let a = [1, 2, 3, 4, 5];`, write `fn sum(slice: &[i32]) -> i32` and call it with `&a[1..4]`.

**Tips**

- Ask yourself: "Who owns this value right now?" before every line that moves or borrows.
- `i32`, `bool`, `char`, and tuples of them are `Copy`. `String` and `Vec` are not.
- Prefer `&str` in function parameters. It accepts both `String` and string literals.
- The rule is: many `&` OR one `&mut`, not both at once. Use it to predict errors before the compiler does.
- Don't reach for `.clone()` every time the compiler complains. Read why it complains first.

---

## Ch 5: Structs

**Topics:** defining and instantiating structs, field init shorthand, struct update syntax, tuple structs, unit structs, `impl` blocks, methods (`&self`, `&mut self`, `self`), associated functions, `#[derive(Debug)]`, `dbg!`.

**E5.1** Define `struct Book { title: String, pages: u32, available: bool }`. Create two instances, one using field init shorthand. Print both with `{:?}`.

**E5.2** Use struct update syntax to create a second `Book` that only changes `title`, reusing the rest of the first one.

**E5.3** Define a tuple struct `Point3(f64, f64, f64)` and a method `fn distance_from_origin(&self) -> f64`.

**E5.4** Implement `Rectangle` with fields `width` and `height`, plus:

- `fn area(&self) -> u32`
- `fn can_hold(&self, other: &Rectangle) -> bool`
- `fn square(size: u32) -> Self` as an associated function

**E5.5** Add a `fn scale(&mut self, factor: u32)` method to `Rectangle`. Call it on a `mut` instance and print the new area.

**E5.6** Use `dbg!` on an expression inside a method. Remove it after you've seen the output.

**Tips**

- Use `&self` for read-only methods, `&mut self` for mutation, and `self` only when the method consumes the value.
- `Self::square(5)` and `Rectangle::square(5)` are both associated function calls. No `self` parameter.
- `#[derive(Debug)]` is required before `{:?}` works on your struct.
- `dbg!` prints to stderr with the file and line. It returns the value, so you can wrap expressions in it.
- Struct update syntax `..other` moves the remaining fields. Watch for `String` fields that get moved.

---

## Ch 6: Enums and Pattern Matching

**Topics:** defining enums, enums with data, `Option<T>`, `match`, exhaustiveness, `_` placeholder, `if let`, `while let`, `let else`.

**E6.1** Define `enum Direction { North, South, East, West }`. Write `fn turn_right(d: Direction) -> Direction` using `match`.

**E6.2** Define `enum Message { Quit, Move { x: i32, y: i32 }, Write(String), ChangeColor(u8, u8, u8) }`. Write `fn describe(msg: &Message) -> String` that handles all four variants.

**E6.3** Write `fn divide(a: f64, b: f64) -> Option<f64>` that returns `None` when `b` is zero. Use it in `main` with `match` and then with `if let`.

**E6.4** Write `fn first_even(nums: &[i32]) -> Option<i32>` returning the first even number or `None`. Use `?` inside a function that returns `Option`.

**E6.5** Given `let v = vec![1, 2, 3];`, use `while let Some(x) = stack.pop()` on a `Vec` to pop and print all elements.

**E6.6** Write a `match` on an `i32` that prints `"negative"`, `"zero"`, `"small"` for 1 to 9, and `"large"` for anything else. Use a guard (`if`) for one arm.

**E6.7** Use `let else` to extract a number from a `Option<&str>` and parse it, returning early if it fails.

**Tips**

- `match` must be exhaustive. If the compiler complains about a missing arm, read it. It's telling you a case you forgot.
- Use `_` when you genuinely don't care about a value. Use `_name` if you want to silence the unused-variable warning but keep the name.
- `if let Some(x) = opt { }` is `match` with one arm. Use it when you only care about one case.
- `Option<T>` is in the prelude. You don't need to import it.
- `.unwrap()` is a panic waiting to happen. Prefer `match`, `if let`, `?`, or `unwrap_or`.
- `let else` is for "this must match, otherwise bail out". It keeps the happy path unindented.

---

## Bonus: Mixed Exercise

Build a small **inventory tracker** in one project that uses everything above:

- A `struct Item { name: String, qty: u32, price_cents: u32 }`
- An `enum Command { Add(Item), Remove(String), Restock(String, u32), Report }`
- A function `fn apply(inventory: &mut Vec<Item>, cmd: Command) -> Result<(), String>`. Return an error string for unknown items.
- Process a hard-coded list of commands in `main` and print the final report.

**Tip:** Start with the enum and the `match` in `apply`. The rest follows from the types.
