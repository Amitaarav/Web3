# Rust Assignments (100 Problems)

## Solution Guide
To get the most out of these assignments, it is recommended to solve them in the following order:
1. **Fundamentals**: Start here to master Rust basics, syntax, and core ownership concepts.
2. **Intermediate**: Move to this section for traits, generics, and more complex data structures.
3. **Async**: Finally, tackle asynchronous programming and concurrency.


## How to Run

Tests must be run from the **crate root** (e.g., `cd fundamentals`) or from the workspace root using the `-p` flag.

### Run a specific problem
To run tests for a specific problem (e.g., `sum`), use:
```bash
cargo test sum
```

### Run all tests in a specific part
```bash
cargo test -p fundamentals
cargo test -p intermediate
cargo test -p async
```

### Run all 100 problems at once
```bash
cargo test
```


## How to Evaluate Assignment

Assignments are verified by running their corresponding tests. Each problem has an associated test file located in the `tests/` directory of its crate.

### Steps to Evaluate:

1.  **Navigate to the Crate Directory**:
    Go to the specific part you are working on (e.g., `cd async`).

2.  **Run the Test for the Problem**:
    Each problem follow the naming convention `<problem_name>_test.rs` in the `tests/` folder. Use the following command:
    ```bash
    cargo test --test <problem_name>_test
    ```

    *Example:* To evaluate the `concurrent_compute` problem in the `async` crate:
    ```bash
    cargo test --test concurrent_compute_test
    ```

3.  **Check Output**:
    The solution is considered correct when the test passes without panics.

---
## Found bug ?
If you find bug in any of the file, please report in discord Web3 Chat.