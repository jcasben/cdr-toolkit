use colored::Colorize;

pub mod flow_control;
pub mod error_control;
pub mod wifi;
pub mod ethernet;

/// Calculates the value of a, which formula is tprop/tframe.
/// It is used for all the efficiency calculations.
fn calculate_a(tprop: f32, tframe: f32) -> f32 {
    let a: f32 = tprop / tframe;
    println!("\n{}{}", "a = ".blue(), a.to_string().blue());

    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculate_a_test() {
        let tprop: f32 = 4672.89;
        let tframe: f32 = 78.4;
        assert_eq!(59.603188, super::calculate_a(tprop, tframe));
    }
}