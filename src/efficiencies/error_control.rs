use std::io::{self, Write};
use colored::Colorize;
use crate::efficiencies::calculate_a;
use crate::parse_user_input;

/// Shows the menu to select which control error control
/// we want to calculate.
pub fn error_correction_menu() {
    const MENU: &str =
    r#"    *********** CORRECCIÓN ERRORES **********
    *   1 - Stop & Wait                     *
    *   2 - Go Back N                       *
    *   3 - Selective Reject                *
    *   v - Volver                          *
    *****************************************"#;

    'ec_loop: loop {
        let mut option: String = String::new();
        println!("\n\n");
        println!("{}", MENU.bright_cyan());
        print!("\n{}", "Escoge que deseas hacer: ".bright_cyan());
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut option)
            .expect("ERROR: no se pudo leer el input del usuario.");

        match option.as_str().trim() {
            "v" => break 'ec_loop,
            "1" => input_ec_stop_and_wait(),
            "2" => input_ec_go_back_n(),
            "3" => input_ec_selective_reject(),
            _ => eprintln!("\n{}", "ERROR: La opción escogida no existe dentro de las posibles".red()),
        }
    }
}

/// Calculates the efficiency of the Stop & Wait ARQ mechanism.
///
/// # Arguments
/// * `tprop` - the value of the relation d/vprop.
/// * `tfrane` - the value of the relation L/R.
/// * `p` - the value for pretr = pframe = 1 - (1 - BER)^L
///
/// # Returns
/// The value for the efficiency of this mechanism.
fn ec_stop_and_wait(tprop: f32, tframe: f32, p: f32) -> f32 {
    (1.0 - p) / (1.0 + 2.0 * calculate_a(tprop, tframe))
}

/// Takes the user input for the required values and calls
/// to ec_stop_and_wait(). It prints the result if the
/// process was successful or an error if there was any.
pub fn input_ec_stop_and_wait() {
    let tprop = match parse_user_input("Introduce el valor para tprop(d/vprop): ") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let tframe = match parse_user_input("Introduce el valor para tframe(L/R): ") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let p = match parse_user_input("Introduce el valor para p: ") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let efficiency = ec_stop_and_wait(tprop, tframe, p);
    println!("\n{}{}", "Eficiencia Stop & Wait ARQ = ".green(), efficiency.to_string().green());
}

/// Calculates the efficiency of the Go-back-N mechanism.
///
/// # Arguments
/// * `tprop` - the value of the relation d/vprop.
/// * `tfrane` - the value of the relation L/R.
/// * `p` - the value for p = pretr = pframe = 1 - (1 - BER)^L
/// * `n` - the size of the window(2^k - 1).
///
/// # Returns
/// The value for the efficiency of this mechanism.
fn ec_go_back_n(tprop: f32, tframe: f32, p: f32, n: u16) -> f32 {
    let a: f32 = calculate_a(tprop, tframe);
    if n as f32 >= 2.0 * a + 1.0 {
        return (1.0 - p) / (1.0 + 2.0 * a * p);
    }

    (n as f32 * (1.0 - p)) / ((2.0 * a + 1.0) * (1.0 - p + n as f32 * p))
}

/// Takes the input of the required values and calls to
/// ec_go_back_n(). It prints the result if the process was
/// successful or an error if there was any.
pub fn input_ec_go_back_n() {
    let tprop = match parse_user_input("Introduce el valor para tprop(d/vprop): ") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let tframe = match parse_user_input("Introduce el valor para tframe(L/R): ") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let p = match parse_user_input("Introduce el valor para p: ") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let n = match parse_user_input("Introduce el valor para n(2^k - 1): ") {
        Ok(value) => value as u16,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let efficiency = ec_go_back_n(tprop, tframe, p, n);
    println!("\n{}{}", "Eficiencia Go-back-N = ".green(), efficiency.to_string().green());
}

/// Calculates the efficiency of the Selective Reject mechanism.
///
/// # Arguments
/// * `tprop` - the value of the relation d/vprop.
/// * `tfrane` - the value of the relation L/R.
/// * `p` - the value for p = pretr = pframe = 1 - (1 - BER)^L
/// * `n` - the size of the window(2^(k-1)).
///
/// # Returns
/// The value for the efficiency of this mechanism.
fn ec_selective_reject(tprop: f32, tframe: f32, p: f32, n: u16) -> f32 {
    let a: f32 = calculate_a(tprop, tframe);

    if n as f32 >= 2.0 * a + 1.0  {
        return 1.0 - p;
    }

    (n as f32 * (1.0 - p)) / ((2.0 * a + 1.0) * (1.0 - p + n as f32 * p))
}

/// Takes the user input for the required values and
/// calls to ec_selective_reject(). It prints the result
/// if the process was successful or an error if there was any.
pub fn input_ec_selective_reject() {
    let tprop = match parse_user_input("Introduce el valor para tprop(d/vprop): ") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let tframe = match parse_user_input("Introduce el valor para tframe(L/R): ") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let p = match parse_user_input("Introduce el valor para p: ") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let n = match parse_user_input("Introduce el valor para n(2^(k - 1)): ") {
        Ok(value) => value as u16,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let efficiency = ec_selective_reject(tprop, tframe, p, n);
    println!("\n{}{}", "Eficiencia Selective Reject = ".green(), efficiency.to_string().green());
}