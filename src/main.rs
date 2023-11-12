use std::io::{self, Write};
use std::num::ParseFloatError;
use std::process::exit;
use colored::Colorize;

use crate::code_characterization::input_characterization;
use crate::efficiencies::flow_control::flow_control_menu;
use crate::entropy::input_calculate_entropy;

mod entropy;
mod code_characterization;
mod efficiencies;

/// Entry point of the program. Performs an infinite loop which has a menu to
/// select the option that we want to do in each iteration.
fn main() {
    let tab: String = String::from("   ");
    loop {
        let mut option = String::new();
        //Print 
        println!("\n\n");
        println!("************** CDR TOOLKIT **************");
        println!("*{tab}1 - Entropía                        *");
        println!("*{tab}2 - Caracterización de un código    *");
        println!("*{tab}3 - Eficiencia controles de flujo   *");
        println!("*{tab}s - Salir                           *");
        println!("*****************************************");
        print!("\nEscoge que deseas hacer: ");
        io::stdout().flush().unwrap();
        
        io::stdin()
            .read_line(&mut option)
            .expect("ERROR: no se pudo leer el input del usuario.");

        match option.as_str().trim() {
            "s" => {
                println!("\n{}", "Gracias por usar CDR TOOLKIT :)".green().bold());
                exit(0);
            },
            "1" => input_calculate_entropy(),
            "2" => input_characterization(),
            "3" => flow_control_menu(),
            _ => eprintln!("\n{}", "ERROR: La opción escogida no existe dentro de las posibles".red()),
        }
    }
}

/// Takes the user input and parses it to a Result<Vec<f32>, &str>.
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
pub fn parse_user_input_vec(message: &str) -> Result<Vec<f32>, &str> {
    let mut user_input = String::new();
    print!("\n{}",message.blue());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("ERROR: no se pudo leer el input del usuario.");

    let result: Result<Vec<f32>, &'static str> = user_input
        .split(',')
        .map(|s| s.trim().parse::<f32>().map_err(|_| "No se puedo parsear a f32"))
        .collect();

    result.and_then(|v| {
        if v.iter().any(|&x| x.is_nan()) {
            Err("Se encontró un elemento que no se pudo parsear.")
        } else if v.is_empty() {
            Err("No se encontraron elementos válidos.")
        } else {
            Ok(v)
        }
    })
}

pub fn parse_user_input(message: &str) -> Result<f32, ParseFloatError> {
    let mut user_input = String::new();
    print!("\n{}",message.blue());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("ERROR: no se pudo leer el input del usuario.");

    user_input.trim().parse()
}