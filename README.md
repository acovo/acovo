# Acovo Library

A public Rust framework providing utilities for system-level operations.

## Features

- **Time**: Time and timezone handling utilities
- **File System**: File system operations and utilities
  - `read_file(path)`: Read file contents as bytes
  - `write_file(path, data)`: Write data to a file
  - `delete_file(path)`: Delete a file at the specified path
  - `list_directory(path)`: List files and directories in a given path
- **Hash**: Cryptographic hashing functions
- **Network**: Network-related utilities
- **Protobuf**: Protocol buffer serialization support
- **Tracing**: Logging and tracing capabilities
- **HTTP**: HTTP client functionality
- **Compression**: ZIP compression utilities
- **Development**: Development and debugging utilities
- **Device Detection**: Cross-platform device detection (including USB devices)

## File System Operations

The library includes comprehensive file system operations:

### Available Functions

- `get_exe_dir()`: Get the directory of the currently running executable
- `mkdir(path)`: Create a directory at the specified path
- `read_lines(file)`: Read lines from a file
- `write_lines(file, lines, create)`: Write lines to a file
- `get_exe_parent_path()`: Get the parent directory of the currently running executable
- `get_current_parent_path()`: Get the parent directory of the current working directory
- `get_parent_path(path)`: Get the parent directory of a given path
- `list_files(dir, ext)`: List files with a specific extension in a directory (recursively)
- `file_name(path)`: Extract the file name from a path

## USB Device Detection (macOS)

The library includes enhanced USB device detection capabilities on macOS, with specific improvements for:

- Apple devices
- ASIX devices (AX88179)
- GenesysLogic devices
- Logitech devices
- MACROSILICON devices
- Razer devices

### Available Functions

- `LinuxFindUsbDevice(vid, pid)`: Search for a USB device by Vendor ID and Product ID
- `ListUsbDevices()`: List all connected USB devices
- `FindUsbDevicesByType(device_type)`: Search for USB devices by device type (product name or vendor name)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
acovo = "0.1.0"
```

## Usage

Enable specific features in your `Cargo.toml`:

```toml
[dependencies]
acovo = { version = "0.1.0", features = ["time", "fs", "hash", "trace", "proto", "net", "dev"] }
```

## Examples

See the `examples/` directory for usage examples, including:
- `debug_usb.rs`: Demonstrates USB device detection capabilities

## Testing

Run tests with:

```bash
cargo test
```

For device-specific tests:

```bash
cargo test --features dev -- dev::tests::test_find_usb_device
```

## License

MIT