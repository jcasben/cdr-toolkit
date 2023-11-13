use colored::Colorize;

pub mod flow_control;

/// Calculates the value of a, which formula is tprop/tframe.
/// It is used for all the efficiency calculations.
fn calculate_a(tprop: f32, tframe: f32) -> f32 {
    let a: f32 = tprop / tframe;
    println!("\n{}{}", "a = ".blue(), a.to_string().blue());

    a
}