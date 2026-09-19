# Answers — VISHNURAJ D

Application for: **Rust Blockchain Developer**

## About You

### 1. Introduce yourself

I'm Vishnuraj D, a blockchain infrastructure developer from India (MCA, Cochin
University of Science and Technology). Most of my recent work has been around
Rust, TypeScript and Solidity: I built GlobalPay, a blockchain settlement
infrastructure project on Base Sepolia with USDC ERC-20 deterministic
execution, an MPC threshold-signing setup across three local signing nodes
communicating over raw TCP/UDP sockets, and a proof layer anchoring settlement
states to Hedera HCS consensus topics with mirror-node verification. I care a
lot about security-first design — tamper-proof workflows, signature
verification, chaos probing — and I enjoy the systems side of things:
networking, concurrency, Linux, and making code clean enough to audit. I also
enjoy the unglamorous side of good engineering — profiling, debugging, and
squeezing performance out of databases and key-value stores.

I'm applying for the Rust Blockchain Developer role because it lines up
directly with what I like building: production blockchain systems where
correctness and security actually matter.

### 2. Do you own a personal computer?

Yes — a MacBook Air M1, which is my primary development machine.

### 3. Describe your development environment

- **Machine:** MacBook Air M1 (Apple Silicon)
- **OS:** macOS 15 (Sequoia)
- **Editor/IDE:** VS Code with the rust-analyzer extension
- **Shell:** zsh (macOS default); bash also available
- **Package management:** Homebrew for system tools, rustup for the Rust toolchain
- **Toolchain:** Rust (rustc/cargo), Node.js, Python 3, Docker, and git for
  version control; Docker Compose for containerized work and PM2 for process
  management on deployments.

## Social Profile

1. **Personal website / profiles:**
   - Portfolio: https://my-portfolio-black-rho-52.vercel.app/
   - GitHub: https://github.com/vi5hnuraj
   - LinkedIn: https://www.linkedin.com/in/vi5hnuraj/

## The real stuff

All puzzles are answered in **Rust**, since that's the language I'm applying
with. The complete, tested implementation is in `src/lib.rs`, with unit tests
in the same file and a runnable demo in `src/main.rs` (`cargo run`).

### 1. Which all programming languages are installed on your system?

Rust (rustc/cargo), JavaScript/TypeScript (Node.js), Python 3, and shell
scripting (zsh/bash). I also work with SQL through database tooling
(PostgreSQL/Supabase) in my projects.

### 2. Write a function that takes a number and returns a list of its digits in an array

```rust
pub fn digits(n: i64) -> Vec<u32> {
    let s = n.abs().to_string();
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}
```

Examples: `digits(12345) == [1, 2, 3, 4, 5]`, `digits(-402) == [4, 0, 2]`,
`digits(0) == [0]`. The sign is ignored, so negative numbers return the digits
of their absolute value.

### 3. Remove duplicates of an array and returning an array of only unique elements

```rust
pub fn unique<T: PartialEq + Clone>(items: &[T]) -> Vec<T> {
    let mut out: Vec<T> = Vec::with_capacity(items.len());
    for item in items {
        if !out.contains(item) {
            out.push(item.clone());
        }
    }
    out
}
```

Keeps the first occurrence of each element and preserves the original order:
`unique(&[1, 2, 2, 3, 1, 4]) == [1, 2, 3, 4]`. It's generic, so it works for
any element type that supports equality (ints, strings, custom types, ...).

### 4. Write function that translates a text to Pig Latin and back

```rust
pub fn to_pig_latin(text: &str) -> String {
    translate(text, true)
}

pub fn from_pig_latin(text: &str) -> String {
    translate(text, false)
}
```

- `to_pig_latin("The quick brown fox")` returns `"Hetay uickqay rownbay oxfay"`
  — exactly the example from the question.
- `from_pig_latin("Hetay uickqay rownbay oxfay")` returns
  `"The quick brown fox"` — the round trip back to English.
- Case is preserved (`The` -> `Hetay`, and back), and punctuation attached to
  a word stays in place (`here.` -> `erehay.`).
- Single-letter words like `I` translate to `Iay` and back.

The full implementation (with the word-level logic in `translate_word`) is in
`src/lib.rs`, covered by round-trip unit tests.

### 5. Write a function that rotates a list by `k` elements

```rust
pub fn rotate<T>(items: &mut [T], k: usize) {
    let len = items.len();
    if len == 0 {
        return;
    }
    let k = k % len; // rotating by len (or a multiple) changes nothing
    if k == 0 {
        return;
    }
    // Reverse the whole slice, then the two halves around the split point.
    reverse(items, 0, len - 1);
    reverse(items, 0, len - k - 1);
    reverse(items, len - k, len - 1);
}
```

- Rotates **in place** — no copy of the list is created and no extra buffer is
  allocated; the input slice is mutated directly.
- `[1,2,3,4,5,6]` rotated by `2` becomes `[3,4,5,6,1,2]`, as required.
- **Operations count:** every element is swapped at most twice across the
  three reversals, so it needs exactly `n` swap operations in total
  (`k/2 + (n-k)/2 + n/2` rounded down, which sums to `n`). That's O(n) time
  with O(1) extra memory.
- Edge cases handled: empty list, `k = 0`, and `k >= n` (reduced with
  modulo, so rotating by `8` on a 6-element list behaves like rotating by `2`).

## How to run this code

```bash
cargo test   # runs all 11 unit tests
cargo run    # small demo printing the output of every puzzle
```
