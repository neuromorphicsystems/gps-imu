mod ft4222;

use std::io::Write;

fn main() -> anyhow::Result<()> {
    let serial_to_device = ft4222::list_devices()?;
    if serial_to_device.is_empty() {
        return Err(anyhow::anyhow!("No devices found"));
    }
    let mut selected_device = None;
    for (serial, device) in serial_to_device.iter() {
        if selected_device.is_none() {
            println!("Selected device {}", serial.to_string());
            selected_device = Some(device);
        } else {
            println!(
                "More than one devices detected, ignoring {}",
                serial.to_string()
            );
        }
    }
    if let Some(device) = selected_device {
        let mut gps_handle = device.open_spi(
            0,
            ft4222::ClockRate::EightyMHz,
            ft4222::ClockDivider::Sixteen,
            ft4222::Active::Low,
            ft4222::ClockPhase::LeadingEdge,
            ft4222::OutputMap {
                enable_0: true,
                enable_1: false,
                enable_2: false,
                enable_3: false,
            },
            std::time::Duration::from_millis(100),
            std::time::Duration::from_millis(100),
        )?;
        let mut last_call = std::time::Instant::now();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let mut write_buffer = [0xff; 2048];
        let mut read_buffer = [0x00; 2048];
        let mut log_file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("gps.nmea")?;
        loop {
            if last_call.elapsed() < std::time::Duration::from_secs(1) {
                std::thread::sleep(std::time::Duration::from_secs(1) - last_call.elapsed());
            }
            last_call = std::time::Instant::now();
            let bytes_transferred = gps_handle.read_write(&mut write_buffer, &mut read_buffer)?;
            let unix_timestamp = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap_or(std::time::Duration::new(0, 0))
                .as_micros() as u64;
            let message: String = read_buffer[0..bytes_transferred as usize]
                .iter()
                .filter(|value| **value < 128)
                .map(|value| char::from(*value))
                .collect();
            println!("unix_us={}\n{}", unix_timestamp, message);
            writeln!(log_file, "unix_us={}\n{}", unix_timestamp, message)?;
        }
    } else {
        unreachable!();
    }
}
