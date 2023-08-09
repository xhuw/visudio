use std::io::Write;

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device,
};

fn get_index_from_user(max: usize) -> std::io::Result<usize> {
    let mut user_input = String::new();
    loop {
        print!("Select microphone by index (0 - {}): ", max);
        std::io::stdout().flush()?;

        std::io::stdin().read_line(&mut user_input)?;
        if let Ok(index) = user_input.trim().parse::<usize>() {
            if index < max {
                return Ok(index);
            }
        }
        user_input.clear();
    }
}

fn run_app(device: &Device) {}

fn main() -> Result<(), &'static str> {
    let host = cpal::default_host();

    if let Ok(devices) = host.input_devices() {
        let devices: Vec<_> = devices.collect();
        let indices = (0..).map(|i| i.to_string());
        let device_names = devices.iter().map(|d| d.name().unwrap_or("".to_string()));
        for (i, name) in std::iter::zip(indices, device_names) {
            println!("{i} - {name}")
        }

        println!("");
        let selected =
            get_index_from_user(devices.len() - 1).map_err(|_| "Failed to read from stdin")?;
        run_app(&devices[selected]);
    } else {
        return Err("This computer has no supported audio hosts");
    }

    Ok(())
}
