use std::io::Cursor;
use std::path::Path;
use std::{fs, io};
use std::sync::mpsc::TryRecvError;
use std::time::Duration;

use super::err_house;
use super::device::{Device, generate_device_emulator};
use serde_json::Value;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::Receiver;
use super::transport_layer::{self, TranportPack};
use super::console_server::{Service, ConsoleCmd};
use super::protocol;
use bincode;
use log::*;

const SERVER_ADDR: &str = "127.0.0.1:4444";

pub struct UdpServer {
    devices: HashMap<String, Device>,
    rx: Option<Receiver<ConsoleCmd>>,
}

impl Service for UdpServer {
    fn start_service(mut self, rx: Receiver<ConsoleCmd>) -> JoinHandle<()>{
        self.rx = Some(rx);
        self.start()
    }
}

impl UdpServer {
    pub fn new(config_path: &Path) -> Self {
        let config_json_str =
        match fs::read_to_string(config_path) {
            Ok(res) => res,
            Err(e) => {
                error!("Can't read config from {:?}: {:?}", config_path, e);
                panic!();
            }
        };
        let config_json: Value =
        match serde_json::from_str(&config_json_str){
            Ok(res) => res,
            Err(e) => {
                error!("Can't parse config json {}: {:?}", config_json_str, e);
                panic!();
            }
        };

        let tcp_config = config_json.get("udp").expect("Wrong input config");

        let mut devices: HashMap<String, Device> = HashMap::new();
        for dev_info in tcp_config["devices"].as_array().expect("Wrong input config: devices isn't array") {
            let dev_name = dev_info["name"].as_str().expect("Wrong input config: name isn't string");
            let dev_type = dev_info["type"].as_str().expect("Wrong input config: type isn't string");
            devices.insert(dev_name.to_owned(), generate_device_emulator(dev_type).expect("Wrong input config: unknown device type"));
        }
        info!("UdpServer created");
        Self {
            devices,
            rx: None,
        }
    }

    pub fn name() -> &'static str{
        "UdpServer"
    }

    fn get_cmd(&self) -> Option<ConsoleCmd>{
        match self.rx.as_ref().unwrap().try_recv(){
            Ok(cmd) => {
                Some(cmd)
            }
            Err(e) => {
                if let TryRecvError::Disconnected = e{
                    error!("Channel disconnected");
                    panic!();
                }else{
                    None
                }
            }
        }
    }

    fn start(mut self) -> thread::JoinHandle<()> {
        thread::spawn(move||{
            let sock =
            match UdpSocket::bind(SERVER_ADDR){
                Ok(res) => res,
                Err(e) => {
                    error!("Can't bind to {SERVER_ADDR}: {:?}", e);
                    panic!();
                }
            };

            if let Err(e) = sock.set_read_timeout(Some(Duration::from_millis(100))){
                error!("Can't read timeout: {:?}", e);
                panic!();
            }

            loop{
                if let Some(cmd) = self.get_cmd() {
                    match cmd {
                        ConsoleCmd::Exit => break,
                    }
                }

                let mut req = vec![0u8; 1400];
                let (cnt_bytes, remote_addr) =
                match sock.recv_from(&mut req){
                    Ok(res) => {
                        (res.0, res.1)
                    }
                    Err(e) => {
                        match e.kind(){
                            io::ErrorKind::TimedOut => {
                                continue;
                            }
                            _ => {
                                error!("Socket error: {:?}", e);
                                panic!();
                            }
                        }
                    }
                };

                req.shrink_to(cnt_bytes);
                let resp = 
                match self.handle_request(&req){
                    Ok(res) => res,
                    Err(e) => {
                        warn!("Invalid request: {e}");
                        continue;
                    }
                };

                let mut resp = TranportPack::new(transport_layer::TypePack::Simple, resp).serialize();
                if let Err(e) = sock.send_to(&mut resp, remote_addr){
                    info!("Remote host unavailable: {:?}", e);
                }
            }
        }
        )
    }

    fn handle_request(&mut self, req: &[u8]) -> Result<Vec<u8>, err_house::Err> {
        let mut p = Cursor::new(req);
        let raw_req = TranportPack::from_reader(&mut p)?.into_payload();
        let req: protocol::Request =
        match bincode::deserialize(&raw_req){
            Ok(val) => val,
            Err(e) => {
                warn!("Wrong format request: {:?}, {:?}", req, e);
                return Err(err_house::Err::new(err_house::ErrorKind::SerializationError));
            }
        };

        let resp =
        match req.cmd {
            protocol::Cmd::GetListDevices => {
                let mut devices = Vec::new();
                for (name, dev) in self.devices.iter() {
                    devices.push(protocol::Device::new(name.to_owned(), dev.get_type_device()));
                }
                protocol::Response::new_success_response(req, protocol::SuccessKind::ListDev(devices))
            }
            protocol::Cmd::TurnOn => {
                if let Some(dev) = self.devices.get_mut(&req.dev_name) {
                    dev.turn_on();
                    info!("Device: {} is turned on", req.dev_name);
                    protocol::Response::new_success_response(req, protocol::SuccessKind::Ack)
                }else{
                    info!("Device: {} not found", req.dev_name);
                    protocol::Response::new_err_response(req, protocol::ErrorKind::DevNotFound)
                }
            }

            protocol::Cmd::TurnOff => {
                if let Some(dev) = self.devices.get_mut(&req.dev_name) {
                    dev.turn_off();
                    info!("Device: {} is turned off", req.dev_name);
                    protocol::Response::new_success_response(req, protocol::SuccessKind::Ack)
                }else{
                    info!("Device: {} not found", req.dev_name);
                    protocol::Response::new_err_response(req, protocol::ErrorKind::DevNotFound)
                }
            }

            protocol::Cmd::Power => {
                if let Some(dev) = self.devices.get_mut(&req.dev_name) {
                    match dev {
                        Device::Socket(sock) => {
                            let cur_power = sock.get_power();
                            info!("Smart socket {} power is {cur_power}", req.dev_name);
                            protocol::Response::new_success_response(req, protocol::SuccessKind::Power(cur_power))
                        }
                        Device::Therm(_) => {
                            info!("Smart therm unable get power");
                            protocol::Response::new_err_response(req, protocol::ErrorKind::WrongCmd)
                        }
                    }
                }else{
                    info!("Device: {} not found", req.dev_name);
                    protocol::Response::new_err_response(req, protocol::ErrorKind::DevNotFound)
                }
            }
            protocol::Cmd::Temperature => {
                if let Some(dev) = self.devices.get_mut(&req.dev_name) {
                    match dev {
                        Device::Socket(_) => {
                            info!("Smart socket unable get temperature");
                            protocol::Response::new_err_response(req, protocol::ErrorKind::WrongCmd)
                        }
                        Device::Therm(therm) => {
                            let cur_temp = therm.get_temperature();
                            info!("Smart therm {} temperature is {cur_temp}", req.dev_name);
                            protocol::Response::new_success_response(req, protocol::SuccessKind::Temp(cur_temp))
                        }
                    }
                }else{
                    info!("Device: {} not found", req.dev_name);
                    protocol::Response::new_err_response(req, protocol::ErrorKind::DevNotFound)
                }
            }
        };
        let res =
        match bincode::serialize(&resp){
            Ok(val) => val,
            Err(e) => {
                error!("Can't serialize response: {:?}", e);
                panic!();
            } 
        };
        Ok(res)
    }
}