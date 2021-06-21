extern crate portmidi as pm;
use pm::MidiMessage;

fn main() {
    let ctx = match pm::PortMidi::new() {
        Ok(ctx) =>  ctx,
        Err(_) => panic!("Couldn't create portmidi context")
    };

    for dev in ctx.devices().unwrap() {
        println!("{}", dev);
    }

    let mut device_str = String::new();

    std::io::stdin()
        .read_line(&mut device_str)
        .expect("Failed to read");

    println!("{}",device_str);
}
