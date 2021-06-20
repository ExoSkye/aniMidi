extern crate portmidi as pm;

fn main() {
    let ctx = match pm::PortMidi::new() {
        Ok(ctx) =>  ctx,
        Err(_) => panic!("Couldn't create portmidi context")
    };
    println!("Hello, world!");
}
