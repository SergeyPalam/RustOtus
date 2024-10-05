use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Cmd {
    TurnOn,
    TurnOff,
    Power,
    Temperature,
}

impl Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cmd::TurnOn => write!(f, "Turn On"),
            Cmd::TurnOff => write!(f, "Turn Off"),
            Cmd::Temperature => write!(f, "Temperature"),
            Cmd::Power => write!(f, "Power"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub addr: String,
    pub cmd: Cmd,
}

impl Request {
    pub fn new(addr: &str, cmd: Cmd) -> Self {
        Self {
            addr: addr.to_owned(),
            cmd,
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} command: {}", self.addr, self.cmd)
    }
}

#[derive(Serialize, Deserialize)]
pub enum SuccessKind {
    Ack,
    Power(f64),
    Temp(f64),
}

impl Display for SuccessKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuccessKind::Ack => write!(f, "Success"),
            SuccessKind::Power(val) => write!(f, "Power device: {}", val),
            SuccessKind::Temp(val) => write!(f, "Temp device: {}", val),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorKind {
    WrongCmd,
    DevNotFound,
    UnknownCmd,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::WrongCmd => write!(f, "Wrong command"),
            ErrorKind::DevNotFound => write!(f, "Device not found"),
            ErrorKind::UnknownCmd => write!(f, "Unknown command"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ResponseKind {
    Success(SuccessKind),
    Err(ErrorKind),
}

impl Display for ResponseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseKind::Success(val) => write!(f, "{val}"),
            ResponseKind::Err(val) => write!(f, "{val}"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub to_req: Request,
    pub resp_kind: ResponseKind,
}

impl Response {
    pub fn success_response(req: Request) -> Response {
        Self {
            to_req: req,
            resp_kind: ResponseKind::Success(SuccessKind::Ack),
        }
    }
    pub fn current_power(req: Request, power: f64) -> Response {
        Self {
            to_req: req,
            resp_kind: ResponseKind::Success(SuccessKind::Power(power)),
        }
    }

    pub fn current_temp(req: Request, temp: f64) -> Response {
        Self {
            to_req: req,
            resp_kind: ResponseKind::Success(SuccessKind::Temp(temp)),
        }
    }

    pub fn err_response(req: Request, err_kind: ErrorKind) -> Response {
        Self {
            to_req: req,
            resp_kind: ResponseKind::Err(err_kind),
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "response for request {}: {}",
            &self.to_req, self.resp_kind
        )
    }
}
