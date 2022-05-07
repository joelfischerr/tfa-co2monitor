//! A crate for reading co2 concentration from a Dostmann CO2-Sensor
//! and writing it to influxdb.
extern crate hidapi;
extern crate reqwest;

mod zytemp;
mod influxudp;

use zytemp::Reading;
use std::collections::HashMap;
use reqwest::header;

const co2WarnLevel : u16 = 500;
const temperatureWarnLevel : f32 = 22.0;

fn main() {
    let api: hidapi::HidApi = hidapi::HidApi::new().unwrap();
    let mut device = zytemp::initialize(&api);

    loop {
        let reading = zytemp::read_data(&mut device);
        println!("{:?}", reading);
        checkValues(reading)
    }
}

fn checkValues(reading : Reading) {
    match reading {
      Reading::CO2(v) => { 
        if v > co2WarnLevel {
          sendCo2Warn(v)
        }
      }
      Reading::Temperature(v) => {
        if v > temperatureWarnLevel {
          send_temp_warn(v)
        }
      }
    }
}

fn sendCo2Warn(level : u16) {
  let newString = ("🔴🔴🔴 CO2 Level is High ({})", level);

  let mut map = HashMap::new();
  map.insert("type", "text");
  map.insert("text", "");

  let mut headers = header::HeaderMap::new();
  headers.insert("X-HEADER-1", header::HeaderValue::from_static("val1"));

  let client = reqwest::blocking::Client::new();
  let res = client.post("http://httpbin.org/post")
    .body("the exact body that is sent")
    .send();
}

fn send_temp_warn(level : f32) {

}

// /// Send the reading to influxdb via UDP
// ///
// /// This is pretty minimalistic and supports no security whatsoever, so this
// /// is only a sane thing to do if influx is running on the same host 
// fn send_to_influxdb(socket: &UdpSocket, reading: Reading) {
//     let field;
//     let value;

//     match reading {
//         Reading::CO2(v) => { field="CO2"; value=v as f32; },
//         Reading::Temperature(v) => { field="temperature"; value=v; },
//     }

//     let line = WireLine {measurement: "climate", field: field, value: value };
//     if influxudp::send(&socket, line).is_err() {
//         println!("Failed to send measurement to Influx");
//     }
// }
