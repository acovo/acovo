# Changelog

### v0.1.1 - 2025-04-05

### Added
- Enhanced proto module unit tests with comprehensive edge case coverage
  - Added boundary condition tests for Request::validate method
  - Added edge case tests for Response::new_with_state method
  - Added additional State structure test cases
  - Added error handling and special scenario tests
  - Added boundary condition tests for Response::raiseRequestError method
- Added tests for None body serialization in Request and Response structures
- Added tests for minimal field headers in RequestHeader and ResponseHeader
- Added tests for high return code values in State structure
- Added tests for special characters and Unicode support
- Implemented macOS platform network functionality support
- Enhanced time module with comprehensive documentation and additional unit tests
  - Added detailed function documentation for all time-related functions
  - Added comprehensive unit tests for time parsing and formatting functions
  - Added tests for different timezone handling
  - Added tests for timestamp parsing edge cases
  - Added extensive edge case testing for time functions
  - Added format validation tests for date strings
  - Added comprehensive timezone coverage tests
- Enhanced proto module unit tests with comprehensive edge case coverage
  - Added boundary condition tests for Request::validate method
  - Added edge case tests for Response::new_with_state method
  - Added additional State structure test cases
  - Added error handling and special scenario tests
  - Added boundary condition tests for Response::raiseRequestError method
- Added tests for None body serialization in Request and Response structures
- Added tests for minimal field headers in RequestHeader and ResponseHeader
- Added tests for high return code values in State structure
- Added tests for special characters and Unicode support
- Implemented macOS platform network functionality support

### Fixed
- Fixed ownership transfer issues in proto module tests
- Added Clone trait derivation to State structure to prevent compilation errors
- Fixed compilation errors in time module tests by importing required chrono traits

### Changed
- Refactored network module code structure
- Optimized conditional compilation configuration for Linux and macOS network implementations
- Improved ping command parameter handling across different platforms
- Enhanced error handling mechanism for macOS network functionality
- Improved time module documentation with detailed explanations and examples

## [Historical Versions]

## [0.1.0] - 2024-12-12
- Initial release
- Basic filesystem operations
- Hash functions (CRC64)
- HTTP client functionality
- USB device detection
- Protocol buffer support
- Synchronization primitives
- Time utilities
- Zip file extraction
- Logging and tracing capabilities