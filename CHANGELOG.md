# Changelog

## [Unreleased]

### Added
- Comprehensive unit tests for all functions in `src/fs.rs`
- New `FindUsbDevicesByType` function to search for USB devices by device type (product name or vendor name) instead of just VID/PID
- Test cases for the new `FindUsbDevicesByType` function
- Comprehensive documentation comments for all functions in `src/fs.rs` including `get_exe_dir`, `mkdir`, `read_lines`, `write_lines`, `get_exe_parent_path`, `get_current_parent_path`, `get_parent_path`, `list_files`, and `file_name`

### Fixed
- Improved error handling in `get_exe_parent_path` and `get_current_parent_path` functions in `src/fs.rs`
- Optimized `list_files` function in `src/fs.rs` to avoid unwrap() calls
- Removed redundant `format!` macro usage in `write_lines` function in `src/fs.rs`
- Improved `file_name` function in `src/fs.rs` to handle edge cases better
- Improved USB device detection on macOS by expanding search range from ±20 to ±30 lines
- Enhanced search algorithm to first collect device block lines before checking for product ID
- Resolved issues with detecting specific devices:
  - Apple USB3 Gen2 Hub (05ac:800c)
  - UGREEN 35287 (2b89:5287)
  - AX88179 (0b95:1790)
  - USB Optical Mouse (046d:c077)
  - Razer Cynosa Pro (1532:020d)

### Changed
- Updated `LinuxFindUsbDevice` function in `src/dev.rs` to use more robust parsing of `ioreg` output
- Modified test cases in `src/dev.rs` to remove decimal equivalents of hexadecimal product IDs from comments

## [0.1.0] - 2024-01-01
### Added
- Initial release