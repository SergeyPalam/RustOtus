use log::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

use super::err_house;
use super::smart_house_tcp_client::TcpClient;
use super::smart_house_udp_client::UdpClient;

const GET_DEVICES: &str = "get_devs";
const TURN_ON: &str = "turn_on";
const TURN_OFF: &str = "turn_off";
const GET_POWER: &str = "get_power";
const GET_TEMP: &str = "get_temp";
const GET_REPORT: &str = "report";

const EXIT: &str = "exit";

pub enum ConsoleCmd {
    GetDevs,
    TurnOn(String),
    TurnOff(String),
    GetPower(String),
    GetTemp(String),
    Exit,
}

fn help() {
    println!("Type \"{}\" to all devices from all servers", GET_DEVICES);
    println!(
        "Type \"{}\" \"dev name\" to turn on particular device",
        TURN_ON
    );
    println!(
        "Type \"{}\" \"dev name\" to turn off particular device",
        TURN_OFF
    );
    println!("Type \"{}\" \"dev name\" to get power of socket", GET_POWER);
    println!(
        "Type \"{}\" \"dev name\" to get temperature of thermometer",
        GET_TEMP
    );
    println!("Type \"{}\" get report from servers", GET_REPORT);
    println!("Type \"exit\" to exit from smart house app");
}

pub trait Service {
    fn start_service(self, rx: Receiver<ConsoleCmd>) -> JoinHandle<()>;
}

pub struct Channel {
    thread_handle: JoinHandle<()>,
    tx: Sender<ConsoleCmd>,
}

impl Channel {
    pub fn new(thread_handle: JoinHandle<()>, tx: Sender<ConsoleCmd>) -> Self {
        Self { thread_handle, tx }
    }
}

pub struct ConsoleServer {
    channels: HashMap<&'static str, Channel>,
}

impl ConsoleServer {
    fn connect_to_service(&mut self, service: impl Service, serv_name: &'static str) {
        let (tx, rx) = mpsc::channel();
        let thread_handle = service.start_service(rx);
        let channel = Channel::new(thread_handle, tx);
        let entry = self.channels.entry(serv_name);
        match entry {
            Entry::Occupied(_) => {
                error!("Service {serv_name} already connected");
            }
            Entry::Vacant(e) => {
                e.insert(channel);
                log::info!("Service: {serv_name} connected");
            }
        }
    }

    fn join(self) {
        for (_, channel) in self.channels {
            if let Err(e) = channel.thread_handle.join() {
                error!("Can't join finished thread: {:?}", e);
                panic!();
            }
        }
    }

    fn send_service_cmd(
        &mut self,
        service_name: &str,
        cmd: ConsoleCmd,
    ) -> Result<(), err_house::Err> {
        if let Some(channel) = self.channels.get(service_name) {
            if channel.tx.send(cmd).is_err() {
                error!("Service: {service_name} isn't responding");
                return Err(err_house::Err::new(err_house::ErrorKind::ServiceNotRespond));
            }
        } else {
            error!("Unknown service: {service_name}");
            return Err(err_house::Err::new(err_house::ErrorKind::UnknownService));
        }
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn start(mut self) {
        println!("Start client");
        help();
        let std_in = io::stdin();
        let tcp_client = TcpClient::new();
        let udp_client = UdpClient::new();

        self.connect_to_service(tcp_client, TcpClient::name());
        self.connect_to_service(udp_client, UdpClient::name());

        for line in std_in.lock().lines() {
            let params = match line {
                Ok(res) => res
                    .split(' ')
                    .filter(|param| !param.is_empty())
                    .map(|param| param.to_owned())
                    .collect::<Vec<String>>(),
                Err(_) => {
                    error!("IO error");
                    panic!();
                }
            };
            if params.is_empty() {
                continue;
            }

            match params[0].as_str() {
                GET_DEVICES => {
                    if let Err(e) = self.send_service_cmd(TcpClient::name(), ConsoleCmd::GetDevs) {
                        error!("Can't get tcp devices: {e}");
                        panic!();
                    }

                    if let Err(e) = self.send_service_cmd(UdpClient::name(), ConsoleCmd::GetDevs) {
                        error!("Can't get udp devices: {e}");
                        panic!();
                    }
                }
                TURN_ON => {
                    if params.len() != 2 {
                        println!("Wrong command");
                        help();
                        continue;
                    }

                    self.send_service_cmd(
                        TcpClient::name(),
                        ConsoleCmd::TurnOn(params[1].to_owned()),
                    )
                    .unwrap();
                    self.send_service_cmd(
                        UdpClient::name(),
                        ConsoleCmd::TurnOn(params[1].to_owned()),
                    )
                    .unwrap();
                }
                TURN_OFF => {
                    if params.len() != 2 {
                        println!("Wrong command");
                        help();
                        continue;
                    }

                    self.send_service_cmd(
                        TcpClient::name(),
                        ConsoleCmd::TurnOff(params[1].to_owned()),
                    )
                    .unwrap();
                    self.send_service_cmd(
                        UdpClient::name(),
                        ConsoleCmd::TurnOff(params[1].to_owned()),
                    )
                    .unwrap();
                }

                GET_POWER => {
                    if params.len() != 2 {
                        println!("Wrong command");
                        help();
                        continue;
                    }

                    self.send_service_cmd(
                        TcpClient::name(),
                        ConsoleCmd::GetPower(params[1].to_owned()),
                    )
                    .unwrap();
                    self.send_service_cmd(
                        UdpClient::name(),
                        ConsoleCmd::GetPower(params[1].to_owned()),
                    )
                    .unwrap();
                }

                GET_TEMP => {
                    if params.len() != 2 {
                        println!("Wrong command");
                        help();
                        continue;
                    }

                    self.send_service_cmd(
                        TcpClient::name(),
                        ConsoleCmd::GetTemp(params[1].to_owned()),
                    )
                    .unwrap();
                    self.send_service_cmd(
                        UdpClient::name(),
                        ConsoleCmd::GetTemp(params[1].to_owned()),
                    )
                    .unwrap();
                }

                EXIT => {
                    if let Err(e) = self.send_service_cmd(TcpClient::name(), ConsoleCmd::Exit) {
                        error!("Can't stop tcp client: {e}");
                    }
                    if let Err(e) = self.send_service_cmd(UdpClient::name(), ConsoleCmd::Exit) {
                        error!("Can't stop udp server: {e}");
                    }
                    info!("Exit from emulator");
                    println!("Exit from emulator");
                    break;
                }
                _ => {
                    println!("Unexpected command");
                    help();
                }
            }
        }
        self.join();
        println!("All services stopped");
    }
}
