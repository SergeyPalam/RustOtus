mod sock_emulator;
mod sock_handler;
mod sock_view;
mod therm_emulator;
mod therm_handler;
mod therm_view;
use super::err_house;

use crate::{protocol, DB_TASKS};
use anyhow::{bail, Result};
use sock_emulator::SockEmulator;
use sock_handler::SockHandler;
use sock_view::SockView;
use therm_emulator::ThermEmulator;
use therm_handler::ThermHandler;
use therm_view::ThermView;
use tokio::task::AbortHandle;

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
    pub async fn send_req(&mut self, req: protocol::Request) -> Result<()> {
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
    emulator_abort: Option<AbortHandle>,
    handler_abort: Option<AbortHandle>,
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
            emulator_abort: None,
            handler_abort: None,
            view: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_addr(&self) -> &str {
        &self.ip_addr
    }

    pub async fn connect(&mut self, is_use_emulator: bool) -> Result<()> {
        match self.dev_type {
            DevType::Sock => {
                if is_use_emulator {
                    let mut lock = DB_TASKS.write().unwrap();
                    let abort_handle = lock.spawn(SockEmulator::new(&self.ip_addr).start());
                    self.emulator_abort = Some(abort_handle);
                }
                let (view, rx) = SockView::connect(&self.ip_addr, 3).await?;
                self.view = Some(View::SockView(view));
                let mut lock = DB_TASKS.write().unwrap();
                let abort_handle = lock.spawn(SockHandler::new(rx).start());
                self.handler_abort = Some(abort_handle);
            }
            DevType::Therm => {
                if is_use_emulator {
                    let mut lock = DB_TASKS.write().unwrap();
                    let abort_handle = lock.spawn(ThermEmulator::new(&self.ip_addr).start());
                    self.emulator_abort = Some(abort_handle);
                }
                let (view, rx) = ThermView::connect(&self.ip_addr).await?;
                self.view = Some(View::ThermView(view));
                let mut lock = DB_TASKS.write().unwrap();
                let abort_handle = lock.spawn(ThermHandler::new(rx).start());
                self.handler_abort = Some(abort_handle);
            }
        }
        Ok(())
    }

    pub async fn send_req(&mut self, req: protocol::Request) -> Result<()> {
        let view = if let Some(val) = self.view.as_mut() {
            val
        } else {
            bail!(err_house::ErrorKind::NotOpenedConnection);
        };
        view.send_req(req).await
    }
}
