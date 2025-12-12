use acovo::fs::*;
use std::fs;

fn main() {
    let test_dir = "/tmp/acovo_debug_test";
    mkdir(test_dir).expect("Failed to create test directory");
    
    let mixed_endings_file = format!("{}/mixed_endings.txt", test_dir);
    let mixed_lines = vec![
        "Line 1\n".to_string(),  // Contains \n
        "Line 2\r\n".to_string(), // Contains \r\n
        "Line 3\r".to_string(),   // Contains \r
        "Line 4".to_string()      // No line ending
    ];
    
    let result = write_lines(mixed_endings_file.clone(), mixed_lines, true);
    println!("Write result: {:?}", result);
    
    // Read raw content
    let raw_content = fs::read_to_string(&mixed_endings_file).unwrap();
    println!("Raw file content:");
    println!("{:?}", raw_content);
    println!("Raw content length: {}", raw_content.len());
    
    // Read with read_lines
    if let Ok(read_lines) = read_lines(&mixed_endings_file) {
        let lines: Vec<_> = read_lines.map(|line| line.unwrap()).collect();
        println!("Lines read by read_lines:");
        for (i, line) in lines.iter().enumerate() {
            println!("Line {}: {:?}", i, line);
        }
        println!("Total lines: {}", lines.len());
    }
}