# Expressing ideas in Rust

Miguel Palhas <mpalhas@gmail.com>

SMARTT November 2022

---

## What makes a language good/bad?

---

## For me

### Security
It should be hard to do the wrong thing

### Efficiency
It should be fast do write, and fast to run

### Expressiveness
How clearly it can express the developer's intentions

### Tooling / Community
To help us achieve all the other three

---

# Security


```javascript
// JavaScript
"1" + 1
```

```solidity
// Solidity
msg.sender.call(data);
```

```c++
// C++
int* pointer = NULL;
printf("%d", *pointer);
```

---


## Expressiveness

```javascript
// JavaScript
function fib(max) {
  let n0=0, n1=1;

  for (let i = 1; i < max; i++) {
    let nextN = n0 + n1;
    n0=n1;
    n1=nextN;
  }

  return n1;
}
```

```haskell
-- Haskell
fib 0 = 0
fib 1 = 1
fib n = fib (n-1) + fib (n-2)
```

---

# #1 Most loved language&ast;, but why?

1. Excellent docs
2. Excellent compiler
3. Modern typing system
4. Fast, but also *safe* by default
5. Excellent tooling (`cargo`, `clippy`, `rustfmt`, `rust-analyzer`, ...)

<br>

&ast; StackOverflow developer surveys 2022, 2021, 2020, 2019, 2018, 2017, 2016

---

# Hello World

```rust
fn main() {
    println!("Hello, World!");
}
```

---

# Hello World

```rust
fn main() {
    let name = "Miguel";
    let year = 2022;
    println!("Hello, {name}, welcome to {age}!");
}
```

Do you see any types here?

---

# Hello World

```rust
fn main() {
    let name = "Miguel"; // &str
    let year = 2022; // i32
    println!("Hello, {name}, welcome to {age}!");
}
```

Rust tries to infer types whenever possible, with one exception...

---

# Hello World

```rust
fn main() {
    hello("Miguel", 2022);
}

fn hello(name: &str, age: i32) {
    println!("Hello, {name}, welcome to {age}!");
}
```

Function headers and new types require explicit types. This is a design choice:
* promotes readability
* makes the compiler's job much easier
* less error-prone

---

# Collections & Generics

---
```rust
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 1, 2, 3];
}
```

The type is not actually needed. It will be infered.

---
```rust
fn main() {
    let v = vec![1, 2, 3, 1, 2, 3];
}

```

---

```rust
fn main() {
    let v = vec![1, 2, 3, 1, 2, 3];

    let set: HashSet<i32> = v.iter().cloned().collect::<HashSet<i32>>();
    // set = [1, 2, 3]
}
```

---

```rust
fn main() {
    let v = vec![1, 2, 3, 1, 2, 3];

    let set: HashSet<i32> = v.iter().cloned().collect::<HashSet<_>>();
    // set = [1, 2, 3]
}
```

---

```rust
fn main() {
    let v = vec![1, 2, 3, 1, 2, 3];

    let set: HashSet<i32> = v.iter().cloned().collect();
}
```

---

```rust
fn main() {
    let v = vec![1, 2, 3, 1, 2, 3];

    let set: HashSet<_> = v.iter().cloned().collect();
}
```

---

```rust
fn main() {
    let v = vec![1, 2, 3, 1, 2, 3];

    let vec2: Vec<_> = v.iter().cloned().collect();
    // vec2 = [1, 2, 3, 1, 2, 3]
}
```

---

```rust
fn main() {
    let v = vec![1, 2, 3, 1, 2, 3];

    // this would not compile
    // rust doesn't know which collection to return
    let whatever = v.iter().collect();
}
```

---

```rust
fn foo() -> HashSet<i32> {
    let v = vec![1, 2, 3, 1, 2, 3];

    // but this compiles
    let whatever = v.iter().collect();

    return whatever;
}
```

---

# Traits

---

```rust
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p);
}
```

This fails with:

```text
println!("{}", p);
               ^ `Point` cannot be formatted with the default formatter
```

---

```rust
struct Point {
    x: i32,
    y: i32
}

use std::fmt::{Display, Formatter, Result};

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p);
}
```

`Point` now implements `Display` (for short: `Point` is `Display`).

But this is cumbersome, and most of the time is only useful for debugging purposes

---

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}
```

Output:

```text
Point { x: 1, y: 2 }
```

Other derivable traits: `Clone`, `Copy`, `Default`, `Eq`, `Hash`, ...

---

# Trait magic

---

## pt. 1, `serde`

A SERializer/DEserializer

We'll focus on `serde_json` specifically, but serde is a generic framework for serialization formats.

```rust
struct Person {
    name: String,
    age: u8,
    emails: Vec<String>
}
```

---

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    emails: Vec<String>
}

fn main() {
    let data = r#"{ "name": "John Doe", "age": 43, "emails": [ "mpalhas@gmail.com", "miguel@subvisual.co" ] }"#;

    // this parses the string correctly into a struct
    // if the string does not match an error is returned
    // (which we're here ignoring with `unwrap()`)
    let person: Person = serde_json::from_str(data).unwrap();


    // we can also do the reverse
    let json: String = serde_json::to_string(&person).unwrap();
}
```

