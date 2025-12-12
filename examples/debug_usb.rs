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

    // Test case 1: Try to find a device that likely exists (Apple vendor ID)
    println!("\nTest Case 1: Searching for Apple devices (Vendor ID: 05ac)");
    println!("-------------------------------------------------------");
    match acovo::dev::LinuxFindUsbDevice("05ac", "") {
        Ok(found) => {
            if found {
                println!("✓ Apple device found!");
            } else {
                println!("✗ No Apple device found with the specified criteria.");
            }
        }
        Err(e) => {
            eprintln!("Error occurred during search: {}", e);
        }
    }

    // Test case 2: Try to find a device that definitely doesn't exist
    println!("\nTest Case 2: Searching for non-existent device (Vendor ID: ffff, Product ID: ffff)");
    println!("----------------------------------------------------------------------------------");
    match acovo::dev::LinuxFindUsbDevice("ffff", "ffff") {
        Ok(found) => {
            if found {
                println!("✓ Unexpectedly found the non-existent device!");
            } else {
                println!("✓ Correctly determined that the device doesn't exist.");
            }
        }
        Err(e) => {
            eprintln!("Error occurred during search: {}", e);
        }
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