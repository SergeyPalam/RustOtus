use std::fmt;

pub enum DeviceType {
    SmartSocket,
    SmartThermometer,
}

#[derive(Debug)]
pub enum DeviceState {
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
pub trait SmartDevice {
    fn get_type(&self) -> DeviceType;
    fn description(&self) -> String;
    fn get_state(&self) -> DeviceState;
}
#[derive(Default)]
pub struct SmartSocket {}

impl SmartSocket {
    pub fn _turn_on(&mut self) {
        todo!("Impl turn on functionality");
    }

    pub fn _turn_off(&mut self) {
        todo!("Impl turn off functionality");
    }

    pub fn get_current_power(&self) -> Result<f64, String> {
        Ok(0.0)
    }
}

impl SmartDevice for SmartSocket {
    fn get_type(&self) -> DeviceType {
        DeviceType::SmartSocket
    }
    fn description(&self) -> String {
        "Smart socket".to_owned()
    }
    fn get_state(&self) -> DeviceState {
        match self.get_current_power() {
            Ok(power) => {
                let report = format!("{}, current power: {}", self.description(), power);
                DeviceState::Ok(report)
            }
            Err(err) => DeviceState::Fault(err),
        }
    }
}

#[derive(Default)]
pub struct SmartThermometer {}

impl SmartThermometer {
    pub fn get_current_temp(&self) -> Result<f64, String> {
        Ok(0.0)
    }
}

impl SmartDevice for SmartThermometer {
    fn get_type(&self) -> DeviceType {
        DeviceType::SmartThermometer
    }
    fn description(&self) -> String {
        "Smart thermometer".to_owned()
    }
    fn get_state(&self) -> DeviceState {
        match self.get_current_temp() {
            Ok(temperature) => {
                let report = format!(
                    "{}, current temperature: {}",
                    self.description(),
                    temperature
                );
                DeviceState::Ok(report)
            }
            Err(err) => DeviceState::Fault(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smart_socket_dev_type() {
        let smart_device: Box<dyn SmartDevice> = Box::new(SmartSocket::default());
        let dev_type = smart_device.get_type();
        match dev_type {
            DeviceType::SmartSocket => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_smart_thermometer_dev_type() {
        let smart_device: Box<dyn SmartDevice> = Box::new(SmartThermometer::default());
        let dev_type = smart_device.get_type();
        match dev_type {
            DeviceType::SmartThermometer => assert!(true),
            _ => assert!(false),
        }
    }
}
