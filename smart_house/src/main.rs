// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

const CNT_ROOMS: usize = 2;
const CNT_DEVICES_IN_ROOM: usize = 3;

struct Room {
    name: &'static str,
    dev_names: [&'static str; CNT_DEVICES_IN_ROOM],
}

struct SmartHouse {
    #[allow(dead_code)]
    name: &'static str,
    rooms: [Room; CNT_ROOMS],
}

enum DeviceState {
    Ok,
    #[allow(dead_code)]
    Fault(String),
}

impl DeviceState {
    fn as_string(&self) -> String {
        match self {
            DeviceState::Ok => "Ok".to_owned(),
            DeviceState::Fault(reason) => reason.to_owned(),
        }
    }
}

impl SmartHouse {
    fn new() -> Self {
        let bedroom = Room {
            name: "Bedroom",
            dev_names: ["dev1", "dev2", "dev3"],
        };

        let kitchen = Room {
            name: "Kitchen",
            dev_names: ["dev1", "dev2", "dev3"],
        };
        Self {
            name: "My house",
            rooms: [bedroom, kitchen],
        }
    }

    fn _get_rooms(&self) -> [&str; CNT_ROOMS] {
        let mut res = [""; CNT_ROOMS];
        for (i, room) in self.rooms.iter().enumerate() {
            res[i] = room.name;
        }
        res
    }

    fn _devices(&self, room: &str) -> [&str; CNT_DEVICES_IN_ROOM] {
        // linear search isn't optimized.
        for inner_room in self.rooms.iter() {
            if inner_room.name == room {
                return inner_room.dev_names;
            }
        }
        ["NotFound"; CNT_DEVICES_IN_ROOM]
    }

    fn create_report<T>(&self, device_info_provider: &T) -> String
    where
        T: DeviceInfoProvider,
    {
        let mut res = String::new();
        for room in self.rooms.iter() {
            for dev_name in room.dev_names {
                let dev_state = device_info_provider.get_dev_state(room.name, dev_name);
                res.push_str(
                    format!(
                        "Room: {}, device: {}, state: {}",
                        room.name,
                        dev_name,
                        dev_state.as_string()
                    )
                    .as_str(),
                );
                res.push('\n');
            }
        }
        res.pop();
        res
    }
}

trait DeviceInfoProvider {
    fn get_dev_state(&self, room_name: &str, dev_name: &str) -> DeviceState;
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
struct SmartSocket {}
struct SmartThermometer {}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    #[allow(dead_code)]
    socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_dev_state(&self, _room_name: &str, _dev_name: &str) -> DeviceState {
        DeviceState::Ok
    }
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    #[allow(dead_code)]
    socket: &'a SmartSocket,
    #[allow(dead_code)]
    thermo: &'b SmartThermometer,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_dev_state(&self, _room_name: &str, _dev_name: &str) -> DeviceState {
        DeviceState::Ok
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    // Инициализация дома
    let house = SmartHouse::new();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1:\n{report1}");
    println!("Report #2:\n{report2}");
}
