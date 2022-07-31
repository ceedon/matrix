use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Scd30Reading {
    co2_ppm: f32,
    humidity_relative: f32,
    temp_in_f: f32,
}

#[derive(Debug, Serialize, Deserialize)]

struct Pmsa003iReading {
    pm1: u16,
    pm25: u16,
    pm10: u16,
    um_pt3: u16,
    um_pt5: u16,
    um_1: u16,
    um_2pt5: u16,
    um_5: u16,
    um_10: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct AdafruitIoMqttMsg {
    value: (Scd30Reading, Pmsa003iReading),
    lat: f32,
    lon: f32,
    ele: f32,
}

/// Converts Celsius to Farenheit
fn c_to_f(t: f32) -> f32 {
    return t * 9.0 / 5.0 + 32.0;
}

fn read_float(bytes: [u8; 4]) -> f32 {
    f32::from_be_bytes(bytes)
}

fn read_int(bytes: [u8; 2]) -> u16 {
    u16::from_be_bytes(bytes)
}

/// Polls the status of the SCD30 to ensure valid data.
fn scd30_is_ready(i2c: &mut I2c) -> bool {
    let i2c_addr: u16 = 0x61;
    let status_check: [u8; 2] = [0x02, 0x02];
    let mut sensor_ready = [0u8; 2];
    i2c.set_slave_address(i2c_addr).unwrap();
    i2c.write(&status_check).unwrap();
    i2c.read(&mut sensor_ready).unwrap();
    return if sensor_ready[1] == 1 { true } else { false };
}

/// Tells the sensor to start continuous data collection.
fn scd30_startup(i2c: &mut I2c) -> () {
    let i2c_addr: u16 = 0x61;
    let start_monitoring_cmd: [u8; 5] = [0x00, 0x10, 0x00, 0x00, 0x81];
    i2c.set_slave_address(i2c_addr).unwrap();
    i2c.write(&start_monitoring_cmd).unwrap();
}

fn read_scd30(i2c: &mut I2c) -> [u8; 18] {
    let mut regs = [0u8; 18];
    let read_cmd: [u8; 2] = [0x03, 0x00];
    i2c.write(&read_cmd).unwrap();
    i2c.read(&mut regs).unwrap();
    return regs;
}

fn read_pmsa003i(i2c: &mut I2c) -> [u8; 32] {
    let i2c_addr: u16 = 0x12;
    let mut regs = [0u8; 32];
    i2c.set_slave_address(i2c_addr).unwrap();
    i2c.block_read(0x00 as u8, &mut regs).unwrap();
    return regs;
}

fn parse_scd30(b: [u8; 18]) -> Scd30Reading {
    let co2 = read_float([b[0], b[1], b[3], b[4]]);
    let temp_c = read_float([b[6], b[7], b[9], b[10]]);
    let hum = read_float([b[12], b[13], b[15], b[16]]);
    Scd30Reading {
        co2_ppm: co2,
        humidity_relative: hum,
        temp_in_f: c_to_f(temp_c),
    }
}

fn parse_pmsa003i(b: [u8; 32]) -> Pmsa003iReading {
    let pm1 = read_int([b[0x0a], b[0x0b]]);
    let pm25 = read_int([b[0x0c], b[0x0d]]);
    let pm10 = read_int([b[0x0e], b[0x0f]]);
    let um_pt3 = read_int([b[0x10], b[0x11]]);
    let um_pt5 = read_int([b[0x12], b[0x13]]);
    let um_1 = read_int([b[0x14], b[0x15]]);
    let um_2pt5 = read_int([b[0x16], b[0x17]]);
    let um_5 = read_int([b[0x18], b[0x19]]);
    let um_10 = read_int([b[0x1a], b[0x1b]]);
    Pmsa003iReading {
        pm1,
        pm25,
        pm10,
        um_pt3,
        um_pt5,
        um_1,
        um_2pt5,
        um_5,
        um_10,
    }
}

fn build_mqtt_msg(s: Scd30Reading, p: Pmsa003iReading) -> AdafruitIoMqttMsg {
    // Geo info for 632 S Eugenia
    let lon: f32 = -84.4542467999;
    let lat: f32 = 33.7727154;
    let ele: f32 = 268.31;
    AdafruitIoMqttMsg {
        value: (s, p),
        lat,
        lon,
        ele,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;

    scd30_startup(&mut i2c);

    loop {
        if !scd30_is_ready(&mut i2c) {
            continue;
        }
        let scd30 = read_scd30(&mut i2c);
        let scd30 = parse_scd30(scd30);

        let pmsa003i = read_pmsa003i(&mut i2c);
        let pmsa003i = parse_pmsa003i(pmsa003i);

        let msg = build_mqtt_msg(scd30, pmsa003i);
        let serialized = serde_json::to_string_pretty(&msg).unwrap();
        println!("{}", serialized);
        thread::sleep(Duration::from_secs(30));
    }
}
