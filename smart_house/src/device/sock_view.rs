use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{OwnedWriteHalf, OwnedReadHalf};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};
use log::*;
use crate::{protocol, err_house, transport_layer::{TranportPack, TypePack}};

use bincode;
pub struct SockView {
    tx_sock: OwnedWriteHalf,
}

impl SockView {
    pub async fn connect(ip_addr: &str, cnt_connect_attempts: usize) -> Result<(Self, OwnedReadHalf), err_house::Err> {
        let mut tcp_stream = None;
        for _ in 0..cnt_connect_attempts{
            match TcpStream::connect(ip_addr).await {
                Ok(stream) => {
                    tcp_stream = Some(stream);
                    break;
                },
                Err(_) => sleep(Duration::from_millis(10)).await,
            }
        }

        let tcp_stream =
        match tcp_stream {
            Some(stream) => stream,
            None => {
                info!("Can't connect to {ip_addr}");
                return Err(err_house::Err::new(err_house::ErrorKind::IoTimeOut));
            }
        };

        let (rx_sock, tx_sock) = tcp_stream.into_split();
        let sock_view = Self {
            tx_sock,
        };

        Ok((sock_view, rx_sock))
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
        let res = self.tx_sock.write_all(&bin_pack).await?;
        Ok(res)
    }
}

