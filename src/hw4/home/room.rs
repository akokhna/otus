use std::collections::HashSet;

pub struct Room {
    name: String,
    devices: HashSet<String>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            name,
            devices: HashSet::new(),
        }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn add_device(&mut self, device: &str) {
        self.devices.insert(String::from(device));
    }
    pub fn get_devices(&self) -> &HashSet<String> {
        &self.devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_device_to_room() {
        let mut room = Room::new(String::from("Test Room"));
        room.add_device("test_device");
        assert!(room.get_devices().contains("test_device"));
    }
}
