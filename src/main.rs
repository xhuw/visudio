use cpal::traits::{HostTrait, DeviceTrait};
use cpal::ALL_HOSTS;
fn main() {

    let host = cpal::default_host();
    println!("{:?}", host.id());

    if let Ok(devices) = host.input_devices() {
        for d in devices {
            println!("{:?}", d.name());
        }
    }
    println!("{:?}", ALL_HOSTS);

    println!("Hello, world!");
}
