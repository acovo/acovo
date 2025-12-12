use anyhow::{anyhow, Result as AnyResult};
use std::process::Command;

#[cfg(feature = "dev")]
/// Search for a USB device on Linux/macOS using the specified Vendor ID (VID) and Product ID (PID).
/// 
/// This function uses different system utilities depending on the platform:
/// - Linux: Uses `lsusb` command
/// - macOS: Uses `ioreg` command
/// 
/// # Parameters
/// - `vid`: The Vendor ID to search for (without "0x" prefix)
/// - `pid`: The Product ID to search for (without "0x" prefix)
/// 
/// # Returns
/// - `Ok(true)`: If the device is found
/// - `Ok(false)`: If the device is not found
/// - `Err`: If there's an error executing the system command
/// 
/// # Platform Notes
/// - On Linux, requires `lsusb` to be installed (usually part of usbutils package)
/// - On macOS, uses the built-in `ioreg` command
/// - On other platforms, returns an error indicating lack of support
/// 
/// # Example
/// ```rust
/// use acovo::dev::LinuxFindUsbDevice;
/// 
/// #[cfg(feature = "dev")]
/// match LinuxFindUsbDevice("05ac", "1234") {
///     Ok(found) => println!("Device found: {}", found),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[cfg(feature = "dev")]
pub fn LinuxFindUsbDevice(vid: &str, pid: &str) -> AnyResult<bool> {
    use std::process::Command;
    use anyhow::{anyhow, Result as AnyResult};
    
    // Format the device ID for searching
    let dev_id = format!("{}:{}", vid, pid);
    
    // Determine which command to use based on the operating system
    if cfg!(target_os = "linux") {
        // Execute the lsusb command on Linux
        match Command::new("/bin/lsusb").output() {
            Ok(output) => {
                // Convert the command output to a UTF-8 string
                let sOutput = String::from_utf8(output.stdout)?;
                
                // Log the lsusb output for debugging purposes
                tracing::debug!("LSUSB-OUTPUT:\n{}", sOutput);
                
                // Check if the output is empty (indicating an error)
                if sOutput.len() == 0 {
                    // If stdout is empty, check stderr for error information
                    let sErr = String::from_utf8(output.stderr)?;
                    return Err(anyhow!("check-usb-device-error {}", sErr));
                }
                
                // Split the output into individual device entries (one per line)
                let usblist = sOutput.split("\n");
                
                // Iterate through each device entry to find a match
                for dev in usblist {
                    // Check if this device entry contains our target device ID
                    if dev.contains(&dev_id) {
                        // Device found, return success
                        return Ok(true);
                    }
                }
                
                // No matching device found after checking all entries
                return Ok(false);
            }
            // Handle errors from executing the lsusb command
            Err(e) => {
                Err(anyhow!("Failed to execute lsusb command: {}. Check that lsusb is installed (usually part of usbutils package).", e))
            }
        }
    } else if cfg!(target_os = "macos") {
        // Execute the ioreg command on macOS with detailed output
        match Command::new("/usr/sbin/ioreg").args(["-p", "IOUSB", "-w", "0", "-l"]).output() {
            Ok(output) => {
                // Convert the command output to a UTF-8 string
                let sOutput = String::from_utf8(output.stdout)?;
                
                // Log the ioreg output for debugging purposes
                tracing::debug!("IOREG-OUTPUT:\n{}", sOutput);
                
                // Check if the output is empty (indicating an error)
                if sOutput.len() == 0 {
                    // If stdout is empty, check stderr for error information
                    let sErr = String::from_utf8(output.stderr)?;
                    return Err(anyhow!("check-usb-device-error {}", sErr));
                }
                
                // On macOS with ioreg, we need to parse the detailed output structure
                // Look for devices with matching idVendor and idProduct values
                // ioreg outputs idVendor and idProduct as decimal values, e.g. "idVendor" = 2965
                // We need to convert our hex input to decimal for comparison
                
                // Parse the vendor ID from hex to decimal
                let vid_decimal = u16::from_str_radix(vid, 16).map_err(|e| anyhow!("Invalid vendor ID format: {}", e))?;
                
                // Look for the device in the ioreg output by searching for the vendor ID pattern
                // Format in ioreg output: "idVendor" = 2965
                let vid_pattern = format!("\"idVendor\" = {}", vid_decimal);
                
                // Split the output into lines for easier processing
                let lines: Vec<&str> = sOutput.lines().collect();
                
                // If product ID is empty, we only search for vendor ID
                if pid.is_empty() {
                    // Search for devices with matching vendor ID
                    for line in &lines {
                        if line.contains(&vid_pattern) {
                            return Ok(true);
                        }
                    }
                    return Ok(false);
                }
                
                // Parse the product ID from hex to decimal
                let pid_decimal = u16::from_str_radix(pid, 16).map_err(|e| anyhow!("Invalid product ID format: {}", e))?;
                // Format in ioreg output: "idProduct" = 6032
                let pid_pattern = format!("\"idProduct\" = {}", pid_decimal);
                
                // Search for devices with matching vendor ID
                for i in 0..lines.len() {
                    if lines[i].contains(&vid_pattern) {
                        // Found a device with matching vendor ID, now check if it has the matching product ID
                        // Look in the surrounding lines for the product ID (typically within a few lines)
                        let start = i.saturating_sub(30); // Look up to 30 lines before
                        let end = std::cmp::min(i + 30, lines.len()); // Look up to 30 lines after
                        
                        // First, collect all lines in the device block
                        let mut device_block_lines = Vec::new();
                        for j in start..end {
                            device_block_lines.push(lines[j]);
                        }
                        
                        // Then check if any of these lines contain the product ID
                        for line in &device_block_lines {
                            if line.contains(&pid_pattern) {
                                // Found both vendor and product ID matching
                                return Ok(true);
                            }
                        }
                    }
                }
                
                // No matching device found
                return Ok(false);
            }
            // Handle errors from executing the ioreg command
            Err(e) => {
                Err(anyhow!("Failed to execute ioreg command: {}. Check that ioreg is available on this system.", e))
            }
        }
    } else {
        // Unsupported platform
        Err(anyhow!("USB device detection is not supported on this platform ({}). Only Linux and macOS are supported.", std::env::consts::OS))
    }
}

