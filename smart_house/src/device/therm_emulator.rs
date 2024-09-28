use tokio;
use tokio::net::UdpSocket;
use log::*;
use crate::transport_layer::{TranportPack, TypePack};
use bincode;
use crate::protocol;
use crate::task;
use rand::prelude::{thread_rng, Rng};
use rand_distr::StandardNormal;

const AVG_TEMP: f64 = 25.0; // C
const TEMP_SPREAD: f64 = 5.0; // 100W

pub struct ThermEmulator {
    ip_addr: String,
    is_turned_on: bool,
}

impl task::Start for ThermEmulator {
     async fn start(mut self) {
        let udp_sock = 
        match UdpSocket::bind(&self.ip_addr).await{
            Ok(res) => res,
            Err(e) => {
                error!("ThermEmulator: can't bind to addr: {}, reason: {:?}", self.ip_addr, e);
                return;
            }
        };

        loop {
            let mut bin_pack = vec![0u8; 1500];
            let (pack_size, remote_addr) = 
            match udp_sock.recv_from(&mut bin_pack).await{
                Ok(res) => res,
                Err(e) => {
                    error!("Error recv udp datagram: {:?}", e);
                    break;
                }
            };

            bin_pack.shrink_to(pack_size);

            let pack = 
            match TranportPack::deserialize(&mut bin_pack){
                Ok(pack) => pack,
                Err(e) => {
                    info!("Connection deserialize pack: {:?}", e);
                    continue;
                }
            };
            let payload = pack.into_payload();
            let req: protocol::Request = 
            match bincode::deserialize(&payload){
                Ok(val) => val,
                Err(e) => {
                    info!("Invalid request protocol: {:?}", e);
                    continue;
                }
            };

            if req.addr != self.ip_addr {
                info!("Invalid address: self: {} but received: {}", self.ip_addr, req.addr);
                continue;
            }

            let resp = 
            match req.cmd{
                protocol::Cmd::TurnOn => {
                    self.turn_on();
                    protocol::Response::success_response(req)
                }
                protocol::Cmd::TurnOff => {
                    self.turn_off();
                    protocol::Response::success_response(req)
                }
                protocol::Cmd::Power => {
                    let power = self.get_temperature();
                    protocol::Response::current_temp(req, power)
                }
                _ => {
                    info!("Unsupported command for smart thermometer {:?}", req.cmd);
                    protocol::Response::err_response(req, protocol::ErrorKind::UnknownCmd)
                }
            };

            let bin_resp = match bincode::serialize(&resp) {
                Ok(val) => val,
                Err(e) => {
                    error!("Can't serialize response: {:?}", e);
                    break;
                }
            };
            let bin_pack = TranportPack::new(TypePack::Simple, bin_resp).serialize();
            if let Err(e) = udp_sock.send_to(&bin_pack, remote_addr).await {
                error!("Internal error: {:?}", e);
                break;
            }
        }
    }
}

impl ThermEmulator {
    pub fn new(ip_addr: &str) -> ThermEmulator{
        Self {
            ip_addr: ip_addr.to_owned(),
            is_turned_on: true,
        }
    }

    fn turn_on(&mut self){
        info!("Thermometer is turned on");
        self.is_turned_on = true;
    }

    fn turn_off(&mut self){
        info!("Thermometer is turned off");
        self.is_turned_on = false;
    }

    fn get_temperature(&self) -> f64 {
        if !self.is_turned_on {
            return 0.0;
        }

        let noize = thread_rng().sample::<f64, StandardNormal>(StandardNormal) - 0.5;
        let scalied_noize = noize * TEMP_SPREAD as f64;
        let res = AVG_TEMP + scalied_noize;
        res
    }
}