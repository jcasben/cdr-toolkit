use colored::Colorize;

use crate::parse_user_input;

/// Calculates the efficiency of an Ethernet network.
/// # Arguments
/// * `N` - number of stations in the network.
/// * `L` - the length of the frame in bytes.
/// * `tb` - 512 in Ethernet 10BASE-T and 100BASE-T4, 4096 in Ethernet 1000BASE-T.
/// 
/// # Returns
/// The value of the efficiency of the network. 
fn ethernet_efficiency(n: i32, l: i32, tb: i32) -> f32 {
    let a: f32 = (1.0 - (1.0 / n as f32)).powi(n - 1);
    1.0 / (1.0 + (tb as f32 / l as f32 * 8.0 * a))
}

/// Takes the user input for the required values and calls
/// ethernet_efficiency(). It prints the result if the process
/// was successful or an error if there was any.
pub fn input_ethernet_efficiency() {
    let n = parse_user_input("Enter the number of stations: ");
    let l = parse_user_input("Enter the length of the frame in bytes: ");
    let tb = parse_user_input("Enter the value for tb (512 in Ethernet 10BASE-T and 100BASE-T4, 4096 in Ethernet 1000BASE-T): ");

    if n.is_ok() && l.is_ok() && tb.is_ok() {
        let efficiency = ethernet_efficiency(n.unwrap() as i32, l.unwrap() as i32, tb.unwrap() as i32);
        println!("{}{}", "Ethernet Efficiency = ".blue(), efficiency.to_string().blue());
    } else {
        eprintln!("\n{}{}", "ERROR: ".red(), "Couldn't parse the user input to a numeric value".red());
    }
}