---

```rust
#[derive(Serialize, Deserialize)]
struct Person {
    #[serde(alias = "full_name")]
    name: String,        // now either `name` and `full_name` is accepted

    age: Option<u8>,     // age may not be present

    #[serde(skip)]
    emails: Vec<String> 
}
```

---

## pt. 2, `clap`

```rust
struct Args {
    name: String,
    count: u8,
    foo: String,
    max: Option<u8>,
}
```

---

## pt. 2, `clap`

```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(long, env = "FOO")]
    foo: String,

    #[arg(short, long)]
    max: Option<u8>,
}
```

---

## pt. 2, `clap`

```shell
cargo run -- --help
Usage: smartt [OPTIONS] --name <NAME> --foo <FOO>

Options:
  -n, --name <NAME>
  -c, --count <COUNT>  [default: 1]
      --foo <FOO>      [env: FOO=]
  -m, --max <MAX>
  -h, --help           Print help information
```

---

## pt. 3, `From` / `Into`

```rust
struct Imperial {
    feet: u32,
    inches: u32
}

struct Metric {
  centimeters: u32
}

fn is_short(h: Metric) -> bool {
  h.centimeters <= 170
}
```

---

## pt. 3, `From` / `Into`

```rust
fn main() {
    let alice = Metric { centimeters: 180 };
    let bob = Imperial { feet: 5, inches: 1 };

    is_short(alice);
    // how to do this other ones?
    // is_short(bob);
}
```

---

## pt. 3, `From` / `Into`

```rust
impl From<Imperial> for Metric {
    fn from(i: Imperial) -> Metric {
        (i.feet * 30.0f32 + i.inches * 2.0f32).floor()
    }
}

fn main() {
    let bob = Imperial { feet: 5, inches: 1 };

    is_short(bob.into::<Metric>::())
}
```

---

## pt. 3, `From` / `Into`

```rust
impl From<Imperial> for Metric {
    fn from(i: Imperial) -> Metric {
        (i.feet * 30.0f32 + i.inches * 2.0f32).floor()
    }
}

fn main() {
    let bob = Imperial { feet: 5, inches: 1 };

    is_short(bob.into());
}
```

---

## pt. 3, `From` / `Into`

```rust
fn is_short<T>(h: T) -> bool
where T: Into<Metric>
{
    h.into().centimeters <= 170
}

fn main() {
    is_short(alice);
    is_short(bob);
}
```

---

# The Borrow Checker (skip maybe?)

---

## 70% of all Microsoft's exploits are memory-related

---

## Solutions to memory management

**Manual** (C, C++)
- ✅ (Illusion of) control is in developers' hands
- ❌ Pretty much Everything

**Garbage collecting** (Java, Ruby, Erlang/Elixir)
- ✅ Abstracted from developers; heavily researched topic
- ❌ Introduces runtime cost & unpredictability

**Borrow checker** (Rust)
- ✅ Fully tracked by the compiler. Memory release is injected automatically, and invalid usage triggers compile-time errors
- ✅ Prevents data races by default.
- ❌ Requires more upfront work (both by developers & compiler)
- ❌ Tricky to understand at times

---

> [The borrow checker] operates by a few simple rules.
> If you don’t understand or at least have some intuition for what those rules are,
> then it’s going to be about as useful as using a spell checker to help you write in a language you don’t even know: it’ll just reject everything you say.

> Once you know the rules the borrow checker is based on, you’ll find it useful rather than oppressive and annoying, just like a spell checker.

---

## Borrow checking example

```rust
fn gimme_a_vec<T>(_: Vec<T>) {}

fn main() {
    let v = vec![2, 3, 5, 7, 11, 13, 17];
    gimme_a_vec(v);
    let element = v.get(0);

    println!("{:?}", element);
}
```

Compile-time error:

    let v = vec![2, 3, 5, 7, 11, 13, 17];
      |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
    5 |     gimme_a_vec(v);
      |                 - value moved here
    6 |     let element = v.get(3);
      |                   ^^^^^^^^ value borrowed here after move

---

## Solution #1: cloning

```rust
fn gimme_a_vec<T>(_: Vec<T>) {}

fn main() {
    let v = vec![2, 3, 5, 7, 11, 13, 17];
    gimme_a_vec(v.clone());
    let element = v.get(0);

    println!("{:?}", element);
}
```

