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