#[cfg(feature = "dev")]
/// Search for USB devices by device type (product name or vendor name) on Linux/macOS.
/// 
/// This function searches for USB devices based on their product or vendor names rather than IDs.
/// On macOS, it uses the `ioreg` command to find devices with matching "USB Product Name" or 
/// "USB Vendor Name" properties.
/// 
/// # Parameters
/// - `device_type`: The device type to search for (e.g., "AX88179", "Logitech", "Apple")
/// 
/// # Returns
/// - `Ok(Vec<String>)`: A vector of strings containing information about matching devices
/// - `Err`: If there's an error executing the system command
/// 
/// # Platform Notes
/// - On Linux, requires `lsusb` to be installed (usually part of usbutils package)
/// - On macOS, uses the built-in `ioreg` command
/// - On other platforms, returns an error indicating lack of support
/// 
/// # Example
/// ```rust
/// use acovo::dev::FindUsbDevicesByType;
/// 
/// #[cfg(feature = "dev")]
/// match FindUsbDevicesByType("AX88179") {
///     Ok(devices) => {
///         if devices.is_empty() {
///             println!("No devices of type 'AX88179' found");
///         } else {
///             println!("Found {} devices of type 'AX88179':", devices.len());
///             for device in devices {
///                 println!("  {}", device);
///             }
///         }
///     },
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[cfg(feature = "dev")]
pub fn FindUsbDevicesByType(device_type: &str) -> AnyResult<Vec<String>> {
    use std::process::Command;
    use anyhow::{anyhow, Result as AnyResult};
    
    // Determine which command to use based on the operating system
    if cfg!(target_os = "linux") {
        // Execute the lsusb command on Linux
        match Command::new("/bin/lsusb").output() {
            Ok(output) => {
                // Convert the command output to a UTF-8 string
                let sOutput = String::from_utf8(output.stdout)?;
                
                // Check if the output is empty (indicating an error)
                if sOutput.len() == 0 {
                    // If stdout is empty, check stderr for error information
                    let sErr = String::from_utf8(output.stderr)?;
                    return Err(anyhow!("find-usb-devices-by-type-error {}", sErr));
                }
                
                // Split the output into individual device entries (one per line)
                let usblist = sOutput.split("\n");
                let mut matching_devices = Vec::new();
                
                // Iterate through each device entry to find matches
                for dev in usblist {
                    // Check if this device entry contains our target device type
                    if dev.to_lowercase().contains(&device_type.to_lowercase()) {
                        matching_devices.push(dev.to_string());
                    }
                }
                
                Ok(matching_devices)
            }
            // Handle errors from executing the lsusb command
            Err(e) => {
                Err(anyhow!("Failed to execute lsusb command: {}. Check that lsusb is installed (usually part of usbutils package).", e))
            }
        }
    } else if cfg!(target_os = "macos") {
        // Execute the ioreg command on macOS with detailed output
        match Command::new("/usr/sbin/ioreg").args(["-p", "IOUSB", "-w", "0", "-l"]).output() {
            Ok(output) => {
                // Convert the command output to a UTF-8 string
                let sOutput = String::from_utf8(output.stdout)?;
                
                // Check if the output is empty (indicating an error)
                if sOutput.len() == 0 {
                    // If stdout is empty, check stderr for error information
                    let sErr = String::from_utf8(output.stderr)?;
                    return Err(anyhow!("find-usb-devices-by-type-error {}", sErr));
                }
                
                // Split the output into lines for easier processing
                let lines: Vec<&str> = sOutput.lines().collect();
                let mut matching_devices = Vec::new();
                let device_type_lower = device_type.to_lowercase();
                
                // Search for devices with matching product or vendor names
                for i in 0..lines.len() {
                    let line = lines[i];
                    
                    // Look for "USB Product Name" or "USB Vendor Name" properties
                    if line.contains("\"USB Product Name\"") || line.contains("\"USB Vendor Name\"") {
                        // Check if the line contains our target device type
                        if line.to_lowercase().contains(&device_type_lower) {
                            // Found a matching device, collect information about it
                            let start = i.saturating_sub(10); // Look up to 10 lines before
                            let end = std::cmp::min(i + 20, lines.len()); // Look up to 20 lines after
                            
                            // Collect device information
                            let mut device_info = String::new();
                            for j in start..end {
                                if lines[j].trim_start().starts_with("}") {
                                    // End of device block
                                    break;
                                }
                                device_info.push_str(lines[j]);
                                device_info.push('\n');
                            }
                            
                            matching_devices.push(device_info);
                        }
                    }
                }
                
                Ok(matching_devices)
            }
            // Handle errors from executing the ioreg command
            Err(e) => {
                Err(anyhow!("Failed to execute ioreg command: {}. Check that ioreg is available on this system.", e))
            }
        }
    } else {
        // Unsupported platform
        Err(anyhow!("Finding USB devices by type is not supported on this platform ({}). Only Linux and macOS are supported.", std::env::consts::OS))
    }
}

