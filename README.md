# Funsomware

A Rust-based file encryption tool with obfuscated code using the `cryptify` crate.

## Features

- **Multi-threaded processing**: Uses 64 threads by default (configurable)
- **Fast XOR cipher**: Custom FNV-1a-inspired stream cipher for fast encryption
- **Code obfuscation**: All strings and control flows obfuscated with `cryptify`
- **Windows message boxes**: Random-positioned message boxes all over the screen (Windows only)
- **Wallpaper changer**: Drops and sets a custom wallpaper after encryption
- **Embedded resources**: Wallpaper image embedded in the binary
- **Configurable**: Easy-to-modify configuration in `src/config.rs`
- **CLI output**: Optional debug output showing processing status

## Configuration

Edit `src/config.rs` to customize:

- `TARGET_DIR`: Directory to process (default: empty = auto-detect)
  - Windows: Uses Documents folder
  - Linux: Uses `/tmp/target`
- `PASSWORD`: Encryption password (default: `MySecretPassword123!`)
- `THREAD_COUNT`: Number of worker threads (default: `64`)
- `SHOW_CLI`: Show debug output (default: `true`)
- `SHOW_MESSAGEBOXES`: Show GUI message boxes (default: `true`)
- `MESSAGEBOX_LOOP_COUNT`: How many times to show the message box (default: `100`, set to `0` for infinite)
- `MESSAGEBOX_DELAY_MS`: Delay between message boxes in milliseconds (default: `2000`)
- `SET_WALLPAPER`: Change desktop wallpaper after encryption (default: `true`)
- `MESSAGEBOX_TITLE`: Custom message box title (default: "⚠ CRITICAL SYSTEM ERROR ⚠")
- `MESSAGEBOX_TEXT`: Custom message box text (default: cryptic error message)

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/funsomware
```

## How It Works

1. Determines target directory (Windows: Documents, Linux: /tmp/target)
2. Recursively scans the target directory for files
3. Processes each file in parallel using a thread pool
4. For each file:
   - Reads the entire file into memory
   - Encrypts using XOR stream cipher with the configured password
   - Sleeps for a random duration (1-5 seconds)
   - Writes the encrypted data back to the file
5. **Extracts and sets wallpaper** to warning screen
6. **Spawns message box hell** (Windows only) - 100 message boxes appear at RANDOM POSITIONS all over the screen! 😈
   - Each message box runs in its own thread
   - They don't wait for user interaction
   - **Appear at random screen coordinates** - not in the center!
   - New ones keep spawning every 2 seconds while old ones are still open
   - Creates complete desktop chaos
   - Linux: Message boxes are skipped (Windows-only feature)

## Encryption Algorithm

The encryption uses a custom XOR stream cipher:

- **Key expansion**: FNV-1a-inspired hash function generates a keystream from the password
- **Encryption**: Each byte is XORed with the corresponding keystream byte
- **Symmetric**: The same operation encrypts and decrypts

**Note**: This cipher is designed for speed, not cryptographic security.

## Code Obfuscation

The project uses the `cryptify` crate to obfuscate:

- All user-facing strings via `encrypt_string!` macro
- Control flow via `flow_stmt!` macro
- Makes reverse engineering more difficult

For automatic source code obfuscation using `rust-obfuscator`, see [OBFUSCATION.md](OBFUSCATION.md).

## Project Structure

```
funsomware/
├── Cargo.toml                # Dependencies and project metadata
├── rsrc/
│   └── wallpaper.png         # Embedded wallpaper image
├── src/
│   ├── main.rs               # Entry point with CLI banner
│   ├── config.rs             # Configuration constants
│   ├── crypto.rs             # XOR stream cipher implementation
│   ├── worker.rs             # Multi-threaded file processing
│   ├── wallpaper_manager.rs  # Wallpaper extraction and setting
│   └── messagebox_spawner.rs # Non-blocking message box spawner
├── generate_wallpaper.py     # Script to create custom wallpaper
└── README.md                 # This file
```

## Dependencies

- `cryptify` - Compile-time code obfuscation
- `rayon` - Data parallelism library
- `walkdir` - Recursive directory traversal
- `rand` - Random number generation
- `wallpaper` - Cross-platform wallpaper setting
- `dirs-next` - Platform-specific directory detection
- `winapi` - Windows API for random-positioned message boxes (Windows only)

### Linux Requirements

On Linux, you need either **Zenity** or **Kdialog** installed for message boxes to work:

```bash
# Ubuntu/Debian
sudo apt install zenity

# Fedora/RHEL
sudo dnf install zenity

# Arch
sudo pacman -S zenity
```

## Testing

Generate test files for encryption:

```bash
python3 generate_test_files.py
```

This creates 300 random files (1-50MB each) in the target directory.

## GitHub Actions

The project includes automated Windows builds:

- **`build-windows.yml`** - Standard release build on every push to master
- **`build-windows-obfuscated.yml`** - Build with automatic rust-obfuscator processing

Artifacts are uploaded and available for download from the Actions tab.

To enable custom encryption keys, set the `CRYPTIFY_KEY` secret in your repository settings.

## License

This is educational software. Use responsibly and only on files you own.
