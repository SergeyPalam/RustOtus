use crate::err_house;
use crate::socket_protocol::*;
use crate::transport_layer::*;
use rand::{prelude::ThreadRng, Rng};
use rand_distr::StandardNormal;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

const AVG_POWER: i32 = 4_000_000; // 4 kW
const POWER_SPREAD: i32 = 100_000; // 100W
const BIND_ADDR: &str = "127.0.0.1:80";

#[derive(Default)]
pub struct SocketEmulator {
    rng: ThreadRng,
    is_turn_on: bool,
}

impl SocketEmulator {
    fn handle_request(&mut self, connection: &mut TcpStream) -> Result<(), err_house::Err> {
        let pack = TranportPack::from_reader(connection)?;
        let req = SockRequest::deserialize(pack.get_payload())?;

        match req.get_req_type() {
            ReqType::TurnOn => {
                println!("Socket is turned on");
                self.turn_on();
                let resp = SockResponse::new(RespType::Success, ReqType::TurnOn, Vec::new()).serialize();
                let pack = TranportPack::new(TypePack::Simple, resp.into()).serialize();
                connection.write_all(&pack)?;
            }
            ReqType::TurnOff => {
                println!("Socket is turned off");
                self.turn_off();
                let resp = SockResponse::new(RespType::Success, ReqType::TurnOff, Vec::new()).serialize();
                let pack = TranportPack::new(TypePack::Simple, resp.into()).serialize();
                connection.write_all(&pack)?;
            }
            ReqType::CheckTurned => {
                println!("Socket is turned: {}", self.is_turn_on);
                let flag = if self.is_turn_on { 1 } else { 0 };
                let resp = SockResponse::new(RespType::Success, ReqType::CheckTurned, vec![flag]).serialize();
                let pack = TranportPack::new(TypePack::Simple, resp.into()).serialize();
                connection.write_all(&pack)?;
            }
            ReqType::Power => {
                let cur_pow = self.get_power_mw();
                println!("Socket current power Wt is: {}", cur_pow as f64 / 1000.0);
                let serialized_pow = cur_pow.to_be_bytes().to_vec();
                let resp = SockResponse::new(RespType::Success, ReqType::Power, serialized_pow).serialize();
                let pack = TranportPack::new(TypePack::Simple, resp).serialize();
                connection.write_all(&pack)?;
            }
            ReqType::Unknown(val) => {
                println!("Unknown request: {val}");
            }
        }

        Result::Ok(())
    }

    pub fn start_server(&mut self) {
        let listener = TcpListener::bind(BIND_ADDR).expect("Can't bind listener");
        loop {
            let (mut connection, addr) = listener.accept().expect("Can't accept connection");
            println!("Accept new connection at addr: {addr}");
            loop {
                if let Err(e) = self.handle_request(&mut connection) {
                    println!("Can't handle request: {} from {}", e, addr);
                    break;
                } else {
                    println!("Connection handled");
                }
            }
        }
    }

    fn turn_on(&mut self) {
        println!("Socket is turned on");
        self.is_turn_on = true;
    }

    fn turn_off(&mut self) {
        println!("Socket is turned off");
        self.is_turn_on = false;
    }

    fn get_power_mw(&mut self) -> u32 {
        if !self.is_turn_on {
            return 0;
        }

        let noize = self.rng.sample::<f64, StandardNormal>(StandardNormal) - 0.5;
        let scalied_noize = (noize * POWER_SPREAD as f64).round() as i32;
        let res = (AVG_POWER + scalied_noize) as u32;
        println!("Current power: {} Wt", res / 1000);
        res
    }
}
