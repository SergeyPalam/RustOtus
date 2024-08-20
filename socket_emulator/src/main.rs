mod socket_emulator;
mod socket_protocol;
mod transport_layer;
mod err_house;

use socket_emulator::SocketEmulator;

fn main() {
    println!("Start emulator smart socket");
    let mut emulator = SocketEmulator::default();
    emulator.start_server();
}
