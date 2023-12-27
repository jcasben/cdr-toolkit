use std::io::{self, Write};
use colored::Colorize;

use crate::parse_user_input;

// SIFS in microseconds
const SIFS: f32 = 10.0;
// DIFS in microseconds
const DIFS: f32 = 50.0;
// slot time in microseconds
const SIGMA: f32 = 20.0;
// propagation time, typically very small
const DELTA: f32 = 0.0;
// physical header in microseconds
const PHYH: f32 = 72.0 + 24.0;
// MAC header in IEEE 802.11, in microseconds
const MACH: f32 = 272.0 / 11.0;
// ACK duration in microseconds (112 bits at 11 Mbps + physical header duration)
const ACK: f32 = 112.0 / 11.0 + PHYH;
// Equivalent duration of MAC Header + PHY Header
const TOTALH: f32 = PHYH + MACH;

pub fn wifi_menu() {
    const MENU: &str =
    r#"********** WiFi CALCULATIONS **********
*   1 - WiFi Efficiency                 *
*   2 - Time to transmit                *
*   b - Back                            *
*****************************************"#;

    'wifi_loop: loop {
        let mut option: String = String::new();
        println!("\n\n");
        println!("{}", MENU.bright_cyan());
        print!("\n{}", "Choose what you want to do: ".bright_cyan());
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut option)
            .expect("ERROR: Failed to read user input.");

        match option.as_str().trim() {
            "b" => break 'wifi_loop,
            "1" => input_wifi_efficiency(),
            "2" => input_ts(),
            _ => eprintln!("\n{}", "ERROR: The chosen option does not exist within the possible options".red()),
        }
    }
}

/// Creates a plot of the efficiency of a WiFi network.
/// # Arguments
/// * `payload` - the payload size in bits.
/// * `stations` - the number of stations in the network.
/// 
/// # Returns
/// The value of the efficiency of the WiFi network.
fn wifi_efficiency(payload: f32, stations: i32) -> f32 {
    (payload / 11.0) / (ts(payload) - tc(payload) + (SIGMA * (1.0 - ptr(payload, stations)) / ptr(payload, stations) + tc(payload)) / ps(payload, stations))
}

/// Calculates the value of ts, which is the time to send a packet
/// in a WiFi network.
/// # Arguments
/// * `payload` - the payload size in bits.
/// 
/// # Returns
/// The value of ts.
fn ts(payload: f32) -> f32 {
    TOTALH + payload / 11.0 + SIFS + DELTA + ACK + DIFS + DELTA
}

/// Calculates the value of tc, which is the time to transmit a packet
fn tc(x: f32) -> f32 {
    TOTALH + x / 11.0 + DIFS + DELTA
}

/// Calculates the value of thau, which is the probability of a collision
fn thau(x: f32, sta: i32) -> f32 {
    1.0 / (sta as f32 * (tc(x) / (2.0 * SIGMA)).sqrt())
}

/// Calculates the value of ptr, which is the probability of a transmission
fn ptr(x: f32, sta: i32) -> f32 {
    1.0 - (1.0 - thau(x, sta)).powi(sta)
}

/// Calculates the value of ps, which is the probability of a successful transmission
fn ps(x: f32, sta: i32) -> f32 {
    sta as f32 * thau(x, sta) * (1.0 - thau(x, sta)).powi(sta - 1) / ptr(x, sta)
}

/// Takes the user input for the required values and calls
/// ts(). It prints an error if there was any.
pub fn input_ts() {
    let payload = parse_user_input("Enter the payload size (in bits): ");
    if payload.is_ok() {
        let time_to_send = ts(payload.clone().unwrap());
        println!("\n{}{}", "Time to send a packet = ".green(), time_to_send.to_string().green());
    } else {
        println!("{}{}", "ERROR:".red(), "Couldn't parse the user input".red());
    }
}

/// Takes the user input for the required values and calls
/// wifi_efficiency(). It prints an error if there was any.
pub fn input_wifi_efficiency() {
    let payload = parse_user_input("Enter the payload size (in bits): ");
    let stations = parse_user_input("Enter the number of stations: ");

    if payload.is_ok() && stations.is_ok() {
        let eff = wifi_efficiency(payload.clone().unwrap(), stations.clone().unwrap() as i32);
        println!("\n{}{}", "WiFi Efficiency = ".green(), eff.to_string().green());
    } else {
        println!("{}{}", "ERROR:".red(), "Couldn't parse the user input".red());
    }
}