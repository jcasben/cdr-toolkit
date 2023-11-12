use std::io::{self, Write};
use colored::Colorize;
use crate::efficiencies::calculate_a;
use crate::parse_user_input;

/// Shows the
pub fn flow_control_menu() {
    let tab: String = String::from("   ");
    let mut option: String = String::new();

    'main_loop: loop {
        println!("\n\n");
        println!("************ CONTROL DE FLUJO ***********");
        println!("*{tab}1 - Stop & Wait                     *");
        println!("*{tab}2 - Ventana Deslizante              *");
        println!("*{tab}v - Volver                          *");
        println!("*****************************************");
        print!("\nEscoge que deseas hacer: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut option)
            .expect("ERROR: no se pudo leer el input del usuario.");

        match option.as_str().trim() {
            "v" => break 'main_loop,
            "1" => input_fc_stop_and_wait(),
            "2" => println!(),
            _ => println!(),
        }
    }
}

///
pub fn fc_stop_and_wait(tprop: f32, tframe: f32) -> f32 {
    1.0 / (1.0 + 2.0 * calculate_a(tprop, tframe))
}

pub fn input_fc_stop_and_wait() {
    let mut tprop: f32 = 0.0;
    let mut tframe: f32 = 0.0;
    match parse_user_input("Introduce el valor para tprop(d/vprop): ") {
        Ok(value) => tprop = value,
        Err(err) => eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red())
    }

    //If tprop == 0.0 at this point, it is because an error
    //ocurred.
    if tprop == 0.0 {
        return;
    }

    match parse_user_input("Introduce el valor para tframe(L/R): ") {
        Ok(value) => tframe = value,
        Err(err) => eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red())
    }

    if tframe == 0.0 {
        return;
    }

    let efficiency = fc_stop_and_wait(tprop, tframe);
    println!("{}{}", "Eficiencia s&w = ".blue(), efficiency.to_string().blue());
}

///
pub fn fc_slippery_window(tprop: f32, tframe: f32, n: u16) -> f32  {
    let a: f32 = calculate_a(tprop, tframe);

    if n as f32 >= (2.0 * a + 1.0) {
        return 1.0
    }

    n as f32 / (1.0 + 2.0 * a)
}