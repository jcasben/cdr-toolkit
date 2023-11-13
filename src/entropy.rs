use colored::Colorize;

use crate::parse_user_input_vec;

/// Calculates the entropy of a font given the probabilities of its symbols.
///
/// # Arguments
/// * `vector` - a f32 Vec that holds the probabilities of the font.
///
/// # Returns
/// A f32 number which represents the entropy of that font.
pub fn calculate_entropy(vector: Vec<f32>) -> f32 {
    let mut entropy: f32 = 0.0;

    //Calculate entropy
    for i in vector.iter() {
        entropy += i * f32::log2(1.0/i);
    }

    entropy
}

/// Takes the user input and calls to calculate_entropy().
/// It prints the entropy if the process was successful or
/// the error if there was any.
pub fn input_calculate_entropy() {
    match parse_user_input_vec("Introduce las probabilidades de los símbolos(separados por comas y sin espacios):") {
        Ok(vector) => {
            println!("\n{}{}", "H(x) = ".green(), calculate_entropy(vector).to_string().green());
        },
        Err(e) => eprintln!("\n{}{}", "ERROR: ".red(), e.red())
    }
}