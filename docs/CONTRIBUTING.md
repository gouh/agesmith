# Contributing to AgeSmith

Thank you for your interest in contributing to AgeSmith! This document provides guidelines and instructions for contributing.

## 🤝 Ways to Contribute

- **Report Bugs**: Open an issue with details
- **Suggest Features**: Share your ideas in discussions
- **Fix Bugs**: Submit a PR fixing an issue
- **Add Features**: Implement planned features from roadmap
- **Improve Documentation**: Help make docs better
- **Write Tests**: Increase test coverage
- **Review PRs**: Help review other contributions

## 🚀 Getting Started

### 1. Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/agesmith.git
cd agesmith
```

### 2. Set Up Development Environment

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
brew install sops age  # macOS
# or
sudo apt install age   # Linux

# Build the project
cargo build

# Run tests
cargo test
```

### 3. Create a Branch

```bash
git checkout -b feature/my-feature
# or
git checkout -b fix/bug-description
```

## 📝 Development Workflow

### Making Changes

1. **Write Code**: Follow Rust best practices
2. **Add Tests**: Write tests for new functionality
3. **Run Tests**: Ensure all tests pass
4. **Check Style**: Run `cargo fmt` and `cargo clippy`
5. **Commit**: Write clear commit messages

### Code Style

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Run tests
cargo test
```

### Commit Messages

Use clear, descriptive commit messages:

```
feat: Add password strength indicator
fix: Handle special characters in ENV files
docs: Update installation instructions
test: Add tests for secret generator
refactor: Simplify event handling logic
```

## 🧪 Testing

### Writing Tests

See [WRITING_TESTS.md](WRITING_TESTS.md) for detailed guide.

```rust
#[test]
fn test_my_feature() {
    // Arrange
    let input = "test";
    
    // Act
    let result = my_function(input);
    
    // Assert
    assert_eq!(result, expected);
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_my_feature

# With output
cargo test -- --nocapture
```

## 📋 Pull Request Process

### Before Submitting

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] New tests added for new features
- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] Documentation updated if needed

### Submitting PR

1. **Push to your fork**
   ```bash
   git push origin feature/my-feature
   ```

2. **Create Pull Request** on GitHub

3. **Fill out PR template** with:
   - Description of changes
   - Related issue number
   - Testing performed
   - Screenshots (if UI changes)

4. **Wait for review** and address feedback

### PR Review Process

- Maintainers will review your PR
- Address any requested changes
- Once approved, PR will be merged

## 🐛 Reporting Bugs

### Before Reporting

- Check if bug already reported
- Try to reproduce the bug
- Gather relevant information

### Bug Report Template

```markdown
**Description**
Clear description of the bug

**Steps to Reproduce**
1. Step one
2. Step two
3. See error

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Environment**
- OS: macOS 14.0
- Rust version: 1.70
- AgeSmith version: 0.1.0

**Additional Context**
Any other relevant information
```

## 💡 Suggesting Features

### Feature Request Template

```markdown
**Feature Description**
Clear description of the feature

**Use Case**
Why is this feature needed?

**Proposed Solution**
How should it work?

**Alternatives Considered**
Other approaches you've thought about

**Additional Context**
Mockups, examples, etc.
```

## 📚 Documentation

### Improving Docs

- Fix typos and errors
- Add missing information
- Improve clarity
- Add examples
- Update outdated content

### Documentation Standards

- Clear and concise
- Include code examples
- Use proper formatting
- Keep up-to-date

## 🎯 Priority Areas

Current priorities for contributions:

1. **Testing**: Increase test coverage
2. **Documentation**: Improve guides and examples
3. **Bug Fixes**: Fix known issues
4. **Performance**: Optimize slow operations
5. **Features**: Implement roadmap items

See [ROADMAP.md](ROADMAP.md) for planned features.

## ✅ Code Review Checklist

Reviewers will check:

- [ ] Code quality and style
- [ ] Test coverage
- [ ] Documentation updates
- [ ] No breaking changes (or properly documented)
- [ ] Performance impact
- [ ] Security considerations

## 🏆 Recognition

Contributors will be:

- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Credited in commit history

## 📧 Getting Help

- **Questions**: GitHub Discussions
- **Issues**: GitHub Issues
- **Chat**: (Coming soon)

## 📜 Code of Conduct

Be respectful and professional:

- Be welcoming and inclusive
- Respect differing viewpoints
- Accept constructive criticism
- Focus on what's best for the project
- Show empathy towards others

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to AgeSmith! 🎉
