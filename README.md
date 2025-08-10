# rust-learning

Here are some **simple and fun Rust projects** for beginners that will help you grasp the language’s core concepts like ownership, borrowing, pattern matching, enums, structs, error handling, and basic file I/O:

---

## 🔰 1. **Command-Line Todo App**

**What it teaches:** File I/O, structs, enums, `Result`, `match`, argument parsing.

**Features:**

* Add, list, and delete todo items
* Store todos in a local file (`JSON` or `CSV`)
* Use `clap` crate for CLI arguments

📦 Crates to explore:

```toml
clap = "4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 🧠 2. **Guessing Game**

**What it teaches:** Standard I/O, pattern matching, loops, random number generation.

**Features:**

* User inputs a number
* Computer gives hints until guess is correct

📦 Crate:

```toml
rand = "0.8"
```

```rust
use rand::Rng;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        match guess.trim().parse::<u32>() {
            Ok(n) if n == secret => {
                println!("Correct!");
                break;
            }
            Ok(n) if n < secret => println!("Too low!"),
            Ok(_) => println!("Too high!"),
            Err(_) => println!("Enter a number."),
        }
    }
}
```

---

## 📚 3. **Simple Text File Reader**

**What it teaches:** File I/O, error handling, CLI basics

**Features:**

* Pass file path as argument
* Print contents line by line

```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
    let path = env::args().nth(1).expect("Provide a file path");
    let file = File::open(&path).expect("Could not open file");
    for line in io::BufReader::new(file).lines() {
        println!("{}", line.unwrap());
    }
}
```

---

## 🧪 4. **Simple Calculator**

**What it teaches:** Match, enums, functions

**Features:**

* Accept two numbers and an operator
* Perform + - \* / operations

---

## 💬 5. **Mini Chat Simulator (Local)**

**What it teaches:** Threads, channels, concurrency

**Features:**

* Simulate a sender and receiver thread
* Messages sent through a channel

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("Hello from thread!").unwrap();
    });

    println!("Received: {}", rx.recv().unwrap());
}
```

---

## 🌐 6. **HTTP GET Client**

**What it teaches:** HTTP requests, external crates, error handling

**Features:**

* Accept a URL as argument
* Print response body

📦 Crate:

```toml
reqwest = { version = "0.11", features = ["blocking"] }
```

---

## 📂 7. **Folder Size Analyzer**

**What it teaches:** Recursion, file traversal, error handling

**Features:**

* Walk through a directory
* Sum sizes of all files

📦 Crate:

```toml
walkdir = "2"
```

---

## 🧠 Bonus Project Ideas

* Markdown to HTML converter
* Weather CLI app (uses API)
* Stopwatch / timer in terminal
* Snake game using `crossterm`
* JSON pretty printer

---

## 🚀 Tools and Crates to Explore

| Crate       | Purpose            |
| ----------- | ------------------ |
| `clap`      | Command-line args  |
| `serde`     | Serialization      |
| `reqwest`   | HTTP client        |
| `tokio`     | Async runtime      |
| `rayon`     | Parallel iterators |
| `crossterm` | Terminal UIs       |

---

Would you like a **detailed walkthrough of one** of these projects or a **step-by-step template**?
