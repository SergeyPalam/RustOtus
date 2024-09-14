use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub enum Cmd {
    GetListDevices,
    TurnOn,
    TurnOff,
    Power,
    Temperature,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum TypeDev {
    SmartSocket,
    SmartTherm,
}

impl Display for TypeDev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeDev::SmartSocket => write!(f, "Smart Socket"),
            TypeDev::SmartTherm => write!(f, "Smart Therm"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub type_dev: TypeDev,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub cmd: Cmd,
    pub dev_name: String,
}

impl Request {
    pub fn new(cmd: Cmd, dev_name: String) -> Self {
        Self { cmd, dev_name }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SuccessKind {
    Ack,
    ListDev(Vec<Device>),
    Power(f64),
    Temp(f64),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorKind {
    WrongCmd,
    DevNotFound,
    UnknownCmd,
}

#[derive(Serialize, Deserialize)]
pub enum ResponseKind {
    Success(SuccessKind),
    Err(ErrorKind),
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub to_req: Request,
    pub resp_kind: ResponseKind,
}
