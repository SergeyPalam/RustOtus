use std::net::TcpStream;

use smart_house::device::smart_socket::*;
use smart_house::device::smart_thermometer::*;
use smart_house::room::Room;
use smart_house::SmartHouse;
#[test]
fn test_rooms_operations() {
    let room1 = Room::default();
    let room2 = Room::default();
    let room3 = Room::default();
    let mut smart_house = SmartHouse::new("My smart house");
    smart_house.add_room("bedroom", room1);
    smart_house.add_room("kitchen", room2);
    smart_house.add_room("kitchen", room3);
    let names: Vec<&str> = smart_house.get_rooms_names().collect();
    assert_eq!(names.len(), 2);
    assert_eq!(names.iter().filter(|&&name| name.eq("bedroom")).count(), 1);
    assert_eq!(names.iter().filter(|&&name| name.eq("kitchen")).count(), 1);

    let _ = smart_house.remove_room("kitchen").unwrap();
    let names: Vec<&str> = smart_house.get_rooms_names().collect();
    assert_eq!(names.len(), 1);
    assert_eq!(names[0], "bedroom");
}

#[test]
fn test_report() {
    let smart_soc = Box::new(SmartSocket::<TcpStream>::default());
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

    let report = smart_house.get_report();
    assert!(!report.is_empty());
}
