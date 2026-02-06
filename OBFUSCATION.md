# Using rust-obfuscator

This document explains how to use the `rust-obfuscator` tool to automatically obfuscate the funsomware source code.

## What is rust-obfuscator?

`rust-obfuscator` is an automatic tool that processes Rust source files and:
- Automatically wraps string literals with `cryptify::encrypt_string!()` macros
- Inserts `cryptify::flow_stmt!()` macros into functions for control flow obfuscation
- Optionally renames variables to make code harder to read

## Installation

### Method 1: Build from source

```bash
# Clone the rust-obfuscator repository
git clone https://github.com/dronavallipranav/rust-obfuscator.git
cd rust-obfuscator

# Build the tool
cargo build --release --bin rust-obfuscator

# Copy binary to your project root
cp target/release/rust-obfuscator /path/to/funsomware/
```

### Method 2: Direct build in project

```bash
# From your funsomware directory
git clone https://github.com/dronavallipranav/rust-obfuscator.git temp-obfuscator
cd temp-obfuscator
cargo build --release --bin rust-obfuscator
cp target/release/rust-obfuscator ../
cd ..
rm -rf temp-obfuscator
```

## Usage

### Basic Usage

The tool processes either a single file or an entire directory:

```bash
# Obfuscate all files in src/ directory
./rust-obfuscator src/

# Obfuscate a single file
./rust-obfuscator src/main.rs
```

The obfuscated code will be created in a new `obfuscated_code/` directory.

### Command Line Options

- `--no_string` - Disable string obfuscation (don't wrap strings in `encrypt_string!`)
- `--no_flow` - Disable control flow obfuscation (don't insert `flow_stmt!`)
- `--disable_macro` - Use direct source manipulation instead of macros
- `--var` - Enable variable renaming (experimental)

### Examples

```bash
# Only obfuscate strings, skip control flow
./rust-obfuscator src/ --no_flow

# Only obfuscate control flow, skip strings
./rust-obfuscator src/ --no_string

# Enable variable renaming (may require manual fixes)
./rust-obfuscator src/ --var
```

## Workflow

1. **Backup your source code** (important!)
   ```bash
   cp -r src src_backup
   ```

2. **Run the obfuscator**
   ```bash
   ./rust-obfuscator src/
   ```

3. **Review the obfuscated code**
   ```bash
   ls obfuscated_code/
   ```

4. **Format the obfuscated code** (recommended)
   ```bash
   cd obfuscated_code
   cargo fmt --all
   ```

5. **Replace your source with obfuscated version**
   ```bash
   rm -rf src
   mv obfuscated_code/src src
   ```

6. **Build and test**
   ```bash
   cargo build --release
   cargo test
   ```

## Environment Variables

Set `CRYPTIFY_KEY` for custom encryption key:

```bash
export CRYPTIFY_KEY="your-custom-key-here"
./rust-obfuscator src/
```

If not set, cryptify uses a default fixed key.

## Example Transformation

### Before (Original Code)

```rust
use cryptify;

fn main() {
    let message = "Hello, World!";
    println!("{}", message);
}

fn process_data(data: &str) {
    let result = data.to_uppercase();
    println!("Result: {}", result);
}
```

### After (Obfuscated Code)

```rust
use cryptify;

fn main() {
    cryptify::flow_stmt!();
    let message = cryptify::encrypt_string!("Hello, World!");
    println!("{}", message);
}

fn process_data(data: &str) {
    cryptify::flow_stmt!();
    let result = data.to_uppercase();
    println!("{}", cryptify::encrypt_string!("Result: "), result);
}
```

## GitHub Actions Integration

The project includes two workflows:

1. **`build-windows.yml`** - Standard build without extra obfuscation
2. **`build-windows-obfuscated.yml`** - Automatically runs rust-obfuscator before building

Both workflows trigger on pushes to `master`/`main` branches.

## Notes

- The obfuscator modifies code structure, so always format with `cargo fmt` after
- Variable renaming (`--var`) is experimental and may require manual fixes
- The tool only processes `.rs` files in the specified directory (not subdirectories)
- Always keep a backup of your original source code
- Obfuscation happens at the source level, so you can still debug the obfuscated code

## Troubleshooting

### Compilation errors after obfuscation

1. Run `cargo fmt` on the obfuscated code
2. Check for any syntax issues in the generated code
3. If using `--var`, some variable names might conflict - fix manually

### Obfuscator not found

Make sure the binary is executable:
```bash
chmod +x rust-obfuscator
```

### Missing cryptify import

Ensure all files have:
```rust
use cryptify;
```

The obfuscator expects this import to be present.
