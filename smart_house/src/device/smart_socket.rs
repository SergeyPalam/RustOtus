use super::socket_protocol::*;
use super::transport_layer::*;
use super::{DeviceType, SmartDevice};
use crate::err_house;
use byteorder::{ByteOrder, BE};
use std::cell::RefCell;
use std::io::{Read, Write};

pub struct SmartSocket<T: Read + Write> {
    stream: Option<RefCell<T>>,
}

impl<T: Read + Write> SmartSocket<T> {
    pub fn set_stream(&mut self, stream: T) {
        self.stream = Some(RefCell::new(stream));
    }

    fn issue_cmd(&self, req: SockRequest) -> Result<SockResponse, err_house::Err> {
        if let Some(stream) = self.stream.as_ref() {
            let req = req.serialize();
            let pack = TranportPack::new(TypePack::Simple, req);
            stream.borrow_mut().write_all(&pack.serialize())?;
            let transport_pack = TranportPack::from_reader(&mut *stream.borrow_mut())?;
            let resp_pack = SockResponse::deserialize(transport_pack.get_payload())?;
            Ok(resp_pack)
        } else {
            Err(err_house::Err::new("Stream for this device doesn't set up"))
        }
    }

    pub fn turn_on(&self) -> Result<(), err_house::Err> {
        let resp = self.issue_cmd(SockRequest::new_turn_on())?;
        match resp.get_resp_type() {
            RespType::Success => Ok(()),
            RespType::Error => Err(err_house::Err::new("Can't turn on socket")),
            RespType::Unknown(val) => Err(err_house::Err::new(&format!("Unknown response {val}"))),
        }
    }

    pub fn turn_off(&self) -> Result<(), err_house::Err> {
        let resp = self.issue_cmd(SockRequest::new_turn_off())?;
        match resp.get_resp_type() {
            RespType::Success => Ok(()),
            RespType::Error => Err(err_house::Err::new("Can't turn off socket")),
            RespType::Unknown(val) => Err(err_house::Err::new(&format!("Unknown response {val}"))),
        }
    }

    pub fn is_turned_on(&self) -> Result<bool, err_house::Err> {
        let resp = self.issue_cmd(SockRequest::new_check_turned())?;
        match resp.get_resp_type() {
            RespType::Success => {
                let payload = resp.get_payload();
                if payload.is_empty() {
                    return Err(err_house::Err::new("Wrong check turned response"));
                }

                let is_turned_on = payload[0] != 0;
                Ok(is_turned_on)
            }
            RespType::Error => Err(err_house::Err::new("Can't check turned socket")),
            RespType::Unknown(val) => Err(err_house::Err::new(&format!("Unknown response {val}"))),
        }
    }

    pub fn get_current_power(&self) -> Result<f64, err_house::Err> {
        let resp = self.issue_cmd(SockRequest::new_get_power())?;
        match resp.get_resp_type() {
            RespType::Success => {
                let payload = resp.get_payload();
                if payload.len() < std::mem::size_of::<u32>() {
                    return Err(err_house::Err::new(
                        "Wrong response for current power request",
                    ));
                }
                let res = BE::read_u32(payload) as f64 / 1000.0;
                Ok(res)
            }
            RespType::Error => Err(err_house::Err::new("Can't turn off socket")),
            RespType::Unknown(val) => Err(err_house::Err::new(&format!("Unknown response {val}"))),
        }
    }
}

impl<T: Read + Write> SmartDevice for SmartSocket<T> {
    fn get_type(&self) -> DeviceType {
        DeviceType::SmartSocket
    }
    fn description(&self) -> String {
        "Smart socket".to_owned()
    }
    fn get_state(&self) -> Result<String, err_house::Err> {
        let is_turentd_on = self.is_turned_on()?;
        let current_power = self.get_current_power()?;
        let report = format!(
            "{}, current power Wt: {}, turned on: {}",
            self.description(),
            current_power,
            is_turentd_on
        );
        Ok(report)
    }
}

impl<T: Read + Write> Default for SmartSocket<T> {
    fn default() -> Self {
        Self { stream: None }
    }
}

#[cfg(test)]
mod tests {
    use std::net::TcpStream;

    use super::*;
    #[test]
    fn test_smart_socket_dev_type() {
        let smart_device: Box<dyn SmartDevice> = Box::new(SmartSocket::<TcpStream>::default());
        let dev_type = smart_device.get_type();
        match dev_type {
            DeviceType::SmartSocket => assert!(true),
            _ => assert!(false),
        }
    }
}
