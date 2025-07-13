# Construstor - Smart Contract Constructor & Initialize Function Analyzer

[![Crates.io](https://img.shields.io/crates/v/construstor.svg)](https://crates.io/crates/construstor)
[![Documentation](https://docs.rs/construstor/badge.svg)](https://docs.rs/construstor)
[![Rust](https://img.shields.io/badge/rust-1.86+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/construstor.svg)](https://crates.io/crates/construstor)

A production-ready tool for analyzing Solidity smart contracts to detect zero address validation patterns in constructors and initialize functions.

## 🚀 Features

- **Comprehensive Analysis**: Scans individual files or entire directories for `.sol` files
- **Smart Detection**: Identifies constructors and initialize functions automatically
- **Zero Address Validation Detection**:
  - Direct equality checks (`== address(0)`, `!= address(0)`)
  - `require()` statements with zero address validation
- **Detailed Reporting**:
  - Shows which address arguments are validated
  - Highlights missing validations
  - Provides summary statistics
- **Beautiful Output**: Colored terminal output for better readability
- **Production Ready**: Comprehensive error handling, logging, and testing

## 📦 Installation

### Prerequisites

- [Rust](https://rustup.rs/) 1.86 or later

### Build from Source

```bash
git clone https://github.com/oakenknight/construstor.git
cd construstor
cargo build --release
```

### Install from Crates.io

```bash
cargo install construstor
```

The binary will be available at `target/release/construstor`.

## 🔧 Usage

### Basic Usage

Run the tool with a file or directory:

```bash
# Using the installed version
construstor MyContract.sol

# Or if building from source
cargo run -- MyContract.sol

# Interactive mode (will prompt for path)
construstor
```

### Example Output

```text
Constructor in MyContract.sol:
📋 Found 2 address argument(s): _owner, _manager
✅ Zero address validation found:
  • Direct address(0) comparison
  • require() statement with zero address check
    → Checking variable: _owner
    → Checking variable: _manager
✅ All address arguments are validated!

Initialize function in MyContract.sol:
📋 Found 3 address argument(s): _tokenA, _tokenB, _router
✅ Zero address validation found:
  • require() statement with zero address check
    → Checking variable: _tokenA
    → Checking variable: _tokenB
❌ Missing zero address validation for:
    ⚠️ Argument: _router

📊 Analysis Summary:
  Total functions analyzed: 2
  Functions with address arguments: 2
  Fully validated: 1
  Partially validated: 1
  Not validated: 0
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run tests with verbose output:

```bash
cargo test -- --nocapture
```

## 📊 What It Detects

### Constructor Analysis

```solidity
constructor(address _owner, address _token) {
    require(_owner != address(0), "Owner cannot be zero");
    // Missing validation for _token ❌
    owner = _owner;
    token = _token;
}
```

### Initialize Function Analysis

```solidity
function initialize(address _hookManager, address _test) external initializer {
    require(_hookManager != address(0), "Hook manager cannot be zero address");
    if (_test == address(0)) revert("Test cannot be zero address");
    // Both arguments validated ✅
}
```

## 🏗️ Architecture

The tool is structured with the following key components:

- **`ConstructorAnalyzer`**: Core analysis engine with regex-based pattern matching
- **`AnalysisResult`**: Structured data representing analysis findings
- **`ResultPrinter`**: Pretty-printed output with colors and formatting
- **Error Handling**: Comprehensive error types and propagation
- **Testing**: Unit tests covering core functionality

## 🔍 Detection Patterns

### Address Parameter Extraction

- Regex: `address\s+(\w+)`
- Matches: `address _owner`, `address tokenContract`

### Equality Checks

- Regex: `(\w+)\s*(?:==|!=)\s*address\(0\)`
- Matches: `_owner == address(0)`, `token != address(0)`

### Require Statements

- Regex: `(?:require)\s*\(\s*([^,)]+)\s*(?:==|!=)\s*address\(0\)`
- Matches: `require(_owner != address(0), "message")`

## 🚨 Security Considerations

This tool helps identify potential security vulnerabilities in smart contracts:

- **Zero Address Attacks**: Prevent accidental or malicious zero address assignments
- **Constructor Security**: Ensure critical addresses are validated during deployment
- **Upgradeable Contracts**: Validate addresses in initialize functions for proxy contracts

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### Development Guidelines

- Add tests for new functionality
- Follow Rust naming conventions
- Update documentation for new features
- Ensure `cargo clippy` passes without warnings

## 🙏 Acknowledgments

- Rust community for excellent tooling and documentation
- Solidity developers for security best practices
- Smart contract auditing community for inspiration
- Special thanks to Wyatt Chamberlin ([@elkaholic6](https://github.com/elkaholic6)) for giving me the idea with his [Solidity-constructor-analysis](https://github.com/elkaholic6/Solidity-constructor-analysis)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
