mod ft4222;

fn main() -> anyhow::Result<()> {
    let devices = ft4222::list_devices()?;
    if let Some(device) = devices.first() {
        let mut handle = device.open_spi(
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
        loop {
            if last_call.elapsed() < std::time::Duration::from_secs(1) {
                std::thread::sleep(std::time::Duration::from_secs(1) - last_call.elapsed());
            }
            last_call = std::time::Instant::now();
            let bytes_transferred = handle.read_write(&mut write_buffer, &mut read_buffer)?;
            let message: String = read_buffer[0..bytes_transferred as usize]
                .iter()
                .filter(|value| **value < 128)
                .map(|value| char::from(*value))
                .collect();
            println!("{}", message);
        }
    }
    Ok(())
}
