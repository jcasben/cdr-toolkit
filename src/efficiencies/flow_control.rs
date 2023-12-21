use std::io::{self, Write};
use std::num::ParseFloatError;
use colored::Colorize;
use crate::efficiencies::calculate_a;
use crate::parse_user_input;

/// Shows the menu to select which control flow efficiency
/// we want to calculate.
pub fn flow_control_menu() {
    const MENU: &str =
r#"************ CONTROL DE FLUJO ***********
*   1 - Stop & Wait                     *
*   2 - Ventana Deslizante              *
*   v - Volver                          *
*****************************************"#;

    'fc_loop: loop {
        let mut option: String = String::new();
        println!("\n\n");
        println!("{}", MENU.bright_cyan());
        print!("\n{}", "Escoge que deseas hacer: ".bright_cyan());
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut option)
            .expect("ERROR: no se pudo leer el input del usuario.");

        match option.as_str().trim() {
            "v" => break 'fc_loop,
            "1" => input_fc_stop_and_wait(),
            "2" => input_fc_slippery_window(),
            _ => eprintln!("\n{}", "ERROR: La opción escogida no existe dentro de las posibles".red()),
        }
    }
}

/// Calculates the efficiency of a Stop & Wait control flow mechanism.
///
/// # Arguments
/// * `tprop` - the value of the relation d/vprop.
/// * `tframe` - the value of the relation L/R.
///
/// # Returns
/// The value of the efficiency of this mechanism.
fn fc_stop_and_wait(tprop: f32, tframe: f32) -> f32 {
    1.0 / (1.0 + 2.0 * calculate_a(tprop, tframe))
}

/// Takes the user input for the required values and calls
/// to fc_stop_and_wait(). It prints the result if the process
/// was successful or an error if there was any.
pub fn input_fc_stop_and_wait() {
    let tprop: Result<f32, ParseFloatError> = parse_user_input("Introduce el valor para tframe(L/R): ");
    let tframe: Result<f32, ParseFloatError> = parse_user_input("Introduce el valor para tframe(L/R): ");

    if tprop.is_ok() && tframe.is_ok() {
        let efficiency = fc_stop_and_wait(tprop.unwrap(), tframe.unwrap());
        println!("{}{}", "Eficiencia Stop & Wait = ".blue(), efficiency.to_string().blue());
    } else {
        eprintln!("\n{}{}", "ERROR: ".red(), "Couldn't parse the user input to a numeric value".red());
        return;
    }
}

/// Calculates the efficiency of a slippery window control flow mechanism.
///
/// # Arguments
/// * `tprop` - the value of the relation d/vprop.
/// * `tframe` - the value of the relation L/R.
/// * `k` - the size of the window.
///
/// # Returns
/// The value for the efficiency of this mechanism.
fn fc_slippery_window(tprop: f32, tframe: f32, k: u32) -> f32 {
    let a: f32 = calculate_a(tprop, tframe);
    let n = 2i32.pow(k) - 1;
        
    if n as f32 >= (2.0 * a + 1.0) {
        return 1.0
    }

    n as f32 / (1.0 + 2.0 * a)
}

/// Takes the user input for the required values and calls
/// to fc_slippery_window(). It prints the result if the process
/// was successful or an error if there was any.
pub fn input_fc_slippery_window() {
    let tprop = parse_user_input("Introduce el valor para tframe(L/R): ");
    let tframe = parse_user_input("Introduce el valor para tprop(d/vprop): ");
    let k = parse_user_input("Introduce el valor para k: ");

    if tprop.is_ok() && tframe.is_ok() && k.is_ok() {
        let efficiency = fc_slippery_window(tprop.unwrap(), tframe.unwrap(), k.unwrap() as u32);
        println!("{}{}", "Eficiencia Ventana Deslizante = ".blue(), efficiency.to_string().blue());
    } else {
        eprintln!("\n{}{}", "ERROR: ".red(), "Couldn't parse the user input to a numeric value".red());
        return;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fc_stop_and_wait_test() {
        let tprop: f32 = 4672.89;
        let tframe: f32 = 78.4;

        assert_eq!(super::fc_stop_and_wait(tprop, tframe), 0.008319027);
    }
}