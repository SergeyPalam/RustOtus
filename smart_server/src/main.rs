mod smart_house_tcp_server;
mod smart_house_udp_server;
mod transport_layer;
mod err_house;
mod device;
mod console_server;
mod protocol;

use console_server::ConsoleServer;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log::*;

fn init_logger() {
    let logfile = FileAppender::builder().append(false)
    .encoder(Box::new(PatternEncoder::new("{l}: {f} {L}\\(thread: {I}\\) {m}{n}")))
    .build("log/output.txt").unwrap();

    let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(Root::builder()
               .appender("logfile")
               .build(LevelFilter::Debug)).unwrap();

    log4rs::init_config(config).unwrap();
}

fn main() {
    init_logger();
    info!("Start emulator smart socket");
    let console_server = ConsoleServer::new();
    console_server.start();
}
