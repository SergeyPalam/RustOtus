use tokio::net::UdpSocket;
use tokio;
use crate::transport_layer::TranportPack;
use crate::task;
use log::*;
use crate::protocol;
use std::sync::Arc;
use bincode;

pub struct ThermHandler{
    rx_sock: Arc<UdpSocket>,
}

impl task::Start for ThermHandler {
    async fn start(self) {
        loop{
            let mut buf = vec![0u8; 1500];
            let size =
            match self.rx_sock.recv(&mut buf).await{
                Ok(res) => res,
                Err(e) => {
                    error!("Internal error: {:?}", e);
                    break;
                }
            };

            buf.shrink_to(size);
            let pack =
            match TranportPack::deserialize(&buf){
                Ok(val) => val,
                Err(_) => {
                    info!("Invalid packet");
                    continue;
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

impl ThermHandler{
    pub fn new(rx_sock: Arc<UdpSocket>) -> Self {
        Self {
            rx_sock,
        }
    }

    fn handle_response(&self, resp: &protocol::Response){
        println!("{}", resp);
    }
}