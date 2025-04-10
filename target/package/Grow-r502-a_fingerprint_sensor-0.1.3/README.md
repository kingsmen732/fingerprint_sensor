# Fingerprint-lib

A Rust library for interfacing with R502-A and compatible fingerprint sensors via serial communication.

RAW implementation of this project is available in:
- RUST
- PYTHON
- CPP 

[ ⭐ Link to the Project](https://github.com/kingsmen732/GROW_R502-A_fingerprint) 

If link not redirecting use :
https://github.com/kingsmen732/GROW_R502-A_fingerprint

This crate enables enrolling, verifying, listing, and deleting fingerprints using the sensor's built-in flash memory.

## ✨ Features

- Enroll fingerprints directly to the sensor's memory.
- Verify fingerprints against stored templates.
- Retrieve a list of all enrolled fingerprint IDs.
- Remove specific fingerprint entries.
- Supports [R502-A] and other UART-based fingerprint sensors (e.g., FPM10A, GT-521F52).

---

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fingerprint-lib = "0.1.0"
serialport = "4.2"  # Required for serial port communication
```

## 🚀 Usage

Here's an example of how to use the library:

```rust
use fingerprint_lib::fingerprint;
use std::time::Duration;

fn main() {
    let port_name = "/dev/ttyUSB0"; // Replace with your serial port
    let mut port = serialport::new(port_name, 57600)
        .timeout(Duration::from_secs(2))
        .open()
        .expect("Failed to open serial port");

    // Enroll a new fingerprint with ID 5
    fingerprint::enroll_fingerprint(&mut *port, 5).unwrap();

    // Verify a fingerprint
    fingerprint::verify_fingerprint(&mut *port).unwrap();

    // List all stored fingerprint IDs
    fingerprint::list_fingerprints(&mut *port).unwrap();

    // Delete a fingerprint with ID 5
    fingerprint::delete_fingerprint(&mut *port, 5).unwrap();
}
```
---
## 📜 Research colab in AIR CENTER 

Research outcome supported by <b> Indominus labs Private Limited </b > and <b> Digital Fortress Private Limited </b>

---

## 💡 Sensor Compatibility

This crate is designed for fingerprint modules that communicate over UART and adhere to the GT-511C3 / R30x / R502-A packet protocol.

### Tested Modules

- **R502-A (GT series)**
- Other compatible modules using Adafruit's fingerprint protocol.

---

## 📜 License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## 👋 Contributions

Contributions are welcome! Feel free to open pull requests or suggest new features.

---

Let me know if you'd like me to auto-generate a `Cargo.toml` for publishing!