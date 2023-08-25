pub mod room;

use crate::hw4::provider::DeviceInfoProvider;
use room::Room;
use std::collections::{HashMap, HashSet};

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(name: String) -> Self {
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

    pub fn add_room(&mut self, room: Room) -> Option<bool> {
        if !self.rooms.contains_key(room.get_name()) {
            self.rooms.insert(String::from(room.get_name()), room);
            return Some(true);
        }

        None
    }

    fn get_devices_in_room(&self, room_name: &str) -> Option<&HashSet<String>> {
        if let Some(room) = self.rooms.get(room_name) {
            Some(&room.get_devices())
        } else {
            None
        }
    }

    pub fn get_all_devices(&self) -> HashMap<String, HashSet<String>> {
        let mut all_devices_by_room = HashMap::new();

        for (room_name, room) in &self.rooms {
            all_devices_by_room.insert(room_name.clone(), room.get_devices().clone());
        }

        all_devices_by_room
    }

    pub fn create_report(&self, info: &impl DeviceInfoProvider) -> String {
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

            for device_name in room.get_devices() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_room_to_smart_house() {
        let mut smart_house = SmartHouse::new(String::from("Test House"));
        let room = Room::new(String::from("Test Room"));
        smart_house.add_room(room);
        assert!(smart_house.rooms.contains_key("Test Room"));
    }

    #[test]
    fn test_get_devices_in_room() {
        let mut room = Room::new(String::from("Test Room"));
        room.add_device("test_device");
        let mut smart_house = SmartHouse::new(String::from("Test House"));
        smart_house.add_room(room);

        let devices = smart_house.get_devices_in_room("Test Room");
        assert_eq!(devices.unwrap().len(), 1);
    }

    #[test]
    fn test_create_smart_house() {
        let smart_house = SmartHouse::new(String::from("My Test House"));
        assert_eq!(smart_house.get_name(), "My Test House");
        assert_eq!(smart_house._get_rooms().len(), 0);
    }
}
