use tokio::net::UdpSocket;
use std::sync::Arc;
use log::*;
use crate::{protocol, err_house, transport_layer::{TranportPack, TypePack}};

use bincode;
pub struct ThermView {
    tx_sock: Arc<UdpSocket>,
}

impl ThermView {
    pub async fn connect(ip_addr: &str) -> Result<(Self, Arc<UdpSocket>), err_house::Err> {
        let tx_udp_sock =
        Arc::new(match UdpSocket::bind("127.0.0.1:4450").await{
            Ok(sock) => sock,
            Err(e) => {
                log::error!("Can't bind therm view udp socket: {:?}", e);
                return Err(err_house::Err::new(err_house::ErrorKind::IoError));
            }
        });

        let rx_udp_sock = tx_udp_sock.clone();
        if let Err(e) = tx_udp_sock.connect(ip_addr).await {
            log::error!("Can't connect udp socket: {:?}", e);
            return Err(err_house::Err::new(err_house::ErrorKind::IoError));
        }

        let therm_view = Self {
            tx_sock: tx_udp_sock,
        };

        Ok((therm_view, rx_udp_sock))
    }

    pub async fn send_req(&mut self, req: protocol::Request) -> Result<(), err_house::Err> {
        let bin_req =
        match bincode::serialize(&req){
            Ok(val) => val,
            Err(e) => {
                error!("Can't serialize request: {:?}", e);
                return Err(err_house::Err::new(err_house::ErrorKind::SerializationError));
            }
        };

        let bin_pack = TranportPack::new(TypePack::Simple, bin_req).serialize();
        let res = self.tx_sock.send(&bin_pack).await?;
        if res != bin_pack.len() {
            error!("Internal error");
            return Err(err_house::Err::new(err_house::ErrorKind::IoError));
        }
        Ok(())
    }
}

