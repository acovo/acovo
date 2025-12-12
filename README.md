# Acovo Library

A public Rust framework providing utilities for system-level operations.

## Features

- **Time**: Time and timezone handling utilities
- **File System**: File system operations and utilities
- **Hash**: Cryptographic hashing functions
- **Network**: Network-related utilities
- **Protobuf**: Protocol buffer serialization support
- **Tracing**: Logging and tracing capabilities
- **HTTP**: HTTP client functionality
- **Compression**: ZIP compression utilities
- **Development**: Development and debugging utilities
- **Device Detection**: Cross-platform device detection (including USB devices)

## USB Device Detection (macOS)

The library includes enhanced USB device detection capabilities on macOS, with specific improvements for:

- Apple devices
- ASIX devices (AX88179)
- GenesysLogic devices
- Logitech devices
- MACROSILICON devices
- Razer devices

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