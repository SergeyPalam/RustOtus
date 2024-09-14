mod smart_socket;
mod smart_therm;

use smart_socket::SmartSocket;
use smart_therm::SmartTherm;
use super::err_house;
use super::protocol::TypeDev;
use log::*;

const SOCKET_TYPE: &str = "socket";
const THERM_TYPE: &str = "therm";

pub enum Device {
    Socket(SmartSocket),
    Therm(SmartTherm),
}

impl Device {
    pub fn get_type_device(&self) -> TypeDev {
        match self {
            Device::Socket(_) => TypeDev::SmartSocket,
            Device::Therm(_) => TypeDev::SmartTherm,
        }
    }
    pub fn turn_on(&mut self) {
        match self {
            Device::Socket(sock) => sock.turn_on(),
            Device::Therm(therm) => therm.turn_on(),
        }
    }
    pub fn turn_off(&mut self) {
        match self {
            Device::Socket(sock) => sock.turn_off(),
            Device::Therm(therm) => therm.turn_off(),
        }
    }
}

pub fn generate_device_emulator(dev_type: &str) -> Result<Device, err_house::Err>{
    match dev_type {
        SOCKET_TYPE => Ok(Device::Socket(SmartSocket::default())),
        THERM_TYPE => Ok(Device::Therm(SmartTherm::default())),
        _ => {
            error!("Invalid device type for generation");
            Err(err_house::Err::new(err_house::ErrorKind::WrongDevType))
        }
    }
}