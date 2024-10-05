use tokio::net::tcp::OwnedReadHalf;
use tokio;
use crate::transport_layer::TranportPack;
use log::*;
use crate::protocol;
use bincode;

pub struct SockHandler{
    rx_sock: OwnedReadHalf,
}

impl SockHandler{
     pub async fn start(mut self) {
        loop{
            let pack =
            match TranportPack::from_reader(&mut self.rx_sock).await{
                Ok(val) => val,
                Err(_) => {
                    error!("Invalid connection");
                    break;
                }
            };

            let bin_resp = pack.into_payload();
            let resp = 
            match bincode::deserialize(&bin_resp){
                Ok(val) => val,
                Err(e) => {
                    info!("Can't deserialize response: {:?}", e);
                    continue;
                }
            };
            self.handle_response(&resp);
        }
    }
}

impl SockHandler{
    pub fn new(rx_sock: OwnedReadHalf) -> Self {
        Self {
            rx_sock,
        }
    }

    fn handle_response(&self, resp: &protocol::Response){
        println!("{}", resp);
    }
}