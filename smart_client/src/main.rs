mod console_server;
mod err_house;
mod protocol;
mod smart_house_tcp_client;
mod smart_house_udp_client;
mod transport_layer;

use console_server::ConsoleServer;
use log::*;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

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

fn main() {
    init_logger();
    info!("Start smart house client");
    let console_server = ConsoleServer::new();
    console_server.start();
}
