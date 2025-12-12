# Changelog

## [Unreleased]

## [0.1.1] - 2025-12-12

### Fixed
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