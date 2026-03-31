## Coding Mandates

- Always run `cargo test` at the end of a complete logical code change, and make sure.

- All tests go in `src/tests/`, not in `src/`

- Never write production code without writing tests in TDD fashion.

### TDD

- Follow TDD principles when writing code, meaning:
    1. Write a failing test that defines a function or improvements of a function
    2. Write the minimum amount of code to make the test pass
    3. Refactor the code while keeping the tests passing