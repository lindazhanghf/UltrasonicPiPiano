extern crate i2cdev;

use std::thread;
use std::time::Duration;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const NUNCHUCK_SLAVE_ADDR: u16 = 0x52;

// real code should probably not use unwrap()
fn i2cfun() -> Result<(), LinuxI2CError> {
    let mut dev = try!(LinuxI2CDevice::new("/dev/i2c-1", NUNCHUCK_SLAVE_ADDR));

    // init sequence
    try!(dev.smbus_write_byte_data(0x39, 0x00 | 0x80, 0x03));
    try!(dev.smbus_write_byte_data(0x39, 0x01 | 0x80, 0x02));
    thread::sleep(Duration::from_millis(500));

	let mut data = bus.smbus_read_block_data(0x39, 0x0C | 0x80, 2);
	let mut data1 = bus.smbus_read_block_data(0x39, 0x0E | 0x80, 2);

	let mut ch0 = data[1] * 256 + data[0]
	let mut ch1 = data1[1] * 256 + data1[0]

	println!("Full Spectrum(IR + Visible) {} lux", ch0);
	println!("Infrared Value {} lux", ch1);
	println!("Visible Value {} lux", (ch0 - ch1));
    // loop {
    //     let mut buf: [u8; 6] = [0; 6];
    //     dev.smbus_write_byte(0x00).unwrap();
    //     thread::sleep(Duration::from_millis(10));
    //     dev.read(&mut buf).unwrap();
    //     println!("Reading: {:?}", buf);
    // }
}