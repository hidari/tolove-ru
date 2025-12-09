# ToLOVE-ru

A lovely terminal heart animation (pronounced as "trouble") 💜

Watch a heart float up in your terminal with optional messages inside!

![demo.gif](assets/demo.gif)

## Installation

### Pre-built binaries

You can download pre-built binaries for various platforms from the [GitHub releases page](https://github.com/hidari/tolove-ru/releases).

### From source

```bash
cargo install tolove-ru
```

## Usage

Just run:
```bash
love
```

### Options

- `--message <TEXT>` - Display a message inside the heart
- `--petite` - Show a smaller heart
- `--color <COLOR>` - Change the heart color (available: red, green, blue, yellow, magenta, cyan, white)
- `-h, --help` - Show help message

### Examples

Basic usage:
```bash
love
```

With a message:
```bash
love --message "I love Rust"
```

Small heart:
```bash
love --petite
```

Colorful heart:
```bash
love --color red
```

## Development

### Running Tests

This project has comprehensive test coverage (85%+) including:
- Unit tests for security functions, core logic, and color parsing
- Integration tests for CLI arguments and error handling

Run all tests:
```bash
cargo test
```

Run only unit tests:
```bash
cargo test --lib
```

Run only integration tests:
```bash
cargo test --test '*'
```

Run tests with output:
```bash
cargo test -- --nocapture
```

### Code Quality

Check formatting:
```bash
cargo fmt --check
```

Run linter:
```bash
cargo clippy
```

Apply auto-fixes:
```bash
cargo fmt
cargo clippy --fix
```

### Test Coverage

The project maintains 85%+ test coverage across:
- **Security functions** (100%): Input sanitization and validation
- **Core logic** (90%+): Heart shape calculations and sizing
- **Color parsing** (100%): Color string conversion
- **CLI handling** (80%+): Argument parsing and error messages

For detailed testing guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the LICENSE file for details.
