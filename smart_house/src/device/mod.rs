mod sock_emulator;
mod therm_emulator;
mod sock_handler;
mod therm_handler;
mod sock_view;
mod therm_view;
use super::err_house;

use sock_emulator::SockEmulator;
use therm_emulator::ThermEmulator;
use sock_handler::SockHandler;
use therm_handler::ThermHandler;
use sock_view::SockView;
use therm_view::ThermView;
use crate::task;
use crate::protocol;

use std::{borrow::Borrow, hash::Hash};

#[derive(Clone, Copy)]
pub enum DevType {
    Sock,
    Therm,
}

pub enum View {
    SockView(SockView),
    ThermView(ThermView),
}

impl View {
    pub async fn send_req(&mut self, req: protocol::Request) -> Result<(), err_house::Err> {
        match self {
            View::SockView(val) => val.send_req(req).await,
            View::ThermView(val) => val.send_req(req).await,
        }
    }
}

pub struct Device {
    name: String,
    ip_addr: String,
    dev_type: DevType,
    pub emulator_beacon: Option<task::TaskBeacon>,
    pub handler_beacon: Option<task::TaskBeacon>,
    view: Option<View>,
}

impl Hash for Device {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Device {}

impl Borrow<str> for Device {
    fn borrow(&self) -> &str {
        &self.name
    }
}

impl Device {
    pub fn new(name: &str, ip_addr: &str, dev_type: DevType) -> Device {
        Device {
            name: name.to_owned(),
            ip_addr: ip_addr.to_owned(),
            dev_type,
            emulator_beacon: None,
            handler_beacon: None,
            view: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_addr(&self) -> &str {
        &self.ip_addr
    }

    pub async fn connect(&mut self, is_use_emulator: bool) -> Result<(), err_house::Err> {
        match self.dev_type {
            DevType::Sock => {
                if is_use_emulator {
                    self.emulator_beacon = Some(task::start(SockEmulator::new(&self.ip_addr)));
                }
                let (view, rx) = SockView::connect(&self.ip_addr, 3).await?;
                self.view = Some(View::SockView(view));
                self.handler_beacon = Some(task::start(SockHandler::new(rx)));
            }
            DevType::Therm => {
                if is_use_emulator {
                    self.emulator_beacon = Some(task::start(ThermEmulator::new(&self.ip_addr)));
                }
                let (view, rx) = ThermView::connect(&self.ip_addr).await?;
                self.view = Some(View::ThermView(view));
                self.handler_beacon = Some(task::start(ThermHandler::new(rx)));
            }
        }
        Ok(())
    }

    pub async fn send_req(&mut self, req: protocol::Request) -> Result<(), err_house::Err> {
        let view = if let Some(val) = self.view.as_mut() {
            val
        }else {
            return Err(err_house::Err::new(err_house::ErrorKind::NotOpenedConnection));
        };
        view.send_req(req).await

    }
}