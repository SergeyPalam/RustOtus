use std::sync::mpsc::TryRecvError;
use std::time::Duration;

use super::console_server::{ConsoleCmd, Service};
use super::err_house;
use super::protocol;
use super::transport_layer::{TranportPack, TypePack};
use log::*;
use std::io::Cursor;
use std::net::UdpSocket;
use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};

const SERVER_ADDR: &str = "127.0.0.1:4444";

pub struct UdpClient {
    rx: Option<Receiver<ConsoleCmd>>,
}

impl Service for UdpClient {
    fn start_service(mut self, rx: Receiver<ConsoleCmd>) -> JoinHandle<()> {
        self.rx = Some(rx);
        self.start()
    }
}

impl UdpClient {
    pub fn new() -> Self {
        info!("UdpClient created");
        Self { rx: None }
    }

    pub fn name() -> &'static str {
        "UdpClient"
    }

    fn check_cur_state(&self) -> Option<ConsoleCmd> {
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
            let udp_sock = match UdpSocket::bind("127.0.0.1:3333") {
                Ok(res) => res,
                Err(e) => {
                    error!("Can't bind to local address: {:?}", e);
                    panic!();
                }
            };

            if let Err(e) = udp_sock.connect(SERVER_ADDR) {
                error!("Can't connect to {SERVER_ADDR}: {:?}", e);
                panic!();
            };

            if let Err(e) = udp_sock.set_read_timeout(Some(Duration::from_millis(100))) {
                error!("Error read timeout {:?}", e);
                panic!();
            }

            loop {
                thread::sleep(Duration::from_millis(100));
                let cmd = if let Some(val) = self.check_cur_state() {
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
                        info!("Exit from udp client");
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
                if let Err(e) = udp_sock.send(&pack) {
                    info!("Server: {SERVER_ADDR} doesn't respond: {:?}", e);
                    break;
                }

                let mut resp = vec![0; 1500];
                match udp_sock.recv(&mut resp) {
                    Ok(pack_len) => resp.shrink_to(pack_len),
                    Err(e) => {
                        if let std::io::ErrorKind::TimedOut = e.kind() {
                            continue;
                        } else {
                            info!("Connection ins't valid: {:?}", e);
                            break;
                        }
                    }
                }

                if let Err(e) = self.handle_response(&resp) {
                    error!("Wrong response: {:?}", e);
                    break;
                }
            }
        })
    }

    fn handle_response(&mut self, resp: &[u8]) -> Result<(), err_house::Err> {
        let mut p = Cursor::new(resp);
        let pack = TranportPack::from_reader(&mut p)?;
        let resp: protocol::Response = match bincode::deserialize(&pack.into_payload()) {
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
                    println!("Udp: Command: {:?} success", resp.to_req.cmd);
                }
                protocol::SuccessKind::ListDev(devices) => {
                    println!("Udp: Count connected devices: {}", devices.len());
                    for dev in devices.iter() {
                        println!("{}: {}", dev.name, dev.type_dev)
                    }
                }
                protocol::SuccessKind::Power(power) => {
                    println!("Udp: Device: {} power: {}", resp.to_req.dev_name, power);
                }
                protocol::SuccessKind::Temp(temp) => {
                    println!(
                        "Udp: Device: {} temperature: {}",
                        resp.to_req.dev_name, temp
                    );
                }
            },
            protocol::ResponseKind::Err(e) => {
                println!("Udp: Error: {:?}", e);
            }
        }
        Ok(())
    }
}
