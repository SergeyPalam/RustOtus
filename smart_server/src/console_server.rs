use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use log::*;

use super::err_house;
use super::smart_house_tcp_server::TcpServer;
use super::smart_house_udp_server::UdpServer;

const EXIT: &str = "exit";

#[derive(Clone, Copy)]
pub enum ConsoleCmd {
    Exit,
}

fn help() {
    println!("Type \"exit\" to exit from emulator");
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
        Self {
            thread_handle,
            tx,
        }
    }
}

pub struct ConsoleServer {
    channels: HashMap<&'static str, Channel>,
}

impl ConsoleServer {
    fn connect_to_service(&mut self, service: impl Service, serv_name: &'static str){
        let (tx, rx) = mpsc::channel();
        let thread_handle = service.start_service(rx);
        let channel = Channel::new(thread_handle, tx);
        let entry = self.channels.entry(serv_name);
        match entry{
            Entry::Occupied(_) => {
                error!("Service {serv_name} already connected");
            }
            Entry::Vacant(e) => {
                e.insert(channel);
                log::info!("Service: {serv_name} connected");
            },
        }
    }

    fn join(self){
        for (_, channel) in self.channels{
            if let Err(e) = channel.thread_handle.join(){
                error!("Can't join finished thread: {:?}", e);
                panic!();
            }
        }
    }

    fn send_service_cmd(&mut self, service_name: &str, cmd: ConsoleCmd) -> Result<(), err_house::Err> {
        if let Some(channel) = self.channels.get(service_name) {
            if let Err(_) = channel.tx.send(cmd){
                error!("Service: {service_name} isn't responding");
                return Err(err_house::Err::new(err_house::ErrorKind::ServiceNotRespond));
            }
        }else{
            error!("Unknown service: {service_name}");
            return Err(err_house::Err::new(err_house::ErrorKind::UnknownService));
        }
        Ok(())
    }

    pub fn new () -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn start(mut self) {
        println!("Start server");
        help();
        let std_in = io::stdin();
        let tcp_server = TcpServer::new(Path::new("Config.txt"));
        let udp_server = UdpServer::new(Path::new("Config.txt"));

        self.connect_to_service(tcp_server, TcpServer::name());
        self.connect_to_service(udp_server, UdpServer::name());

        for line in std_in.lock().lines(){
            let cmd =
            match line {
                Ok(res) => res,
                Err(_) => {
                    error!("IO error");
                    panic!();
                }
            };
            match cmd.as_str() {
                EXIT => {
                    if let Err(e) = self.send_service_cmd(TcpServer::name(), ConsoleCmd::Exit){
                        error!("Can't stop tcp server: {e}");
                        panic!();
                    }
                    if let Err(e) = self.send_service_cmd(UdpServer::name(), ConsoleCmd::Exit){
                        error!("Can't stop udp server: {e}");
                        panic!();
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