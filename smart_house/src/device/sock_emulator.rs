use tokio::io::AsyncWriteExt;
use tokio;
use tokio::net::TcpListener;
use log::*;
use crate::transport_layer::{TranportPack, TypePack};
use bincode;
use crate::protocol;
use crate::task;
use rand::prelude::{thread_rng, Rng};
use rand_distr::StandardNormal;


const AVG_POWER: f64 = 4_000.0; // 4 kW
const POWER_SPREAD: f64 = 100.0; // 100W

pub struct SockEmulator {
    ip_addr: String,
    is_turned_on: bool,
}

impl task::Start for SockEmulator {
     async fn start(mut self){
        let listener = 
        match TcpListener::bind(&self.ip_addr).await{
            Ok(res) => res,
            Err(e) => {
                error!("SockEmulator: can't bind to addr: {}, reason: {:?}", self.ip_addr, e);
                return;
            }
        };

        loop {
            let (mut tcp_stream, remote_addr) = 
            match listener.accept().await{
                Ok(res) => res,
                Err(e) => {
                    error!("SockEmulator: can't accept new connection, reason: {:?}", e);
                    return;
                }
            };

            loop {
                let pack = 
                match TranportPack::from_reader(&mut tcp_stream).await{
                    Ok(pack) => pack,
                    Err(e) => {
                        info!("Connection at address: {remote_addr} closed {:?}", e);
                        break;
                    }
                };
                let payload = pack.into_payload();
                let req: protocol::Request = 
                match bincode::deserialize(&payload){
                    Ok(val) => val,
                    Err(e) => {
                        info!("Invalid request protocol: {:?}", e);
                        break;
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
                        let power = self.get_power();
                        protocol::Response::current_power(req, power)
                    }
                    _ => {
                        info!("Unsupported command for smart socket {:?}", req.cmd);
                        protocol::Response::err_response(req, protocol::ErrorKind::UnknownCmd)
                    }
                };

                let bin_resp = match bincode::serialize(&resp) {
                    Ok(val) => val,
                    Err(e) => {
                        error!("Can't serialize response: {:?}", e);
                        return;
                    }
                };
                let bin_pack = TranportPack::new(TypePack::Simple, bin_resp).serialize();
                if let Err(e) = tcp_stream.write_all(&bin_pack).await {
                    info!("Connection at addr: {} closed {:?}", remote_addr, e);
                    break;
                }
            }
        }
    }
}

impl SockEmulator {
    pub fn new(ip_addr: &str) -> SockEmulator{
        Self {
            ip_addr: ip_addr.to_owned(),
            is_turned_on: true,
        }
    }

    fn turn_on(&mut self){
        info!("Socket is turned on");
        self.is_turned_on = true;
    }

    fn turn_off(&mut self){
        info!("Socket is turned off");
        self.is_turned_on = false;
    }

    fn get_power(&self) -> f64 {
        if !self.is_turned_on {
            return 0.0;
        }

        let noize = thread_rng().sample::<f64, StandardNormal>(StandardNormal) - 0.5;
        let scalied_noize = noize * POWER_SPREAD as f64;
        let res = AVG_POWER + scalied_noize;
        res
    }
}