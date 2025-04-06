use fingerprint_lib::fingerprint;
use std::time::Duration;

fn main() {
    let port_name = "/dev/ttyUSB0"; // Update based on the actual device
    let mut port = serialport::new(port_name, 57600)
        .timeout(Duration::from_secs(2))
        .open()
        .expect("Failed to open serial port");

    println!("Connected to fingerprint sensor.");

    // Example: Enroll fingerprint to ID 5
    fingerprint::enroll_fingerprint(&mut *port, 5).unwrap();

    // Example: Verify fingerprint
    fingerprint::verify_fingerprint(&mut *port).unwrap();

    // Example: List all enrolled IDs
    fingerprint::list_fingerprints(&mut *port).unwrap();

    // Example: Delete fingerprint ID 5
    fingerprint::delete_fingerprint(&mut *port, 5).unwrap();
}
