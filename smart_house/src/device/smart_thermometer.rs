use super::{DeviceType, SmartDevice};
use crate::err_house;

#[derive(Default)]
pub struct SmartThermometer {}

impl SmartThermometer {
    pub fn get_current_temp(&self) -> Result<f64, err_house::Err> {
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
    fn get_state(&self) -> Result<String, err_house::Err> {
        let temperature = self.get_current_temp()?;
        let report = format!(
            "{}, current temperature: {}",
            self.description(),
            temperature
        );
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
