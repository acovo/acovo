# Changelog

## [0.1.1] - 2025-12-12
### Added
- Comprehensive unit tests for all functions in `src/fs.rs`
- New `FindUsbDevicesByType` function to search for USB devices by device type (product name or vendor name) instead of just VID/PID
- Test cases for the new `FindUsbDevicesByType` function
- Comprehensive documentation comments for all functions in `src/fs.rs` including `get_exe_dir`, `mkdir`, `read_lines`, `write_lines`, `get_exe_parent_path`, `get_current_parent_path`, `get_parent_path`, `list_files`, and `file_name`
- Additional unit tests for all functions in `src/fs.rs` to improve test coverage
- New `write_lines_batched` function for efficient writing of large datasets
- New `read_lines_batched` function for efficient processing of large files
- New `file_exists` function to check if a file or directory exists
- New `file_readable` function to check if a file or directory has read permissions
- New `file_writable` function to check if a file or directory has write permissions
- New `file_modified_seconds_ago` function to calculate the difference in seconds between a file's modification time and the current time with enhanced error handling
- Additional edge case tests for all functions in `src/fs.rs`:
- Enhanced `read_lines` tests to cover files with only whitespace characters and UTF-8 BOM
- Enhanced `write_lines` tests to cover writing lines with only whitespace characters and UTF-8 BOM
- New edge case tests for `read_lines_batched` function covering non-existent files, empty files, extreme batch sizes, and error propagation
- New edge case tests for `write_lines_batched` function covering empty iterators, extreme batch sizes, append mode, and special character content

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
- Fixed `list_files` function to properly handle empty file extensions in search
- Corrected `file_writable` test cases to align with actual function behavior

### Changed
- Updated `LinuxFindUsbDevice` function in `src/dev.rs` to use more robust parsing of `ioreg` output
- Modified test cases in `src/dev.rs` to remove decimal equivalents of hexadecimal product IDs from comments
- Refactored `write_lines` function to use buffered writer for improved performance
- Removed `println!` statement from `write_lines` function
- Optimized `list_files` function to use iterative approach instead of recursion to prevent stack overflow
- Improved error handling in `list_files` function
- Eliminated redundant calls to `extension.unwrap()` in `list_files` function

## [0.1.0] - 2024-01-01
### Added
- Initial release