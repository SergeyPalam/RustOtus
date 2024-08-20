pub mod smart_socket;
pub mod smart_thermometer;
pub mod socket_protocol;
pub mod transport_layer;

use crate::err_house;
pub enum DeviceType {
    SmartSocket,
    SmartThermometer,
}

pub trait SmartDevice {
    fn get_type(&self) -> DeviceType;
    fn description(&self) -> String;
    fn get_state(&self) -> Result<String, err_house::Err>;
}
