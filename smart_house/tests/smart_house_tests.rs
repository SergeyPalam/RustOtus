use smart_house::device::*;
use smart_house::room::Room;
use smart_house::SmartHouse;
#[test]
fn test_rooms_operations() {
    let room1 = Room::new();
    let room2 = Room::new();
    let room3 = Room::new();
    let mut smart_house = SmartHouse::new("My smart house");
    smart_house.add_room("bedroom", room1);
    smart_house.add_room("kitchen", room2);
    smart_house.add_room("kitchen", room3);
    let names: Vec<&str> = smart_house.get_rooms_names().collect();
    assert_eq!(names.len(), 2);
    assert_eq!(names.iter().filter(|&&name| name.eq("bedroom")).count(), 1);
    assert_eq!(names.iter().filter(|&&name| name.eq("kitchen")).count(), 1);
}

#[test]
fn test_report() {
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

    let report = smart_house.get_report();
    assert!(!report.is_empty());
}