#[cfg(feature = "dev")]
/// List all USB devices on Linux/macOS systems.
/// 
/// This function uses different system utilities depending on the platform:
/// - Linux: Uses `lsusb` command
/// - macOS: Uses `ioreg` command
/// 
/// # Returns
/// - `Ok(String)`: Contains the raw output of the system command
/// - `Err`: If there's an error executing the system command
/// 
/// # Platform Notes
/// - On Linux, requires `lsusb` to be installed (usually part of usbutils package)
/// - On macOS, uses the built-in `ioreg` command
/// - On other platforms, returns an error indicating lack of support
/// 
/// # Example
/// ```rust
/// use acovo::dev::ListUsbDevices;
/// 
/// #[cfg(feature = "dev")]
/// match ListUsbDevices() {
///     Ok(devices) => println!("Connected USB devices:\n{}", devices),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
#[cfg(feature = "dev")]
pub fn ListUsbDevices() -> AnyResult<String> {
    use std::process::Command;
    use anyhow::{anyhow, Result as AnyResult};
    
    // Determine which command to use based on the operating system
    if cfg!(target_os = "linux") {
        // Execute the lsusb command on Linux
        match Command::new("/bin/lsusb").output() {
            Ok(output) => {
                // Convert the command output to a UTF-8 string
                let sOutput = String::from_utf8(output.stdout)?;
                
                // Check if the output is empty (indicating an error)
                if sOutput.len() == 0 {
                    // If stdout is empty, check stderr for error information
                    let sErr = String::from_utf8(output.stderr)?;
                    return Err(anyhow!("list-usb-devices-error {}", sErr));
                }
                
                Ok(sOutput)
            }
            // Handle errors from executing the lsusb command
            Err(e) => {
                Err(anyhow!("Failed to execute lsusb command: {}. Check that lsusb is installed (usually part of usbutils package).", e))
            }
        }
    } else if cfg!(target_os = "macos") {
        // Execute the ioreg command on macOS
        match Command::new("/usr/sbin/ioreg").args(["-p", "IOUSB"]).output() {
            Ok(output) => {
                // Convert the command output to a UTF-8 string
                let sOutput = String::from_utf8(output.stdout)?;
                
                // Check if the output is empty (indicating an error)
                if sOutput.len() == 0 {
                    // If stdout is empty, check stderr for error information
                    let sErr = String::from_utf8(output.stderr)?;
                    return Err(anyhow!("list-usb-devices-error {}", sErr));
                }
                
                Ok(sOutput)
            }
            // Handle errors from executing the ioreg command
            Err(e) => {
                Err(anyhow!("Failed to execute ioreg command: {}. Check that ioreg is available on this system.", e))
            }
        }
    } else {
        // Unsupported platform
        Err(anyhow!("Listing USB devices is not supported on this platform ({}). Only Linux and macOS are supported.", std::env::consts::OS))
    }
}

