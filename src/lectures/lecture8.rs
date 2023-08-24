use std::collections::{HashMap, HashSet};

struct Room {
    name: String,
    devices: HashSet<String>,
}

impl Room {
    fn new(name: String) -> Self {
        Room {
            name,
            devices: HashSet::new(),
        }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }
    fn add_device(&mut self, device: &str) {
        self.devices.insert(String::from(device));
    }
}

struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    fn new(name: String) -> Self {
        SmartHouse {
            name,
            rooms: HashMap::new(),
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn _get_rooms(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    fn add_room(&mut self, room: Room) -> Option<bool> {
        if !self.rooms.contains_key(room.get_name()) {
            self.rooms.insert(String::from(room.get_name()), room);
            return Some(true);
        }

        None
    }

    fn get_devices_in_room(&self, room_name: &str) -> Option<&HashSet<String>> {
        if let Some(room) = self.rooms.get(room_name) {
            Some(&room.devices)
        } else {
            None
        }
    }

    fn get_all_devices(&self) -> HashMap<String, HashSet<String>> {
        let mut all_devices_by_room = HashMap::new();

        for (room_name, room) in &self.rooms {
            all_devices_by_room.insert(room_name.clone(), room.devices.clone());
        }

        all_devices_by_room
    }

    fn create_report(&self, info: &impl DeviceInfoProvider) -> String {
        let mut report = String::new();

        report.push('\n');
        report.push_str(&format!("House name is : {}\n", self.get_name()));
        report.push('\n');

        let mut any_device_found = false;

        for (room_name, room) in &self.rooms {
            report.push_str(&format!("Room: {}\n", room_name));

            if let Some(devices_in_room) = self.get_devices_in_room(room_name) {
                if !devices_in_room.is_empty() {
                    any_device_found = true;
                    let devices_str = devices_in_room
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    report.push_str(&format!("Devices: {}\n", devices_str));
                }
            }

            for device_name in &room.devices {
                if let Some(device_info) = info.get_info(room_name, device_name) {
                    any_device_found = true;
                    report.push_str(&format!("  {}\n", device_info));
                }
            }

            report.push('\n');
        }

        if !any_device_found {
            report.push_str("No devices found in any room.\n");
        }

        report
    }
}

pub trait Device {
    fn get_name(&self) -> &str;
    fn report(&self) -> String;
}

// Пользовательские устройства:
pub struct SmartSocket {
    name: String,
    enable: bool,
}

impl SmartSocket {
    pub fn new(name: String, enable: bool) -> Self {
        SmartSocket { name, enable }
    }
}

impl Device for SmartSocket {
    fn report(&self) -> String {
        format!("Socket: {} and status is {:?}", self.name, self.enable)
    }

    fn get_name(&self) -> &str {
        self.name.as_ref()
    }
}

pub struct SmartThermometer {
    name: String,
    enable: bool,
}

impl SmartThermometer {
    pub fn new(name: String, enable: bool) -> Self {
        SmartThermometer { name, enable }
    }
}

impl Device for SmartThermometer {
    fn report(&self) -> String {
        format!("Thermometer: {} and status is {}", self.name, self.enable)
    }

    fn get_name(&self) -> &str {
        self.name.as_ref()
    }
}

pub trait DeviceInfoProvider {
    fn get_info(&self, room: &str, device: &str) -> Option<String>;
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_info(&self, room: &str, device: &str) -> Option<String> {
        let devices: HashMap<&str, &dyn Device> =
            HashMap::from([(self.socket.get_name(), &self.socket as &dyn Device)]);

        get_full_device_info(room, device, devices)
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_info(&self, room: &str, device: &str) -> Option<String> {
        let devices: HashMap<&str, &dyn Device> = HashMap::from([
            (self.socket.get_name(), self.socket as &dyn Device),
            (self.thermo.get_name(), self.thermo as &dyn Device),
        ]);

        get_full_device_info(room, device, devices)
    }
}

fn get_full_device_info(
    room: &str,
    device: &str,
    devices: HashMap<&str, &dyn Device>,
) -> Option<String> {
    let dev = devices.get(device)?;
    Some(format!(
        "Full info of device in Room {} - {}",
        room,
        dev.report()
    ))
}

pub fn main() {
    // House
    let mut sm_house = SmartHouse::new(String::from("My Smart House"));

    // Rooms
    let mut living_room = Room::new(String::from("Living Room"));
    let mut bed_room = Room::new(String::from("Bedroom"));
    let mut kitchen_room = Room::new(String::from("Kitchen"));

    // Devises
    let socket1 = SmartSocket::new(String::from("sm_socket_1"), false);
    let socket2 = SmartSocket::new(String::from("sm_socket_2"), true);
    let socket3 = SmartSocket::new(String::from("sm_socket_3"), true);

    let thermo = SmartThermometer::new(String::from("sm_termometer_1"), true);

    living_room.add_device("sm_socket_1");
    bed_room.add_device("sm_socket_2");
    kitchen_room.add_device("sm_socket_3");
    kitchen_room.add_device("sm_termometer_1");
    sm_house.add_room(living_room);
    sm_house.add_room(bed_room);
    sm_house.add_room(kitchen_room);

    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = sm_house.create_report(&info_provider_1);

    let info_provider_2: BorrowingDeviceInfoProvider<'_, '_> = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = sm_house.create_report(&info_provider_2);

    let info_provider_3 = OwningDeviceInfoProvider { socket: socket3 };
    let report3 = sm_house.create_report(&info_provider_3);

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
    println!("Report #3: {report3}");

    let all_devices_by_room = sm_house.get_all_devices();
    for (room_name, devices) in &all_devices_by_room {
        println!("Devices in {}: {:?}", room_name, devices);
    }
}
