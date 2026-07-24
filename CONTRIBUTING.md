# Contributing to NovaTunnel

Thank you for your interest in contributing to NovaTunnel! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Rust 1.70+
- Node.js 18+
- pnpm 8+

### Development Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/novatunnel.git
   cd novatunnel
   ```

3. Install dependencies:
   ```bash
   cd ui
   pnpm install
   cd ..
   ```

4. Start development:
   ```bash
   # Terminal 1: Start UI dev server
   cd ui
   pnpm dev

   # Terminal 2: Run the application
   cargo run --package novatunnel-app
   ```

## Code Style

### Rust

- Follow the Rust API Guidelines
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for linting issues
- Write documentation for public APIs

### TypeScript/React

- Use TypeScript for all code
- Follow the existing code style
- Use functional components with hooks
- Write meaningful component and function names

## Testing

### Rust Tests

```bash
cargo test --all
```

### Linting

```bash
# Rust
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# TypeScript (in ui directory)
pnpm lint
```

## Submitting Changes

1. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes
3. Write or update tests as needed
4. Ensure all tests pass:
   ```bash
   cargo test --all
   ```

5. Commit your changes with a clear message:
   ```bash
   git commit -m "feat: add new feature description"
   ```

6. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

7. Create a Pull Request

## Commit Messages

Use conventional commits format:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

Examples:
```
feat: add WireGuard backend support
fix: resolve DNS leak on Windows
docs: update installation instructions
```

## Reporting Issues

When reporting issues, please include:

- Operating system and version
- Rust version (`rustc --version`)
- Node.js version (`node --version`)
- Steps to reproduce the issue
- Expected behavior
- Actual behavior

## License

By contributing to NovaTunnel, you agree that your contributions will be licensed under the MIT License.
