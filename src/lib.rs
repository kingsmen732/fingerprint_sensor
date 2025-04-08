use serialport;
// use std::io::{Read, Write};
use std::time::Duration;

pub mod fingerprint {
    use super::*;

    pub fn send_packet(port: &mut dyn serialport::SerialPort, payload: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut packet: Vec<u8> = vec![
            0xEF, 0x01,                         // Packet Header
            0xFF, 0xFF, 0xFF, 0xFF,             // Module Address (default)
            0x01,                               // Packet Identifier: Command Packet
            ((payload.len() + 2) >> 8) as u8,   // Length High
            ((payload.len() + 2) & 0xFF) as u8, // Length Low
        ];

        packet.extend_from_slice(payload);

        let checksum: u16 = packet[6..].iter().map(|&b| b as u16).sum();
        packet.push((checksum >> 8) as u8);    // Checksum high
        packet.push((checksum & 0xFF) as u8);  // Checksum low

        port.write_all(&packet)?;

        let mut buf = [0u8; 64];
        let n = port.read(&mut buf)?;
        Ok(buf[..n].to_vec())
    }

    pub fn enroll_fingerprint(port: &mut dyn serialport::SerialPort, id: u16) -> std::io::Result<()> {
        println!("Place finger on sensor...");

        loop {
            let resp = send_packet(port, &[0x01])?; // GetImage
            if resp.get(9) == Some(&0x00) {
                break;
            }
        }

        println!("Image taken. Converting...");
        let conv1 = send_packet(port, &[0x02, 0x01])?; // image2Tz buffer 1
        if conv1.get(9) != Some(&0x00) {
            println!("Failed to convert first image.");
            return Ok(());
        }

        println!("Remove finger...");
        std::thread::sleep(Duration::from_secs(2));

        loop {
            let resp = send_packet(port, &[0x01])?;
            if resp.get(9) == Some(&0x02) {
                break;
            }
        }

        println!("Place same finger again...");
        loop {
            let resp = send_packet(port, &[0x01])?;
            if resp.get(9) == Some(&0x00) {
                break;
            }
        }

        let conv2 = send_packet(port, &[0x02, 0x02])?;
        if conv2.get(9) != Some(&0x00) {
            println!("Failed to convert second image.");
            return Ok(());
        }

        let model = send_packet(port, &[0x05])?;
        if model.get(9) != Some(&0x00) {
            println!("Fingerprints did not match.");
            return Ok(());
        }

        let id_high = (id >> 8) as u8;
        let id_low = (id & 0xFF) as u8;
        let store = send_packet(port, &[0x06, 0x01, id_high, id_low])?;
        if store.get(9) == Some(&0x00) {
            println!("Fingerprint stored at ID {}", id);
        } else {
            println!("Failed to store fingerprint.");
        }

        Ok(())
    }

    pub fn verify_fingerprint(port: &mut dyn serialport::SerialPort) -> std::io::Result<()> {
        println!("Waiting for finger...");

        loop {
            let resp = send_packet(port, &[0x01])?;
            if resp.get(9) == Some(&0x00) {
                break;
            }
        }

        let conv = send_packet(port, &[0x02, 0x01])?;
        if conv.get(9) != Some(&0x00) {
            println!("Failed to convert image.");
            return Ok(());
        }

        let search = send_packet(port, &[0x04, 0x01, 0x00, 0x00, 0x00, 0xFF])?;
        if search.len() >= 14 && search[9] == 0x00 {
            let id = ((search[10] as u16) << 8) | search[11] as u16;
            let score = ((search[12] as u16) << 8) | search[13] as u16;
            println!("Match found! ID: {}, Score: {}", id, score);
        } else {
            println!("No match found.");
        }

        Ok(())
    }

    pub fn list_fingerprints(port: &mut dyn serialport::SerialPort) -> std::io::Result<()> {
        println!("Checking stored fingerprints...");

        let mut used_ids = Vec::new();

        for page in 0..=7 {
            let resp = send_packet(port, &[0x1F, page])?;

            if resp.get(9) != Some(&0x00) {
                println!("Failed to retrieve template index for page {}", page);
                continue;
            }

            if resp.len() >= 42 {
                for (i, byte) in resp[10..42].iter().enumerate() {
                    for bit in 0..8 {
                        if byte & (1 << bit) != 0 {
                            let id = page as u16 * 16 + (i as u16 * 8 + bit);
                            if id < 128 {
                                used_ids.push(id);
                            }
                        }
                    }
                }
            }
        }

        if used_ids.is_empty() {
            println!("No fingerprints stored.");
        } else {
            println!("Stored fingerprint IDs: {:?}", used_ids);
        }

        Ok(())
    }

    pub fn delete_fingerprint(port: &mut dyn serialport::SerialPort, id: u16) -> std::io::Result<()> {
        println!("Attempting to delete fingerprint with ID {}...", id);

        let id_high = (id >> 8) as u8;
        let id_low = (id & 0xFF) as u8;
        let resp = send_packet(port, &[0x0C, id_high, id_low, 0x00, 0x01])?;

        if resp.get(9) == Some(&0x00) {
            println!("Successfully deleted fingerprint ID {}.", id);
        } else {
            println!("Failed to delete fingerprint ID {}.", id);
        }

        Ok(())
    }
}
