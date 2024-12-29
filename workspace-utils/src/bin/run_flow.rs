use std::process::{Command, Stdio};
use std::{thread, time};

fn main() {
    // Start the dispatcher as a background process
    println!("Starting dispatcher...");
    let mut dispatcher = Command::new("cargo")
        .args(&["run", "--package", "ro-dispatcher"])
        .stdout(Stdio::null()) // Redirect output to null or use Stdio::inherit() for logging
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start dispatcher");

    // Wait for the dispatcher to initialize
    let delay = time::Duration::from_secs(2);
    println!("Waiting for dispatcher to initialize...");
    thread::sleep(delay);

    // Run the client in the foreground
    println!("Starting client...");
    let client_status = Command::new("cargo")
        .args(&["run", "--package", "ro-client"])
        .status()
        .expect("Failed to start client");

    // Report client exit status
    if client_status.success() {
        println!("Client exited successfully.");
    } else {
        eprintln!("Client exited with errors.");
    }

    // Kill the dispatcher process if it is still running
    println!("Stopping dispatcher...");
    if let Err(err) = dispatcher.kill() {
        eprintln!("Failed to stop dispatcher: {}", err);
    }
}
