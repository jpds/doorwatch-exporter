//
// doorwatch-exporter
//
// Copyright 2022 Jonathan Davies <jd@upthedownstair.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use prometheus_exporter::prometheus::{register_counter, register_int_gauge, register_gauge};
use rust_gpiozero::input_devices::DigitalInputDevice;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "doorwatch-exporter")]
struct Opt {
    #[structopt(long)]
    gpio_pin: u8,

    #[structopt(
        long,
        help = "GPIO polling interval in milliseconds",
        default_value = "500"
    )]
    poll_interval: u64,

    #[structopt(long, default_value = "9184")]
    port: u16,
}

fn main() {
    let opt = Opt::from_args();
    let device = DigitalInputDevice::new_with_pullup(opt.gpio_pin);
    let addr: SocketAddr = format!("[::]:{}", opt.port)
        .parse()
        .expect("Unable to parse socket address");
    let exporter = prometheus_exporter::start(addr).expect("Unable to start exporter");

    println!("Starting doorwatch-exporter server at {}", addr);

    let doorwatch_gpio_pin_metric = register_gauge!(
        "doorwatch_gpio_pin",
        "Number of GPIO PIN monitored by doorwatch"
    )
    .expect("Unable to create gauge doorwatch_gpio_pin");
    let doorwatch_gpio_value_metric = register_int_gauge!(
        "doorwatch_gpio_value",
        "Value of GPIO PIN monitored by doorwatch"
    )
    .expect("Unable to create gauge doorwatch_gpio_value");
    let doorwatch_last_observed_opening_timestamp_metric = register_gauge!(
        "doorwatch_last_observed_opening_timestamp_seconds",
        "Timestamp in seconds when doorwatch last observed a door opening"
    )
    .expect("Unable to create gauge doorwatch_last_observed_opening_seconds");
    let doorwatch_last_poll_timestamp_metric = register_gauge!(
        "doorwatch_last_poll_timestamp_seconds",
        "Timestamp in seconds when doorwatch last polled GPIO"
    )
    .expect("Unable to create gauge doorwatch_last_poll_timestamp_seconds");
    let doorwatch_opened_seconds_metric = register_counter!(
        "doorwatch_opened_seconds_total",
        "Number of seconds door is detected to be opened"
    )
    .expect("Unable to create counter door_opened_seconds_total");
    let doorwatch_poll_interval_metric = register_gauge!(
        "doorwatch_poll_interval_seconds",
        "Number of seconds doorwatch is set to poll at"
    )
    .expect("Unable to create gauge doorwatch_poll_interval_seconds");

    doorwatch_gpio_pin_metric.set(opt.gpio_pin as f64);
    doorwatch_poll_interval_metric.set(opt.poll_interval as f64 / 1000.0);

    loop {
        let now_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        if !device.value() {
            doorwatch_gpio_value_metric.set(0);
            doorwatch_opened_seconds_metric.inc_by(opt.poll_interval as f64 / 1000.0);
            doorwatch_last_observed_opening_timestamp_metric.set(now_timestamp.as_millis() as f64 / 1000.0);
        } else {
            doorwatch_gpio_value_metric.set(1);
        }

        doorwatch_last_poll_timestamp_metric.set(now_timestamp.as_millis() as f64 / 1000.0);

        let _guard = exporter.wait_duration(std::time::Duration::from_millis(opt.poll_interval));
    }
}