---

## Solution #2: References

```rust
fn gimme_a_vec<T>(_: &Vec<T>) {}

fn main() {
    let v = vec![2, 3, 5, 7, 11, 13, 17];
    gimme_a_vec(&v);
    let element = v.get(0);

    println!("{:?}", element);
}
```

---

## Trying to mutate it now

```rust
fn gimme_a_vec(v: &Vec<i32>) {
  v[0] = 1;
}

fn main() {
    let v = vec![2, 3, 5, 7, 11, 13, 17];
    gimme_a_vec(&v);
    let element = v.get(0);

    println!("{:?}", element);
}
```

---

## Trying to mutate it now, fixed

```rust
fn gimme_a_vec(v: &mut Vec<i32>) { // receive a mut reference
  v[0] = 1;
}

fn main() {
    let mut v = vec![2, 3, 5, 7, 11, 13, 17]; // declare as mut
    gimme_a_vec(&mut v); // send as mut
    let element = v.get(0);

    println!("{:?}", element);
}
```

---

## Thread-safety

```rust
fn gimme_a_vec(v: &mut Vec<i32>) {
  v[0] = 1;
}

fn main() {
    let mut v = vec![2, 3, 5, 7, 11, 13, 17];
    
    let _thread = std::thread::spawn(move || { gimme_a_vec(&mut v); });
    
    let element = v.get(0);

    println!("{:?}", element);
}
```

Compile-time error:

    7  |     
    8  |     let _thread = std::thread::spawn(move || {
       |                                      ------- value moved into closure here
       ...
    12 |     let element = v.get(0);
       |                   ^^^^^^^^ value borrowed here after move

---

## Borrow Checker

* Explicit mutability
* Compiler checks lifetime of each reference
* Compiler checks borrowing/moving rules
* Multiple immutable references; single mutable reference

What do we get for this extra hard work?

* No data races
* No forbidden memory accesses
* More explicit code

---

# Async

---

## Async basics

```rust
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let task = tokio::spawn(async move {
      // async operation
    });

    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);

    task.await?;

    Ok(())
}
```

---

## Tooling

---

## `rustfmt` - Code formatter

* similar phisolophy to `prettier`
* works in real time (blazing fast™)
* near-zero config followed by the entire community
* first class citizen, not an external plugin (meaning better language support)
* runs in your editor

```rust
# my personal rustfmt.toml

edition = "2021"
group_imports = "StdExternalCrate"
```

---

## `clippy`

The best linter I've ever seen

* huge repository of lints to catch
* a lot of lints are things you'd never think a linter would/could catch
* runs in your editor, with near-instant feedback

---

## `clippy` - suspicious arithmetic implementations

```rust
type Foo(i32);

// the `Add` trait allows us
// to do `foo1 + foo2`
impl std::ops::Add for Foo {
    type Output = Foo;

    fn add(self, other: Foo) -> Foo {
        Foo(self.0 - other.0)
    }
}
```

Can you spot the error?

---

## `clippy` - suspicious arithmetic implementations (solution)

```rust
type Foo(i32);

// the `Add` trait allows us
// to do `foo1 + foo2`
impl std::ops::Add for Foo {
    type Output = Foo;

    fn add(self, other: Foo) -> Foo {
        Foo(self.0 - other.0) // suspicious use of `-` in `Add` impl ...
    }
}
```

Documentation:

> Lints for suspicious operations in impls of arithmetic operators, e.g. subtracting elements in an Add impl.

> This is probably a typo or copy-and-paste error and not intended.

---

## `clippy` - approximate constants

```rust
let x = 3.14156;
// approximate value of `f32::consts::PI` found
```

---

## `clippy` - invisible characters

```rust
const DANGER: &str = "there's a zero-width space or soft hyphen some­where in this text.";
```

Compile-time error:

> help: consider replacing the string with: `"there's a zero-width space or soft hyphen some\u{AD}where in this text."`

---

## `clippy` - overly complex bool expressions

```rust
// The `b` is unnecessary, the expression is equivalent to `if a`.
if a && b || a { ... }
```

---

## `clippy` - possible missing comma

```rust
let a = &[
    -1, -2, -3 // <= no comma here
    -4, -5, -6
];
```
---

## `cargo`

* `cargo add ethers`
* `cargo build --release`
* `cargo fmt`
* `cargo clippy`
* `cargo test`
* `cargo watch -x test`

---

## `rust-analyzer`

Live demo!

---

# 3rd party crates

---

## `clap` - Command Line Argument Parser

Live demo!

---
immutability

you can implement traits for existing types

ethers-rs
wasm
actix/rocket/axum
sqlx

yew


rustfmt
clippy
rust-analyzer
