# Construstor - Smart Contract Constructor & Initialize Function Analyzer

[![Crates.io](https://img.shields.io/crates/v/construstor.svg)](https://crates.io/crates/construstor)
[![Documentation](https://docs.rs/construstor/badge.svg)](https://docs.rs/construstor)
[![Rust](https://img.shields.io/badge/rust-1.86+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/construstor.svg)](https://crates.io/crates/construstor)

A tool for analyzing Solidity smart contracts to detect zero address validation patterns in constructors, initialize functions, and all functions with address parameters.

## 🚀 Features

- **Comprehensive Analysis**: Scans individual files or entire directories for `.sol` files
- **Smart Detection**:
  - Identifies constructors and initialize functions automatically
  - **NEW**: Analyzes all functions with address parameters using `--all-functions` flag
- **Advanced Type Recognition**:
  - Detects address arrays (`address[]`, `address[] memory`, `address[] calldata`)
- **Zero Address Validation Detection**:
  - Direct equality checks (`== address(0)`, `!= address(0)`)
  - `require()` statements with zero address validation
- **Detailed Reporting**:
  - Shows which address arguments are validated with full type information
  - Highlights missing validations per argument
  - Provides summary statistics
  - **NEW**: Complete function definitions displayed in terminal output
- **Multiple Output Formats**:
  - JSON output without code(`--json`)
  - Summary-only mode (`--summary`)
  - Beautiful colored terminal output for human readability

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
# Analyze only constructors and initialize functions (default)
construstor MyContract.sol

# Analyze ALL functions with address parameters
construstor MyContract.sol --all-functions

# Output in JSON format
construstor MyContract.sol --json

# Show only summary statistics
construstor MyContract.sol --summary

# Combine flags
construstor MyContract.sol --all-functions --json

# Interactive mode (will prompt for path)
construstor
```

### Command Line Options

- `--all-functions` / `-a`: Analyze all functions with address parameters, not just constructors and initialize functions
- `--json` / `-j`: Output results in JSON format (excludes code for cleaner output)
- `--summary` / `-s`: Show only summary statistics
- `--help` / `-h`: Display help information
- `--version` / `-V`: Display version information

### Example Output

**Constructor and Initialize Functions (Default)**:

```text
Constructor in MyContract.sol:
📋 Found 2 address argument(s): address _owner, address _manager
✅ Zero address validation found:
  • Direct address(0) comparison
  • require() statement with zero address check
    → Checking variable: _owner
    → Checking variable: _manager
✅ All address arguments are validated!
Arguments: address _owner, address _manager
Code:
  constructor(address _owner, address _manager) {
  require(_owner != address(0), "Owner cannot be zero address");
          require(_manager != address(0), "Manager cannot be zero address");
          owner = _owner;
          manager = _manager;
  }
```

**All Functions Analysis (`--all-functions`)**:

```text
Function 'setTokens' in MyContract.sol:
📋 Found 3 address argument(s): address _token, address[] memory _addresses, address _fallback
✅ Zero address validation found:
  • require() statement with zero address check
    → Checking variable: _token
❌ Missing zero address validation for:
    ⚠️ Argument: _addresses
    ⚠️ Argument: _fallback
Arguments: address _token, address[] memory _addresses, address _fallback
Code:
  function setTokens(address _token, address[] memory _addresses, address _fallback) {
  require(_token != address(0), "Token cannot be zero");
          // Missing validation for _addresses array and _fallback
  }

📊 Analysis Summary:
  Total functions analyzed: 1
  Functions with address arguments: 1
  Fully validated: 0
  Partially validated: 1
  Not validated: 0
```

**JSON Output (`--json`)**:

```json
[
  {
    "function_type": "Constructor",
    "file_name": "MyContract.sol",
    "arguments": "address _owner, address _manager",
    "address_arguments": ["_owner", "_manager"],
    "validated_variables": ["_owner", "_manager"],
    "missing_validations": [],
    "validation_types": ["RequireStatement"]
  }
]
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
constructor(address _owner, address[] memory _tokens) {
    require(_owner != address(0), "Owner cannot be zero");
    // Missing validation for _tokens array ❌
    owner = _owner;
    tokens = _tokens;
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

### All Functions Analysis (with `--all-functions`)

```solidity
function setTokenAddresses(
    address _primary,
    address[] calldata _secondary,
    address storage _fallback
) external onlyOwner {
    require(_primary != address(0), "Primary cannot be zero");
    // Missing validation for _secondary array and _fallback ❌
    primaryToken = _primary;
    secondaryTokens = _secondary;
    fallbackToken = _fallback;
}
```

### Advanced Type Detection

The tool now recognizes various address parameter types:

- **Simple addresses**: `address _owner`
- **Address arrays**: `address[] _tokens`, `address[] memory _list`, `address[] calldata _external`
- **Storage keywords**: `address storage _stored`, `address memory _temp`
- **Mixed parameters**: Functions with both address and non-address parameters

## 🏗️ Architecture

The tool is structured with the following key components:

- **`ConstructorAnalyzer`**: Core analysis engine with regex-based pattern matching
- **`AnalysisResult`**: Structured data representing analysis findings
- **`ResultPrinter`**: Pretty-printed output with colors and formatting
- **Error Handling**: Comprehensive error types and propagation
- **Testing**: Unit tests covering core functionality

## 🔍 Detection Patterns

### Address Parameter Extraction

- **Enhanced Regex**: `(address(?:\[\])?(?:\s+memory|\s+storage|\s+calldata)?)\s+(\w+)`
- **Matches**:
  - Simple: `address _owner`, `address tokenContract`
  - Arrays: `address[] _tokens`, `address[] memory _list`
  - Storage: `address storage _stored`, `address calldata _external`

### Function Detection

- **Constructors**: `constructor\s*\((.*?)\)\s*\{(.*?)\}`
- **Initialize Functions**: `function\s+initialize\s*\((.*?)\)\s*[^{]*\{(.*?)\}`
- **Regular Functions**: `function\s+(\w+)\s*\((.*?)\)\s*[^{]*\{(.*?)\}` (with `--all-functions`)

### Equality Checks

- **Regex**: `(\w+)\s*(?:==|!=)\s*address\(0\)`
- **Matches**: `_owner == address(0)`, `token != address(0)`

### Require Statements

- **Regex**: `(?:require)\s*\(\s*([^,)]+)\s*(?:==|!=)\s*address\(0\)`
- **Matches**: `require(_owner != address(0), "message")`

## 🚨 Security Considerations

This tool helps identify potential security vulnerabilities in smart contracts:

- **Zero Address Attacks**: Prevent accidental or malicious zero address assignments
- **Constructor Security**: Ensure critical addresses are validated during deployment
- **Upgradeable Contracts**: Validate addresses in initialize functions for proxy contracts
- **Function Security**: With `--all-functions`, catch missing validations in all address-handling functions
- **Array Validation**: Detect missing validations for address arrays that could contain zero addresses


### Exit Codes

- `0`: Analysis completed successfully
- `1`: Error occurred during analysis (file not found, invalid syntax, etc.)

### JSON Output Format

The `--json` flag outputs clean JSON without code blocks, perfect for:

- Automated security reporting
- Integration with other tools
- Dashboard visualization
- Audit trail generation

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
