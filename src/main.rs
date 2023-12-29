use std::io::{self, Write};
use std::num::ParseFloatError;
use std::process::exit;
use colored::Colorize;

use crate::code_characterization::input_characterization;
//use crate::checksum::input_calculate_checksum;
use crate::efficiencies::ethernet::input_ethernet_efficiency;
use crate::efficiencies::wifi::wifi_menu;
use crate::efficiencies::{error_control::error_correction_menu, flow_control::flow_control_menu};
use crate::entropy::input_calculate_entropy;

mod entropy;
mod code_characterization;
mod efficiencies;
mod checksum;

/// Entry point of the program. Performs an infinite loop which has a menu to
/// select the option that we want to do in each iteration.
fn main() {
    const MENU: &str =
    r#"************** CDR TOOLKIT **************
*   1 - Entrophy                        *
*   2 - Code characterization           *
*   3 - Flow control efficiency         *
*   4 - Control error efficiency        *
*   5 - Ethernet efficiency             *
*   6 - WiFi Calculations               *
*   e - Exit                            *
*****************************************"#;

    loop {
        let mut option = String::new();
        println!("\n{}", MENU);
        print!("\nChoose your option: ");
        io::stdout().flush().unwrap();
        
        io::stdin()
            .read_line(&mut option)
            .expect("ERROR: couldn't read user input.");

        match option.to_lowercase().as_str().trim() {
            "e" => {
                println!("\n{}", "Thanks for using CDR TOOLKIT :)".black().on_green().bold());
                exit(0);
            },
            "1" => input_calculate_entropy(),
            "2" => input_characterization(),
            "3" => flow_control_menu(),
            "4" => error_correction_menu(),
            "5" => input_ethernet_efficiency(),
            "6" => wifi_menu(),
            //"7" => input_calculate_checksum(),
             _ => eprintln!("\n{}", "ERROR: this option doesn't exist".red()),
        }
    }
}

/// Takes the user input and parses it to a Result Vec f32, &str.
///
/// # Arguments
/// * `message` - the message that will be showed to the user.
///
/// # Returns
/// * `Ok(f32 Vec)` - if there was no error at the parsing.
/// The Ok contains the resulting f32 Vec parsed from the user
/// input.
/// * `Err(&str)` - if some error occurred during the parsing.
/// The Err contains the message of the error.
fn parse_user_input_vec(message: &str) -> Result<Vec<f32>, &str> {
    let mut user_input = String::new();
    print!("\n{}",message.blue());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("ERROR: couldn't read user input.");

    let result: Result<Vec<f32>, _> = user_input
        .split(',')
        .map(|s| s.trim().parse::<f32>().map_err(|_| "Couldn't parse to f32"))
        .collect();

    result
}

/// Takes the user input and parses it to a single f32 value.
/// It returns an error if it couldnt parse the user input.
///
/// # Arguments
/// * `message` - The message to be shown to the user.
///
/// # Returns
/// * Ok(f32) - if successful.
/// * Err(ParseFloatError) - if an error occurred.
fn parse_user_input(message: &str) -> Result<f32, ParseFloatError> {
    let mut user_input = String::new();
    print!("\n{}",message.blue());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("ERROR: couldn't read user input.");

    user_input.trim().parse()
}