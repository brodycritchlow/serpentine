# Contributing to Serpentine

Thank you for your interest in contributing to Serpentine! We love your input! We want to make contributing to this project as easy and transparent as possible.

## 🎯 Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct:
- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community

## 🚀 Getting Started

### Prerequisites

- Rust 1.88 or higher (for 2024 edition and let-chains support)
- Git
- Python 3.8+ (for testing against Python files)

### Setting Up Your Development Environment

```bash
# Fork and clone the repository
git clone https://github.com/brodycritchlow/serpentine.git
cd serpentine

# Build the project
cargo build

# Run tests
cargo test

# Run the type checker locally
cargo run -- test_script.py
```

## 🔧 How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues. When you create a bug report, include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Code samples that demonstrate the issue
- Your environment (OS, Rust version, etc.)

### Suggesting Features

Feature suggestions are welcome! Please:

- Check if the feature has already been suggested
- Provide a clear use case
- Explain why this feature would be useful
- Consider how it fits with Serpentine's goals

### Pull Request Process

1. **Fork the repository** and create your branch from `main`:
   ```bash
   git checkout -b feature/amazing-feature
   ```

2. **Write tests** for your changes:
   ```rust
   #[test]
   fn test_my_new_feature() {
       // Your test here
   }
   ```

3. **Ensure all tests pass**:
   ```bash
   cargo test
   ```

4. **Check your code style**:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   ```

5. **Commit your changes** using conventional commits:
   ```bash
   git commit -m "feat: add support for type aliases"
   git commit -m "fix: handle empty tuple literals"
   git commit -m "docs: update type inference documentation"
   ```

6. **Push to your fork** and submit a pull request

## 📝 Development Guidelines

### Code Style

- Follow Rust naming conventions
- Use `rustfmt` for formatting
- Fix all `clippy` warnings
- Document public APIs with doc comments

### Testing

- Write unit tests for new functionality
- Add integration tests for user-facing features
- Include test cases that cover edge cases
- Ensure tests are deterministic and fast

### Documentation

- Update relevant documentation
- Add doc comments to public functions
- Include examples in doc comments
- Update README.md if adding user-facing features

## 🏗️ Project Structure

```
serpentine/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── main.rs          # CLI entry point
│   └── python_type.rs   # Type definitions
├── tests/               # Integration tests
├── arch-docs/          # Architecture documentation
└── examples/           # Example Python files
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_type_inference

# Run benchmarks
cargo bench
```

### Writing Tests

```rust
#[test]
fn test_literal_type_checking() {
    let mut checker = TypeChecker::new();
    let code = r#"x: int = 42"#;
    
    assert!(checker.analyze_source(code).is_ok());
    assert_eq!(
        checker.get_variable_type("x"),
        Some(&PythonType::Int)
    );
}
```

## 📚 Understanding the Codebase

Key concepts to understand:

1. **Type System**: How Python types are represented in Rust
2. **AST Processing**: How we parse and analyze Python code
3. **Type Inference**: How types are inferred from expressions
4. **Error Handling**: How type errors are detected and reported

See our [architecture documentation](arch-docs/) for detailed information.

## 💡 Good First Issues

Look for issues labeled `good first issue` for tasks that are suitable for newcomers:

- Adding support for new Python expressions
- Improving error messages
- Adding more test cases
- Updating documentation

## 🤝 Review Process

All submissions require review. We use GitHub pull requests for this purpose. Expect feedback within 48 hours.

### What We Look For

- **Correctness**: Does the code do what it claims?
- **Tests**: Are there adequate tests?
- **Performance**: Does it maintain Serpentine's performance goals?
- **Documentation**: Is it well documented?
- **Style**: Does it follow our conventions?

## 📋 Commit Message Guidelines

We use conventional commits:

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test additions or fixes
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `chore:` Maintenance tasks

Examples:
```
feat: add support for f-string type checking
fix: correctly handle nested tuple types
docs: add examples for union type checking
test: add edge cases for literal types
```

## 🙏 Recognition

Contributors will be recognized in our README. We appreciate all contributions, big and small!

## ❓ Questions?

Feel free to open an issue with the `question` label or reach out to the maintainers.

---

Happy coding! 🐍🦀