#[cfg(test)]
#[cfg(feature = "dev")]
mod tests {
    use super::*;

    #[test]
    fn test_find_usb_device() {
        // Test with a device that definitely doesn't exist
        let result = LinuxFindUsbDevice("ffff", "ffff");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
        println!("✓ Successfully tested non-existent device (ffff:ffff) - correctly returned false");

        // Test with Apple's vendor ID (common on macOS)
        // This test might pass or fail depending on whether an Apple USB device is connected
        let result = LinuxFindUsbDevice("05ac", "");
        assert!(result.is_ok());
        println!("✓ Successfully tested Apple vendor ID (05ac) - result: {}", result.unwrap());
        
        // Test with the ASIX AX88179 USB-to-Ethernet adapter that we know exists
        // Vendor ID: 0xb95 (2965 in decimal), Product ID: 0x1790 (6032 in decimal)
        let result = LinuxFindUsbDevice("0b95", "1790");
        assert!(result.is_ok());
        println!("✓ Successfully tested ASIX AX88179 device (0b95:1790) - result: {}", result.unwrap());
        
        // Test with just the vendor ID to see if any ASIX devices are present
        let result = LinuxFindUsbDevice("0b95", "");
        assert!(result.is_ok());
        println!("✓ Successfully tested for ASIX devices (0b95) - result: {}", result.unwrap());
        
        // Comprehensive tests for all detected USB devices on the system
        // Test Apple devices (Vendor ID: 0x05ac = 1452)
        let result = LinuxFindUsbDevice("05ac", "");
        assert!(result.is_ok());
        println!("✓ Tested Apple devices (05ac) - result: {}", result.unwrap());
        
        // Test GenesysLogic devices (Vendor ID: 0x05e3 = 1507)
        let result = LinuxFindUsbDevice("05e3", "");
        assert!(result.is_ok());
        println!("✓ Tested GenesysLogic devices (05e3) - result: {}", result.unwrap());
        
        // Test MACROSILICON devices (Vendor ID: 0x2b89 = 11145)
        let result = LinuxFindUsbDevice("2b89", "");
        assert!(result.is_ok());
        println!("✓ Tested MACROSILICON devices (2b89) - result: {}", result.unwrap());
        
        // Test ASIX Elec. Corp. devices (Vendor ID: 0x0b95 = 2965)
        let result = LinuxFindUsbDevice("0b95", "");
        assert!(result.is_ok());
        println!("✓ Tested ASIX Elec. Corp. devices (0b95) - result: {}", result.unwrap());
        
        // Test Logitech devices (Vendor ID: 0x046d = 1133)
        let result = LinuxFindUsbDevice("046d", "");
        assert!(result.is_ok());
        println!("✓ Tested Logitech devices (046d) - result: {}", result.unwrap());
        
        // Test Razer devices (Vendor ID: 0x1532 = 5426)
        let result = LinuxFindUsbDevice("1532", "");
        assert!(result.is_ok());
        println!("✓ Tested Razer devices (1532) - result: {}", result.unwrap());
        
        // Specific device tests
        // Test specific Apple USB3 Gen2 Hub (Vendor: 0x05ac, Product: 0x800c)
        let result = LinuxFindUsbDevice("05ac", "800c");
        assert!(result.is_ok());
        println!("✓ Tested Apple USB3 Gen2 Hub (05ac:800c) - result: {}", result.unwrap());
        
        // Test specific UGREEN 35287 (Vendor: 0x2b89, Product: 0x5287)
        let result = LinuxFindUsbDevice("2b89", "5287");
        assert!(result.is_ok());
        println!("✓ Tested UGREEN 35287 (2b89:5287) - result: {}", result.unwrap());
        
        // Test specific AX88179 (Vendor: 0x0b95, Product: 0x1790)
        let result = LinuxFindUsbDevice("0b95", "1790");
        assert!(result.is_ok());
        println!("✓ Tested AX88179 (0b95:1790) - result: {}", result.unwrap());
        
        // Test specific USB Optical Mouse (Vendor: 0x046d, Product: 0xc077)
        let result = LinuxFindUsbDevice("046d", "c077");
        assert!(result.is_ok());
        println!("✓ Tested USB Optical Mouse (046d:c077) - result: {}", result.unwrap());
        
        // Test specific Razer Cynosa Pro (Vendor: 0x1532, Product: 0x020d)
        let result = LinuxFindUsbDevice("1532", "020d");
        assert!(result.is_ok());
        println!("✓ Tested Razer Cynosa Pro (1532:020d) - result: {}", result.unwrap());
        
        // Note: We don't assert a specific result here since it depends on the hardware
        // connected to the test machine
    }
    
