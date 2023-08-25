use crate::hw4::device::{Device, SmartSocket, SmartThermometer};
use std::collections::HashMap;

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
