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

#[cfg(test)]
mod tests {
    #[test]
    fn calculate_entropy_test() {
        let probs: Vec<f32> = vec![
            0.082, 0.015, 0.028, 0.043, 0.124, 0.022, 0.020, 0.061, 0.070, 0.002,
            0.008, 0.040, 0.024, 0.067, 0.075, 0.019, 0.001, 0.060, 0.063, 0.091,
            0.028, 0.010, 0.024, 0.002, 0.0200, 0.001
        ];
        let entropy = (super::calculate_entropy(probs) * 100000.0).round() / 100000.0;
        assert_eq!(entropy, 4.18752);
    }
}