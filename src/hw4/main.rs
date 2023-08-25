use crate::hw4::device::{SmartSocket, SmartThermometer};
use crate::hw4::home::room::Room;
use crate::hw4::home::SmartHouse;
use crate::hw4::provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_report() {
        let mut smart_house = SmartHouse::new(String::from("Test House"));

        let mut living_room = Room::new(String::from("Living Room"));
        living_room.add_device("sm_socket_1");
        smart_house.add_room(living_room);

        let socket1 = SmartSocket::new(String::from("sm_socket_1"), false);
        let info_provider = OwningDeviceInfoProvider { socket: socket1 };
        let report = smart_house.create_report(&info_provider);

        assert!(report.contains("House name is : Test House"));
        assert!(report.contains("Room: Living Room"));
        assert!(report.contains("Socket: sm_socket_1 and status is false"));
    }

    // Add more tests for other functions and structures
}
