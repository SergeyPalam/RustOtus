use super::device::SmartDevice;
use std::collections::HashMap;

#[derive(Default)]
pub struct Room {
    devices: HashMap<String, Box<dyn SmartDevice>>,
}

impl Room {
    pub fn add_device(&mut self, dev_name: &str, device: Box<dyn SmartDevice>) {
        self.devices.insert(dev_name.to_owned(), device);
    }

    pub fn remove_device(&mut self, dev_name: &str) -> Option<(String, Box<dyn SmartDevice>)> {
        self.devices.remove_entry(dev_name)
    }

    pub fn get_device(&self, name: &str) -> Option<&dyn SmartDevice> {
        self.devices.get(name).map(|val| val.as_ref())
    }

    pub fn get_devices_names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
        Box::new(self.devices.keys().map(|name| name.as_str()))
    }

    pub fn get_report(&self) -> String {
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

#[cfg(test)]
mod tests {
    use crate::device::{SmartSocket, SmartThermometer};

    use super::*;

    #[test]
    fn test_add_device() {
        let mut room = Room::default();
        let dev1 = Box::new(SmartSocket::default());
        let dev2 = Box::new(SmartThermometer::default());
        let dev3 = Box::new(SmartSocket::default());

        room.add_device("dev1", dev1);
        room.add_device("dev2", dev2);
        room.add_device("dev2", dev3);

        let dev_names = room.get_devices_names().collect::<Vec<&str>>();
        assert_eq!(dev_names.len(), 2);

        let _ = room.get_device("dev1").unwrap();
        let _ = room.get_device("dev2").unwrap();
        let dev3 = room.get_device("dev3");
        assert!(dev3.is_none());
    }

    #[test]
    fn test_remove_device() {
        let mut room = Room::default();
        let dev1 = Box::new(SmartSocket::default());
        let dev2 = Box::new(SmartThermometer::default());
        let dev3 = Box::new(SmartSocket::default());

        room.add_device("dev1", dev1);
        room.add_device("dev2", dev2);
        room.add_device("dev3", dev3);

        let dev_names = room.get_devices_names().collect::<Vec<&str>>();
        assert_eq!(dev_names.len(), 3);

        let removed = room.remove_device("dev2").unwrap();
        assert_eq!(removed.0, "dev2");
        let dev_names = room.get_devices_names().collect::<Vec<&str>>();
        assert_eq!(dev_names.len(), 2);

        let removed = room.remove_device("dev4");
        assert!(removed.is_none());
    }

    #[test]
    fn test_get_report() {
        let mut room = Room::default();
        let dev1 = Box::new(SmartSocket::default());
        let dev2 = Box::new(SmartThermometer::default());
        let dev3 = Box::new(SmartSocket::default());

        room.add_device("dev1", dev1);
        room.add_device("dev2", dev2);
        room.add_device("dev2", dev3);

        let room_report = room.get_report();
        assert!(!room_report.is_empty());
    }
}
