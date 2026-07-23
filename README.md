# Rust 101 Workshop

A two-day hands-on Rust workshop. You will write real code from the first session, no slides-only stretches :)). Every concept is immediately followed by an exercise you implement.

---

## Prerequisites

- A laptop with internet access
- A GitHub account
- Git installed (`git --version` should work in your terminal)
- VS Code (recommended) or any editor you're comfortable with

---

## Step 0 — Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify the install:

```bash
rustc --version   # should be 1.78.0 or later
cargo --version
```

If you already have Rust installed, make sure it's up to date:

```bash
rustup update
```

---

## Getting Started

### Step 1 — Fork the repo

Go to: **https://github.com/nkatha23/rust101-workshop**

Click **Fork** in the top right. This creates your own copy — `YOUR_USERNAME/rust101-workshop` — where you'll push all your solutions.

### Step 2 — Clone your fork

```bash
git clone git@github.com:YOUR_USERNAME/rust101-workshop.git
cd rust101-workshop
```

Replace `YOUR_USERNAME` with your actual GitHub username.

### Step 3 — Open in VS Code

```bash
code .
```

Install the **rust-analyzer** extension.

### Step 4 — Verify everything compiles

```bash
cargo test --workspace
```

You will see failing tests — **that is expected**. Every `todo!()` is waiting for your implementation. The goal across both days is to replace every `todo!()` with working code until all tests pass.

---

## Folder Structure

```
rust101-workshop/
├── day1/
│   ├── session2_syntax/            # Rust syntax basics
│   ├── session3_integers_variables/ # Integer types, variables, BTC/sats
│   ├── session4_branching_loops/   # if/else, loop, while, for
│   ├── session5_panics/            # Panics and when to use them
│   ├── session6_overflow/          # Integer overflow and safe arithmetic
│   └── session7_capstone/          # Day 1 group project: BTC/sats calculator
├── day2/
│   ├── session8_structs_ownership/ # Structs, methods, ownership
│   ├── session9_validation/        # Result, error handling
│   ├── session10_modules/          # Modules and code organisation
│   ├── session11_stack_heap/       # Stack vs heap, Box, Vec
│   └── session12_destructors/      # Drop trait, Day 2 group project: UTXO wallet
└── solutions/
    ├── day1/
    └── day2/
```

Each session folder contains:

| File | Purpose |
|---|---|
| `src/lib.rs` | `todo!()` stubs — filled in together during the session |
| `tests/tests.rs` | CI-checked tests — go green when your implementation is correct |

Capstone sessions (7 and 12) also have a `src/main.rs` — a runnable program your group builds and presents at the end of the day.

---

## How Exercises Work

### During each session

Open `src/lib.rs` for the current session. You'll work through the `todo!()` stubs together as a group. Run the tests at any point to see how you're doing:

```bash
cargo test -p session3_integers_variables
```

When you're happy with a session, push it:

```bash
git add .
git commit -m "session3: implement btc_to_sats"
git push
```

GitHub will run the full test suite automatically. Check the **Actions** tab on your fork to see green or red.

### End of day — capstone (group project)

Sessions 7 and 12 are group capstones. Work in small groups to complete the `src/main.rs` TODOs, then run it and present to the room:

```bash
cargo run -p session7_capstone
```

---

## CI — Continuous Integration

Every push to your fork triggers a GitHub Actions workflow that runs:

```
cargo fmt --check       checks code is formatted correctly
cargo clippy            checks for common mistakes
cargo test --workspace  runs all tests across every session
```

You can see results under the **Actions** tab on your GitHub fork.

- **Green checkmark** — all tests pass, well done
- **Red cross** — something needs fixing, check the log to see which test failed

### Run the same checks locally before pushing

```bash
cargo fmt --all              # auto-format your code
cargo clippy --all-targets   # check for issues
cargo test --workspace       # run all tests
```

To run tests for a single session only:

```bash
cargo test -p session3_integers_variables
```

---

## Day 1 Overview

| Session | Topic | Key concepts |
|---|---|---|
| 2 | Syntax basics | Functions, types, `fn`, `let`, `&str`, `String` |
| 3 | Integers & variables | `u64`, `i32`, `f64`, constants, BTC/sats conversion |
| 4 | Branching & loops | `if`, `match`, `loop`, `while`, `for`, iterators |
| 5 | Panics | `panic!`, `unwrap`, `expect`, index bounds |
| 6 | Overflow | Checked/saturating/wrapping arithmetic |
| 7 | **Capstone** | BTC/sats calculator CLI (group project) |

## Day 2 Overview

| Session | Topic | Key concepts |
|---|---|---|
| 8 | Structs & ownership | `struct`, `impl`, move semantics, borrowing |
| 9 | Validation | `Result`, `?` operator, custom error types |
| 10 | Modules | `mod`, `pub`, `use`, code organisation |
| 11 | Stack & heap | `Box<T>`, `Vec<T>`, stack vs heap allocation |
| 12 | **Capstone** | UTXO wallet with `Drop` trait (group project) |

---

## Useful Commands

```bash
# Check a specific package compiles
cargo check -p session4_branching_loops

# Run tests for one session
cargo test -p session4_branching_loops

# Run a capstone (sessions 7 and 12)
cargo run -p session7_capstone

# Format all code
cargo fmt --all

# Lint all code
cargo clippy --workspace --all-targets
```

---

## Getting Help

- Ask your neighbour first
- Flag the facilitator
- Rust docs: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/

---

*Built for the Rust 101 Workshop. Have fun, and welcome to Rust.*
