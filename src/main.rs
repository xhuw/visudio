use std::io::Write;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, SampleFormat, Stream, SupportedStreamConfig,
};

use anyhow::anyhow;

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

fn build_stream(device: &Device, config: SupportedStreamConfig) -> Result<Stream, anyhow::Error> {
    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };
    let d = match config.sample_format() {
        SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            |data: &[i8], _: &_| dump(data[0]),
            err_fn,
            None,
        )?,
        SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            |data: &[i16], _: &_| dump(data[0]),
            err_fn,
            None,
        )?,
        SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            |data: &[i32], _: &_| dump(data[0]),
            err_fn,
            None,
        )?,
        SampleFormat::I64 => device.build_input_stream(
            &config.into(),
            |data: &[i64], _: &_| dump(data[0]),
            err_fn,
            None,
        )?,
        SampleFormat::U8 => device.build_input_stream(
            &config.into(),
            |data: &[u8], _: &_| dump(data[0]),
            err_fn,
            None,
        )?,
        SampleFormat::U16 => device.build_input_stream(
            &config.into(),
            |data: &[u16], _: &_| dump(data[0]),
            err_fn,
            None,
        )?,
        SampleFormat::U32 => device.build_input_stream(
            &config.into(),
            |data: &[u32], _: &_| dump(data[0]),
            err_fn,
            None,
        )?,
        SampleFormat::U64 => device.build_input_stream(
            &config.into(),
            |data: &[u64], _: &_| dump(data[0]),
            err_fn,
            None,
        )?,
        SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            |data: &[f32], _: &_| dump(data[0]),
            err_fn,
            None,
        )?,
        SampleFormat::F64 => device.build_input_stream(
            &config.into(),
            |data: &[f64], _: &_| dump(data[0]),
            err_fn,
            None,
        )?,
        _ => return Err(anyhow!("Unsupported format")),
    };
    Ok(d)
}

fn run_app(device: &Device) -> Result<(), anyhow::Error> {
    // TODO select sample rate
    let config = device.default_input_config()?;
    println!("using default sample rate: {}", config.sample_rate().0);
    let stream = build_stream(device, config)?;
    let _ = stream.play().unwrap();
    loop {}
}

fn main() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();

    let devices = host.input_devices()?;
    let device_names: Vec<_> = devices
        .map(|d| d.name().unwrap_or("".to_string()))
        .collect();
    for (i, name) in device_names.iter().enumerate() {
        println!("{i} - {name}")
    }

    println!("");
    let selected = get_index_from_user(device_names.len() - 1)?;

    if let Some(device) = host.input_devices()?.find(|d| {
        d.name()
            .map(|y| y == device_names[selected])
            .unwrap_or(false)
    }) {
        run_app(&device)?;
    }

    Ok(())
}
