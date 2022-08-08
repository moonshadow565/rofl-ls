#![allow(clippy::option_map_unit_fn, dead_code)]
#![feature(exclusive_range_pattern)]
#![feature(is_sorted)]
#![feature(map_first_last)]

mod rofl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rofl_path = std::env::args().nth(1).expect("Supply path to .rofl file as first argument!");
    let rofl = rofl::Rofl::from_file(&rofl_path)?;
    let packets = rofl.dump_packets(true)?;
    for packet in packets.iter() {
        println!("{:02}:{:09.6},{:04X},{:08X},{:08X}", (packet.time as u32) / 60, packet.time % 60.0, packet.id, packet.param, packet.data.len());
    }
    Ok(())
}
