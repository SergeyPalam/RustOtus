use std::collections::HashMap;
use std::fmt;

enum DeviceState {
    Ok(String),
    #[allow(dead_code)]
    Fault(String),
}

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceState::Ok(cur_state) => write!(f, "{cur_state}"),
            DeviceState::Fault(fault_reason) => write!(f, "{fault_reason}"),
        }
    }
}
trait SmartDevice {
    fn description(&self) -> String;
    fn get_state(&self) -> DeviceState;
}
struct SmartSocket {}

impl SmartSocket {
    fn new() -> Self {
        Self {}
    }

    fn _turn_on(&mut self) {
        todo!("Impl turn on functionality");
    }

    fn _turn_off(&mut self) {
        todo!("Impl turn off functionality");
    }

    fn get_current_power(&self) -> f64 {
        0.0
    }
}

impl SmartDevice for SmartSocket {
    fn description(&self) -> String {
        "Smart socket".to_owned()
    }
    fn get_state(&self) -> DeviceState {
        let report = format!(
            "{}, current power: {}",
            self.description(),
            self.get_current_power()
        );
        DeviceState::Ok(report)
    }
}

struct SmartThermometer {}

impl SmartThermometer {
    fn new() -> Self {
        Self {}
    }

    fn get_current_temp(&self) -> f64 {
        0.0
    }
}

impl SmartDevice for SmartThermometer {
    fn description(&self) -> String {
        "Smart thermometer".to_owned()
    }
    fn get_state(&self) -> DeviceState {
        let report = format!(
            "{}, current temperature: {}",
            self.description(),
            self.get_current_temp()
        );
        DeviceState::Ok(report)
    }
}

struct Room {
    devices: HashMap<String, Box<dyn SmartDevice>>,
}

impl Room {
    fn new() -> Room {
        Room {
            devices: HashMap::new(),
        }
    }

    fn add_device(&mut self, dev_name: &str, device: Box<dyn SmartDevice>) {
        self.devices.insert(dev_name.to_owned(), device);
    }

    fn get_device(&self, name: &str) -> Option<&dyn SmartDevice> {
        self.devices.get(name).map(|val| val.as_ref())
    }

    fn get_devices_names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
        Box::new(self.devices.keys().map(|name| name.as_str()))
    }

    fn get_report(&self) -> String {
        let mut res = String::new();
        for (dev_name, device) in self.devices.iter() {
            let dev_report = format!("{dev_name}: {}", device.get_state());
            res.push_str(&dev_report);
            res.push('\n');
        }
        res.pop();
        res
    }
}

struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            rooms: HashMap::new(),
        }
    }

    fn get_rooms_names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
        Box::new(self.rooms.keys().map(|name| name.as_str()))
    }

    fn add_room(&mut self, room_name: &str, room: Room) {
        self.rooms.insert(room_name.to_owned(), room);
    }

    fn get_room(&self, room_name: &str) -> Option<&Room> {
        self.rooms.get(room_name)
    }

    fn get_report(&self) -> String {
        let mut res = String::from(&self.name);
        res.push('\n');
        for (room_name, room) in self.rooms.iter() {
            res.push_str(format!("{room_name}:\n").as_str());
            res.push_str(&room.get_report());
            res.push('\n');
        }
        res.pop();
        res
    }
}

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

    let room = smart_house.get_room(room_names[0]).unwrap();
    let dev_names: Vec<&str> = room.get_devices_names().collect();
    assert_eq!(dev_names.len(), 2);
    let device = room.get_device(dev_names[0]).unwrap();
    println!("device: {}, state: {}", dev_names[0], device.get_state());
    println!("{}", smart_house.get_report());
}
