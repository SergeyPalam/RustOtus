use super::device::Device;
use log::*;
use std::borrow::Borrow;
use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;

#[derive(Default, Eq)]
pub struct Room {
    name: String,
    devices: HashSet<Device>,
}

impl Room {
    pub fn new(name: &str) -> Room {
        Room {
            name: name.to_owned(),
            devices: HashSet::new(),
        }
    }
    pub fn add_device(&mut self, dev: Device) {
        if !self.devices.insert(dev) {
            warn!("Can't add device: device: already exist");
        }
    }
    pub fn get_device(&mut self, dev_name: &str) -> Option<&Device> {
        self.devices.get(dev_name)
    }
    pub fn get_devices(&self) -> impl Iterator<Item = &Device> {
        self.devices.iter()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Borrow<str> for Room {
    fn borrow(&self) -> &str {
        &self.name
    }
}

impl Hash for Room {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
