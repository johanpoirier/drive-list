use std::{io};
use data::*;
use std::time::Duration;

extern crate libusb;

pub trait Platform {
    fn new() -> Self;

    /// Returns a vector of filesystem drives information objects.
    fn drives(&self) -> io::Result<Vec<Drive>>;

    fn devices(&self) {
        let context = libusb::Context::new().unwrap();

        for mut device in context.devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            match device.open() {
                Ok(handle) => {
                    let langs = handle.read_languages(Duration::new(2, 0)).unwrap();
                    let lang = langs.first().unwrap();
                    let manufacturer = match handle.read_manufacturer_string(*lang, &device_desc, Duration::new(2, 0)) {
                        Ok(manufacturer) => manufacturer,
                        Err(_e) => String::from("<?>"),
                    };
                    let serial_number = match handle.read_serial_number_string(*lang, &device_desc, Duration::new(2, 0)) {
                        Ok(serial_number) => serial_number,
                        Err(_e) => String::from("<?>"),
                    };
                    println!("USB device: {} - {}", manufacturer, serial_number);
                },
                Err(e) => println!("{}", e)
            }
        }
    }
}
