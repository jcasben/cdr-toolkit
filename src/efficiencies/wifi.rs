use colored::Colorize;
use plotters::prelude::*;

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

/// Creates a plot of the efficiency of a WiFi network.
/// # Arguments
/// * `payload` - the payload size in bits.
/// * `stations` - the number of stations in the network.
fn wifi_efficiency(payload: i32, stations: i32) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Efficiency of a payload range", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0 as f32..(payload + 1000) as f32, 0f32..1f32)?;

    chart.configure_mesh()
        .x_desc("Payload size (bits)")
        .y_desc("Efficiency")
        .axis_desc_style(("sans-serif", 15).into_font())
        .draw()?;

    let efficiency_values: Vec<(f32, f32)> = (0..payload)
        .map(|x| (x as f32, efficiency(x as f32, stations)))
        .collect();

    chart.draw_series(LineSeries::new(efficiency_values, &RED))?;

    Ok(())
}

/// Calculates the efficiency of a WiFi network.
fn efficiency(x: f32, sta: i32) -> f32 {
    (x / 11.0) / (ts(x) - tc(x) + (SIGMA * (1.0 - ptr(x, sta)) / ptr(x, sta) + tc(x)) / ps(x, sta))
}

/// Calculates the value of ts, which is the time to send a packet
fn ts(x: f32) -> f32 {
    TOTALH + x / 11.0 + SIFS + DELTA + ACK + DIFS + DELTA
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
/// wifi_efficiency(). It prints an error if there was any.
pub fn input_wifi_efficiency() {
    let payload = parse_user_input("Enter the payload size (in bits): ");
    let stations = parse_user_input("Enter the number of stations: ");

    if payload.is_ok() && stations.is_ok() {
        wifi_efficiency(payload.unwrap() as i32, stations.unwrap() as i32).unwrap();
    } else {
        println!("{}{}", "ERROR:".red(), "Invalid input".red());
    }
}