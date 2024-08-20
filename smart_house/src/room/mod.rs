use super::device::SmartDevice;
use std::collections::HashMap;

#[derive(Default)]
pub struct Room {
    devices: HashMap<String, Box<dyn SmartDevice>>,
}

impl Room {
    pub fn add_device(&mut self, dev_name: &str, device: Box<dyn SmartDevice>) {
        self.devices.insert(dev_name.to_owned(), device);
    }

    pub fn remove_device(&mut self, dev_name: &str) -> Option<(String, Box<dyn SmartDevice>)> {
        self.devices.remove_entry(dev_name)
    }

    pub fn get_device(&self, name: &str) -> Option<&dyn SmartDevice> {
        self.devices.get(name).map(|val| val.as_ref())
    }

    pub fn get_devices_names(&self) -> impl Iterator<Item = &str> {
        self.devices.keys().map(|name| name.as_str())
    }

    pub fn get_report(&self) -> String {
        let mut res = String::new();
        for (dev_name, device) in self.devices.iter() {
            let dev_report = match device.get_state() {
                Ok(state) => {
                    format!("{dev_name}: {}", state)
                }
                Err(err) => {
                    format!("{dev_name}: {}", err)
                }
            };

            res.push_str(&dev_report);
            res.push('\n');
        }
        res.pop();
        res
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read, Write};
    use std::net::TcpStream;

    use crate::device::smart_socket::*;
    use crate::device::smart_thermometer::*;
    use crate::device::socket_protocol::*;
    use crate::device::transport_layer::*;

    use super::*;

    struct TestStream {
        tx: Cursor<Vec<u8>>,
        rx: Cursor<Vec<u8>>,
    }

    impl TestStream {
        fn new(tx: Vec<u8>, rx: Vec<u8>) -> Self {
            Self {
                tx: Cursor::new(tx),
                rx: Cursor::new(rx),
            }
        }
    }

    impl Read for TestStream {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.rx.read(buf)
        }
    }

    impl Write for TestStream {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.tx.write(buf)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            self.tx.flush()
        }
    }

    #[test]
    fn test_add_device() {
        let mut room = Room::default();
        let dev1 = Box::new(SmartSocket::<TcpStream>::default());
        let dev2 = Box::new(SmartThermometer::default());
        let dev3 = Box::new(SmartSocket::<TcpStream>::default());

        room.add_device("dev1", dev1);
        room.add_device("dev2", dev2);
        room.add_device("dev2", dev3);

        let dev_names = room.get_devices_names().collect::<Vec<&str>>();
        assert_eq!(dev_names.len(), 2);

        let _ = room.get_device("dev1").unwrap();
        let _ = room.get_device("dev2").unwrap();
        let dev3 = room.get_device("dev3");
        assert!(dev3.is_none());
    }

    #[test]
    fn test_remove_device() {
        let mut room = Room::default();
        let dev1 = Box::new(SmartSocket::<TcpStream>::default());
        let dev2 = Box::new(SmartThermometer::default());
        let dev3 = Box::new(SmartSocket::<TcpStream>::default());

        room.add_device("dev1", dev1);
        room.add_device("dev2", dev2);
        room.add_device("dev3", dev3);

        let dev_names = room.get_devices_names().collect::<Vec<&str>>();
        assert_eq!(dev_names.len(), 3);

        let removed = room.remove_device("dev2").unwrap();
        assert_eq!(removed.0, "dev2");
        let dev_names = room.get_devices_names().collect::<Vec<&str>>();
        assert_eq!(dev_names.len(), 2);

        let removed = room.remove_device("dev4");
        assert!(removed.is_none());
    }

    #[test]
    fn test_get_report() {
        let sock_req = SockRequest::new_turn_on().serialize();
        let sock_resp =
            SockResponse::new(RespType::Success, ReqType::TurnOn, Vec::new()).serialize();
        let sock_pack_req = TranportPack::new(TypePack::Simple, sock_req).serialize();
        let sock_pack_resp = TranportPack::new(TypePack::Simple, sock_resp).serialize();

        let test_stream = TestStream::new(sock_pack_req, sock_pack_resp);
        let mut room = Room::default();
        let mut dev1 = Box::new(SmartSocket::<TestStream>::default());
        dev1.set_stream(test_stream);
        room.add_device("dev1", dev1);

        let room_report = room.get_report();
        assert!(!room_report.is_empty());
    }
}
