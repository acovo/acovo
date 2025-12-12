//! Example to show debug output for USB device detection on macOS
//!
//! This example demonstrates how to use the LinuxFindUsbDevice and ListUsbDevices functions
//! to detect and list USB devices on macOS systems.

use tracing_subscriber;

fn main() {
    // Initialize the logger to show debug output
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("========================================");
    println!("Testing USB device detection on macOS...");
    println!("========================================");
    
    // List all USB devices using the new ListUsbDevices function
    match acovo::dev::ListUsbDevices() {
        Ok(devices) => {
            println!("All connected USB devices:");
            println!("----------------------------------------");
            println!("{}", devices);
            println!("----------------------------------------");
        }
        Err(e) => {
            eprintln!("Failed to list USB devices: {}", e);
        }
    }

    // Test with a known Apple device (VID: 05ac)
    match acovo::dev::LinuxFindUsbDevice("05ac", "") {
        Ok(found) => println!("\n✓ Successfully determined that device with VID 05ac {}exist", if found { "" } else { "does not " }),
        Err(e) => println!("\n✗ Error checking device with VID 05ac: {}", e),
    }

    // Test with a non-existent device (VID: ffff, PID: ffff)
    match acovo::dev::LinuxFindUsbDevice("ffff", "ffff") {
        Ok(found) => println!("\n✓ Successfully determined that device ffff:ffff {}exist", if found { "" } else { "does not " }),
        Err(e) => println!("\n✗ Error checking device ffff:ffff: {}", e),
    }
    
    // Demonstrate the new FindUsbDevicesByType function
    println!("\n=== Testing FindUsbDevicesByType function ===");
    
    // Try to find devices by a common device type
    match acovo::dev::FindUsbDevicesByType("Apple") {
        Ok(devices) => {
            if devices.is_empty() {
                println!("\n✓ FindUsbDevicesByType('Apple') completed successfully, no Apple devices found");
            } else {
                println!("\n✓ FindUsbDevicesByType('Apple') found {} device(s):", devices.len());
                for (i, device) in devices.iter().enumerate() {
                    println!("  Device {}: {}", i + 1, device.lines().next().unwrap_or("Unknown device"));
                }
            }
        },
        Err(e) => println!("\n✗ Error in FindUsbDevicesByType('Apple'): {}", e),
    }
    
    // Try with another device type
    match acovo::dev::FindUsbDevicesByType("AX88179") {
        Ok(devices) => {
            if devices.is_empty() {
                println!("\n✓ FindUsbDevicesByType('AX88179') completed successfully, no AX88179 devices found");
            } else {
                println!("\n✓ FindUsbDevicesByType('AX88179') found {} device(s):", devices.len());
                for (i, device) in devices.iter().enumerate() {
                    println!("  Device {}: {}", i + 1, device.lines().next().unwrap_or("Unknown device"));
                }
            }
        },
        Err(e) => println!("\n✗ Error in FindUsbDevicesByType('AX88179'): {}", e),
    }

    // Additional information section
    println!("\n========================================");
    println!("Additional Information:");
    println!("========================================");
    println!("This debug example demonstrates:");
    println!("1. How the ListUsbDevices function works internally on macOS");
    println!("2. How the LinuxFindUsbDevice function works internally on macOS");
    println!("3. That ListUsbDevices uses the 'ioreg -p IOUSB' command to enumerate USB devices");
    println!("4. That LinuxFindUsbDevice searches for devices by VID:PID in the ioreg output");
    println!("5. Proper error handling when executing system commands");
    println!("6. How to interpret the results of USB device detection");
    println!("\nThe ioreg output shows the complete USB device hierarchy,");
    println!("which is parsed by the LinuxFindUsbDevice function to find");
    println!("specific devices by their vendor and product IDs.");
}