    #[test]
    fn test_list_usb_devices() {
        // Test that the function returns successfully
        let result = ListUsbDevices();
        assert!(result.is_ok());
        
        // Verify that we get some output (at least some characters)
        let devices = result.unwrap();
        assert!(!devices.is_empty());
        println!("✓ Successfully listed USB devices. Output length: {} characters", devices.len());
        println!("First 500 characters of output:\n{}", &devices[..std::cmp::min(500, devices.len())]);
        
        // On macOS, verify that the output contains expected ioreg formatting
        if cfg!(target_os = "macos") {
            assert!(devices.contains("IOUSB"));
            println!("✓ Verified macOS ioreg output format (contains 'IOUSB')");
        }
        
        // On Linux, verify that the output contains expected lsusb formatting
        if cfg!(target_os = "linux") {
            // lsusb typically outputs lines with "Bus XXX Device YYY:"
            assert!(devices.contains("Bus") && devices.contains("Device"));
            println!("✓ Verified Linux lsusb output format (contains 'Bus' and 'Device')");
        }
    }
    
    #[cfg(feature = "dev")]
    #[test]
    fn test_find_usb_devices_by_type() {
        // Test that the function runs without panicking
        let result = FindUsbDevicesByType("Apple");
        assert!(result.is_ok(), "FindUsbDevicesByType should not panic");
        
        // Test with a non-existent device type
        let result = FindUsbDevicesByType("NonExistentDeviceType12345");
        assert!(result.is_ok(), "FindUsbDevicesByType should handle non-existent device types gracefully");
    }
}