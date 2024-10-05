mod err_house;
mod transport_layer;
mod protocol;
mod smart_house;
mod room;
mod device;

use log::*;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use tokio;
use std::io::Result;
use device::*;
use protocol::*;
use tokio::task::JoinSet;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use std::mem::replace;

lazy_static! {
    pub static ref DB_TASKS: Arc<RwLock<JoinSet<()>>> = Arc::new(RwLock::new(JoinSet::new()));
}

fn init_logger() {
    let logfile = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new(
            "{l}: {f} {L}\\(thread: {I}\\) {m}{n}",
        )))
        .build("log/output.txt")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

#[tokio::main]
async fn main() -> Result<()>{
    init_logger();
    info!("Start smart house app");
    let mut smart_socket = Device::new("Smart Sock", "127.0.0.1:444", DevType::Sock);
    let mut smart_therm = Device::new("Smart therm", "127.0.0.1:4444", DevType::Therm);
    if let Err(e) = smart_socket.connect(true).await{
        error!("Can't connect to remote smart socket: {:?}", e);
    }
    if let Err(e) = smart_therm.connect(true).await{
        error!("Can't connect to remote smart thermometer: {:?}", e);
    }

    let sock_req = protocol::Request::new(smart_socket.get_addr(), Cmd::Power);
    smart_socket.send_req(sock_req).await.unwrap();

    let therm_req = protocol::Request::new(smart_therm.get_addr(), Cmd::Power);
    smart_therm.send_req(therm_req).await.unwrap();
    let mut lock = DB_TASKS.write().unwrap();
    let all_tasks = replace(&mut *lock, JoinSet::new());
    all_tasks.join_all().await;
    info!("End smart house app");
    Ok(())
}
