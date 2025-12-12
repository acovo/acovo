# Changelog

### v0.1.1 - 2024-12-12

### Added
 - Enhanced zip module with comprehensive unit tests
   - Added tests for valid ZIP file extraction
   - Added tests for directory structure preservation
   - Added tests for empty filename and destination handling
   - Added tests for special character support in filenames
   - Added tests for multiple file extraction
 - Enhanced proto module unit tests with comprehensive edge case coverage
  - Added boundary condition tests for Request::validate method
  - Added edge case tests for Response::new_with_state method
  - Added additional State structure test cases
  - Added error handling and special scenario tests
  - Added boundary condition tests for Response::raiseRequestError method
- Enhanced time module with comprehensive documentation and additional unit tests
  - Added detailed function documentation for all time-related functions
  - Added comprehensive unit tests for time parsing and formatting functions
  - Added tests for different timezone handling
  - Added tests for timestamp parsing edge cases
  - Added extensive edge case testing for time functions
  - Added format validation tests for date strings
  - Added comprehensive timezone coverage tests
- Enhanced trace module with additional unit tests
  - Added compilation tests for trace macros
  - Added tests for LocalTimeFormatter and get_exe_dir functionality
  - Created integration tests for trace module features
- Optimized trace module implementation
  - Improved module organization and documentation
  - Enhanced init_tracing and init_global_tracing macros with better error handling
  - Expanded test coverage for trace functionality

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
- Refactored trace module with improved code organization and enhanced documentation

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