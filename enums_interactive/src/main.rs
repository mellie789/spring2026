/*
Create an interactive Rust program that performs basic file operations 
(ls, cat, create, remove, pwd) by executing system commands using Command::new(). 
Use enums to represent different file operations. 
The program should accept user input via a menu system until the user decides to exit.

Your program should:
Run in a loop, continuously displaying a menu of options to the user.
Allow the user to select an operation by entering a number.
Based on the selection, prompt for additional arguments if necessary.
Map the user input to the corresponding FileOperation variant.
Call the function to perform the operation.
Display the output or any messages to the user.
Handle minimal error cases (e.g., invalid menu option).

Write a function perform_operation that takes a FileOperation 
and performs the corresponding system command using Command::new().


*/

enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

use std::process::Command;

fn perform_operation(operation: FileOperation) {
    // Implement command execution based on the operation
}