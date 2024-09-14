use std::io::Write;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;

use super::console_server::{ConsoleCmd, Service};
use super::err_house;
use super::protocol;
use super::transport_layer::{TranportPack, TypePack};
use log::*;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};

const SERVER_ADDR: &str = "127.0.0.1:444";

pub struct TcpClient {
    rx: Option<Receiver<ConsoleCmd>>,
}

impl Service for TcpClient {
    fn start_service(mut self, rx: Receiver<ConsoleCmd>) -> JoinHandle<()> {
        self.rx = Some(rx);
        self.start()
    }
}

impl TcpClient {
    pub fn new() -> Self {
        info!("TcpClient created");
        Self { rx: None }
    }

    pub fn name() -> &'static str {
        "TcpClient"
    }

    fn check_cmd(&self) -> Option<ConsoleCmd> {
        match self.rx.as_ref().unwrap().try_recv() {
            Ok(cmd) => Some(cmd),
            Err(e) => {
                if let TryRecvError::Disconnected = e {
                    error!("Channel disconnected");
                    panic!();
                } else {
                    None
                }
            }
        }
    }

    fn start(mut self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut tcp_stream = match TcpStream::connect(SERVER_ADDR) {
                Ok(res) => res,
                Err(e) => {
                    error!("Can't connect to {SERVER_ADDR}: {:?}", e);
                    panic!();
                }
            };
            if let Err(e) = tcp_stream.set_read_timeout(Some(Duration::from_millis(100))) {
                error!("Error read timeout {:?}", e);
                panic!();
            }

            loop {
                thread::sleep(Duration::from_millis(100));
                let cmd = if let Some(val) = self.check_cmd() {
                    val
                } else {
                    continue;
                };
                let req = match cmd {
                    ConsoleCmd::GetDevs => {
                        protocol::Request::new(protocol::Cmd::GetListDevices, String::new())
                    }
                    ConsoleCmd::TurnOn(name) => {
                        protocol::Request::new(protocol::Cmd::TurnOn, name.to_owned())
                    }
                    ConsoleCmd::TurnOff(name) => {
                        protocol::Request::new(protocol::Cmd::TurnOff, name.to_owned())
                    }
                    ConsoleCmd::GetPower(name) => {
                        protocol::Request::new(protocol::Cmd::Power, name.to_owned())
                    }
                    ConsoleCmd::GetTemp(name) => {
                        protocol::Request::new(protocol::Cmd::Temperature, name.to_owned())
                    }
                    ConsoleCmd::Exit => {
                        info!("Exit from tcp client");
                        break;
                    }
                };

                let raw_req = match bincode::serialize(&req) {
                    Ok(val) => val,
                    Err(e) => {
                        error!("Can't serialize request: {:?}", e);
                        panic!();
                    }
                };
                let pack = TranportPack::new(TypePack::Simple, raw_req).serialize();
                if let Err(e) = tcp_stream.write_all(&pack) {
                    info!("Connection closed: {:?}", e);
                    break;
                }

                let resp = match TranportPack::from_reader(&mut tcp_stream) {
                    Ok(pack) => pack.into_payload(),
                    Err(e) => {
                        if let err_house::ErrorKind::IoTimeOut = e.kind() {
                            continue;
                        } else {
                            info!("Connection closed");
                            break;
                        }
                    }
                };

                if let Err(e) = self.handle_response(&resp) {
                    error!("Wrong response: {:?}", e);
                    break;
                }
            }
        })
    }

    fn handle_response(&mut self, resp: &[u8]) -> Result<(), err_house::Err> {
        let resp: protocol::Response = match bincode::deserialize(resp) {
            Ok(res) => res,
            Err(e) => {
                error!("Wrong protocol: {:?}", e);
                return Err(err_house::Err::new(
                    err_house::ErrorKind::DeserializationError,
                ));
            }
        };

        match resp.resp_kind {
            protocol::ResponseKind::Success(success_kind) => match success_kind {
                protocol::SuccessKind::Ack => {
                    println!("Tcp: Command: {:?} success", resp.to_req.cmd);
                }
                protocol::SuccessKind::ListDev(devices) => {
                    println!("Tcp: Count connected devices: {}", devices.len());
                    for dev in devices.iter() {
                        println!("{}: {}", dev.name, dev.type_dev)
                    }
                }
                protocol::SuccessKind::Power(power) => {
                    println!("Tcp: Device: {} power: {}", resp.to_req.dev_name, power);
                }
                protocol::SuccessKind::Temp(temp) => {
                    println!(
                        "Tcp: Device: {} temperature: {}",
                        resp.to_req.dev_name, temp
                    );
                }
            },
            protocol::ResponseKind::Err(e) => {
                println!("Tcp: Error: {:?}", e);
            }
        }
        Ok(())
    }
}
