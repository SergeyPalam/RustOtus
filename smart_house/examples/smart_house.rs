use std::net::TcpStream;

use smart_house::device::smart_socket::SmartSocket;
use smart_house::device::smart_thermometer::SmartThermometer;
use smart_house::room::Room;
use smart_house::SmartHouse;

fn main() {
    let dev1_stream =
        TcpStream::connect("127.0.0.1:80").expect("Can't connect to local addres at port 80");

    let mut smart_soc = Box::new(SmartSocket::default());
    smart_soc.set_stream(dev1_stream);
    if let Err(e) = smart_soc.turn_on(){
        print!("{}", e);
    }

    let smart_therm = Box::new(SmartThermometer::default());

    let mut bedroom = Room::default();
    bedroom.add_device("dev1", smart_soc);
    bedroom.add_device("dev2", smart_therm);

    let mut smart_house = SmartHouse::new("My house");
    smart_house.add_room("bedroom", bedroom);

    let smart_soc = Box::new(SmartSocket::<TcpStream>::default());
    let smart_therm = Box::new(SmartThermometer::default());

    let mut kitchen = Room::default();
    kitchen.add_device("dev1", smart_soc);
    kitchen.add_device("dev2", smart_therm);

    smart_house.add_room("kitchen", kitchen);

    let room_names: Vec<&str> = smart_house.get_rooms_names().collect();
    assert_eq!(room_names.len(), 2);

    let room = smart_house.get_room("bedroom").expect("Room not found");
    let dev_names: Vec<&str> = room.get_devices_names().collect();
    assert_eq!(dev_names.len(), 2);

    let device = room.get_device("dev1").expect("Device not found");
    println!(
        "device: {}, state: {}",
        dev_names[0],
        device.get_state().unwrap()
    );
    println!("{}", smart_house.get_report());
}
