use std::io::Write;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, SupportedStreamConfig, Stream, SampleFormat,
};

fn get_index_from_user(max: usize) -> std::io::Result<usize> {
    let mut user_input = String::new();
    loop {
        print!("Select microphone by index (0 - {}): ", max);
        std::io::stdout().flush()?;

        std::io::stdin().read_line(&mut user_input)?;
        if let Ok(index) = user_input.trim().parse::<usize>() {
            if index <= max {
                return Ok(index);
            }
        }
        user_input.clear();
    }
}

fn dump<T: std::fmt::Display>(val: T) {
    print!("\r{val}");
    std::io::stdout().flush().unwrap();
}

fn build_stream(device: &Device, config: SupportedStreamConfig) -> Result<Stream, &'static str> {
    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };
    match config.sample_format() {
        SampleFormat::I8 => device.build_input_stream(&config.into(), |data: &[i8], _: &_| dump(data[0]), err_fn, None).map_err(|_| "failed to create stream"),
        SampleFormat::I16 => device.build_input_stream(&config.into(), |data: &[i16], _: &_| dump(data[0]), err_fn, None).map_err(|_| "failed to create stream"),
        SampleFormat::I32 => device.build_input_stream(&config.into(), |data: &[i32], _: &_| dump(data[0]), err_fn, None).map_err(|_| "failed to create stream"),
        SampleFormat::I64 => device.build_input_stream(&config.into(), |data: &[i64], _: &_| dump(data[0]), err_fn, None).map_err(|_| "failed to create stream"),
        SampleFormat::U8 => device.build_input_stream(&config.into(), |data: &[u8], _: &_| dump(data[0]), err_fn, None).map_err(|_| "failed to create stream"),
        SampleFormat::U16 => device.build_input_stream(&config.into(), |data: &[u16], _: &_| dump(data[0]), err_fn, None).map_err(|_| "failed to create stream"),
        SampleFormat::U32 => device.build_input_stream(&config.into(), |data: &[u32], _: &_| dump(data[0]), err_fn, None).map_err(|_| "failed to create stream"),
        SampleFormat::U64 => device.build_input_stream(&config.into(), |data: &[u64], _: &_| dump(data[0]), err_fn, None).map_err(|_| "failed to create stream"),
        SampleFormat::F32 => device.build_input_stream(&config.into(), |data: &[f32], _: &_| dump(data[0]), err_fn, None).map_err(|_| "failed to create stream"),
        SampleFormat::F64 => device.build_input_stream(&config.into(), |data: &[f64], _: &_| dump(data[0]), err_fn, None).map_err(|_| "failed to create stream"),
        _ => Err("Unsupported format"),
        
    }
}

fn run_app(device: &Device) -> Result<(), &'static str>{
    // TODO select sample rate
    let config = device.default_input_config().map_err(|_|"Failed to get device config")?;
    let stream = build_stream(device, config)?;
    let _ = stream.play().unwrap();
    loop{}
}

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
        run_app(&devices[selected])?;
    } else {
        return Err("This computer has no supported audio hosts");
    }

    Ok(())
}