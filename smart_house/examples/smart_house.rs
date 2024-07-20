use smart_house::device::*;
use smart_house::room::Room;
use smart_house::SmartHouse;
fn main() {
    let smart_soc = Box::new(SmartSocket::new());
    let smart_therm = Box::new(SmartThermometer::new());

    let mut bedroom = Room::new();
    bedroom.add_device("dev1", smart_soc);
    bedroom.add_device("dev2", smart_therm);

    let mut smart_house = SmartHouse::new("My house");
    smart_house.add_room("bedroom", bedroom);

    let smart_soc = Box::new(SmartSocket::new());
    let smart_therm = Box::new(SmartThermometer::new());

    let mut kitchen = Room::new();
    kitchen.add_device("dev1", smart_soc);
    kitchen.add_device("dev2", smart_therm);

    smart_house.add_room("kitchen", kitchen);

    let room_names: Vec<&str> = smart_house.get_rooms_names().collect();
    assert_eq!(room_names.len(), 2);

    let room = smart_house.get_room(room_names[0]).expect("Room not found");
    let dev_names: Vec<&str> = room.get_devices_names().collect();
    assert_eq!(dev_names.len(), 2);
    let device = room.get_device(dev_names[0]).expect("Device not found");
    println!("device: {}, state: {}", dev_names[0], device.get_state());
    println!("{}", smart_house.get_report());
}
