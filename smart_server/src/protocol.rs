use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Cmd {
  GetListDevices,
  TurnOn,
  TurnOff,
  Power,
  Temperature,
}

#[derive(Serialize, Deserialize)]
pub enum TypeDev {
  SmartSocket,
  SmartTherm,
}

#[derive(Serialize, Deserialize)]
pub struct Device {
  pub name: String,
  pub type_dev: TypeDev,
}

impl Device {
  pub fn new(name: String, type_dev: TypeDev) -> Self{
    Self {
      name,
      type_dev,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Request {
  pub cmd: Cmd,
  pub dev_name: String,
}

impl Request {
  pub fn new(cmd: Cmd, dev_name: String) -> Self {
    Self {
      cmd,
      dev_name,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub enum SuccessKind {
  Ack,
  ListDev(Vec<Device>),
  Power(f64),
  Temp(f64),
}

#[derive(Serialize, Deserialize)]
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

impl Response {
  pub fn new_success_response(to_req: Request, success_kind: SuccessKind) -> Self{
    Self {
      to_req,
      resp_kind: ResponseKind::Success(success_kind),
    }
  }
  pub fn new_err_response(to_req: Request, err_kind: ErrorKind) -> Self {
    Self {
      to_req,
      resp_kind: ResponseKind::Err(err_kind),
    }
  }
}