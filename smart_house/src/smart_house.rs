use std::collections::HashSet;
use super::room::Room;
use log::*;

pub struct SmartHouse {
    rooms: HashSet<Room>,
}

impl SmartHouse {
    pub fn new () -> SmartHouse{
        Self {
            rooms: HashSet::new(),
        }
    }

    pub fn get_room(&self, room_name: &str) -> Option<&Room>{
        self.rooms.get(room_name)
    }

    pub fn add_room(&mut self, room: Room){
        if !self.rooms.insert(room) {
            info!("Room already exist in house")
        }
    }

    pub fn delete_room(&mut self, room_name: &str) {
        self.rooms.remove(room_name);
    }
}