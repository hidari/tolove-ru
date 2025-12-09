# Contributing to ToLOVE-ru

Thank you for your interest in contributing to ToLOVE-ru! 💜

## Development Setup

1. Clone the repository:
```bash
git clone https://github.com/hidari/tolove-ru.git
cd tolove-ru
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## Code Quality Standards

Before submitting a pull request, ensure your code meets our quality standards:

### 1. Testing

**All new features and bug fixes must include tests.**

#### Writing Unit Tests

Unit tests are located in the same file as the code being tested, under a `#[cfg(test)]` module at the bottom of the file.

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function("input");
        assert_eq!(result, "expected");
    }
}
```

#### Writing Integration Tests

Integration tests are located in the `tests/` directory. Each file in `tests/` is a separate test suite.

Example (`tests/my_feature.rs`):
```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_feature() {
    let mut cmd = Command::cargo_bin("love").unwrap();
    cmd.arg("--my-option");
    cmd.assert().success();
}
```

#### Test Coverage Guidelines

- **Security functions**: 100% coverage required
- **Core logic**: 90%+ coverage expected
- **Helper functions**: 80%+ coverage expected
- **Integration tests**: Cover all user-facing features

#### Test Naming Convention

Use descriptive test names that follow this pattern:
```
test_<function_name>_<condition>_<expected_result>
```

Examples:
- `test_sanitize_input_removes_control_characters`
- `test_validate_message_rejects_long_input`
- `test_parse_color_returns_default_for_invalid_input`

### 2. Code Formatting

We use `rustfmt` with default settings:

```bash
cargo fmt
```

Check formatting without modifying files:
```bash
cargo fmt --check
```

### 3. Linting

We use `clippy` with strict warnings:

```bash
cargo clippy -- -D warnings
```

Apply auto-fixes:
```bash
cargo clippy --fix
```

### 4. Security Best Practices

This project prioritizes security. Follow these guidelines:

#### Input Validation

- **Always sanitize user input** before processing
- **Validate input length** to prevent resource exhaustion
- **Filter control characters** to prevent terminal injection attacks

Example:
```rust
fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|&c| {
            let code = c as u32;
            (0x20..0x7F).contains(&code) || c == '\t' || c == '\n'
        })
        .collect()
}
```

#### Error Handling

- **Never use `panic!`** in production code
- **Use `Result` types** for error propagation
- **Provide meaningful error messages**

Example:
```rust
// Bad
if input.is_empty() {
    panic!("Input is empty!");
}

// Good
if input.is_empty() {
    return Err("Input cannot be empty".to_string());
}
```

### 5. Documentation

- Add doc comments (`///`) for public functions
- Keep comments concise and focused on "why", not "what"
- Update README.md if adding user-facing features

## Pull Request Process

1. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make your changes** following the code quality standards above

3. **Run all checks**:
   ```bash
   cargo fmt
   cargo clippy --fix
   cargo test
   ```

4. **Commit your changes** with a descriptive message:
   ```bash
   git commit -m "feat: add new color option for hearts"
   ```

   Use conventional commit prefixes:
   - `feat:` - New features
   - `fix:` - Bug fixes
   - `test:` - Test additions or modifications
   - `docs:` - Documentation updates
   - `refactor:` - Code refactoring without behavior changes
   - `perf:` - Performance improvements
   - `style:` - Code formatting changes
   - `ci:` - CI/CD configuration changes

5. **Push to your fork**:
   ```bash
   git push origin feature/my-feature
   ```

6. **Create a Pull Request** on GitHub

7. **Address review feedback** if requested

## Testing Checklist

Before submitting a PR, ensure:

- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] New features include tests
- [ ] Tests cover edge cases and error conditions
- [ ] Documentation is updated if needed

## Example: Adding a New Feature

Here's a complete example of adding a new color option:

1. **Add the logic**:
```rust
fn parse_color(color_str: &str) -> Color {
    match color_str {
        "purple" => Color::Magenta, // New color
        // ... existing colors
        _ => Color::White,
    }
}
```

2. **Add unit tests**:
```rust
#[test]
fn test_parse_color_purple() {
    let color = parse_color("purple");
    assert_eq!(color, Color::Magenta);
}
```

3. **Add integration test**:
```rust
#[test]
fn test_cli_purple_color() {
    let mut cmd = Command::cargo_bin("love").unwrap();
    cmd.arg("--color").arg("purple");
    cmd.timeout(std::time::Duration::from_millis(100));
    let _ = cmd.ok();
}
```

4. **Update documentation**:
   - Add "purple" to the color list in README.md
   - Update `--help` text if needed

5. **Run quality checks**:
```bash
cargo fmt
cargo clippy --fix
cargo test
```

## Questions?

If you have questions about contributing, feel free to:
- Open an issue on GitHub
- Ask in the pull request discussion

Thank you for contributing! 💜
