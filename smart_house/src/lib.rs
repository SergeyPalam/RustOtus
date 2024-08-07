pub mod device;
pub mod room;

use room::Room;
use std::collections::HashMap;

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            rooms: HashMap::new(),
        }
    }

    pub fn get_rooms_names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
        Box::new(self.rooms.keys().map(|name| name.as_str()))
    }

    pub fn add_room(&mut self, room_name: &str, room: Room) {
        self.rooms.insert(room_name.to_owned(), room);
    }

    pub fn remove_room(&mut self, room_name: &str) -> Option<(String, Room)> {
        self.rooms.remove_entry(room_name)
    }

    pub fn get_room(&self, room_name: &str) -> Option<&Room> {
        self.rooms.get(room_name)
    }

    pub fn get_report(&self) -> String {
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
