use std::io::{self, Write};
use colored::Colorize;

/// Calculates the value of the checksum.
///
/// # Arguments
/// * `values` - a vector that holds the values for the
/// checksum in hexadecimal.
/// * `num_of_bits` - num of bits that will be represented.
///
/// # Returns
/// * The result of the checksum.
fn calculate_checksum(values: Vec<String>, num_of_bits: usize) -> String {
    let mut result: i64 = 0;

    for i in values.iter() {
        let x: i64 = match i64::from_str_radix(i.trim(), 16) {
            Ok(num) => num,
            Err(err) => {
                eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
                0
            }
        };
        result += x;
    }

    result = !result;
    let hex_string = format!("{:b}", result);
    let mut truncated_hex_string = hex_string.chars().rev().take(num_of_bits).collect::<String>();
    truncated_hex_string = truncated_hex_string.chars().rev().collect::<String>();

    truncated_hex_string
}

/// Takes the binary user input for all the values of the checksum
/// and prints the result.
pub fn input_calculate_checksum() {
    let mut user_input = String::new();
    print!("{}", "Introduce la cantidad de valores que se van a introducir: ".blue());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("ERROR: no se pudo leer el input del usuario.");

    let num_of_elements: i64 = match user_input.trim().parse::<i64>() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let mut user_input = String::new();
    print!("{}", "Introduce la cantidad de bits de las palabras: ".blue());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("ERROR: no se pudo leer el input del usuario.");

    let num_of_bits: usize = match user_input.trim().parse::<usize>() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("\n{}{}", "ERROR: ".red(), err.to_string().red());
            return;
        }
    };

    let mut values: Vec<String> = Vec::with_capacity(num_of_elements as usize);
    let mut i = 0;
    while i < num_of_elements {
        let mut user_input = String::new();
        print!("\n{}{}: ", "Introduce el valor hexadecimal ".blue(), (i).to_string().blue());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).expect("ERROR: no se pudo leer el input del usuario.");

        values.push(user_input);
        i += 1;
    }

    println!("\n{}{}", "Valor del checksum(binario) = ".green(), calculate_checksum(values, num_of_bits));
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculate_checksum_test() {
        let values = vec!["0001".to_owned(), "F203".to_owned(), "F4F5".to_owned(), "F6F7".to_owned()];
        assert_eq!("220F".to_owned(), super::calculate_checksum(values, 16));
    }
}