use colored::Colorize;

use crate::entropy::calculate_entropy;
use crate::parse_user_input_vec;

/// Calculates the main characterization of a code, including
/// its average length, Kraft's inequality and efficiency.
///
/// # Arguments
/// * `probabilities` - f32 Vec that holds the probabilities of the font.
/// * `lengths` - f32 Vec that holds the lengths of the code words.
///
/// # Returns
/// A f32 array with length 3 that has the average length at index 0, Kraft's
/// inequality at index 1 and efficiency at index 2.
fn characterization(probabilities: Vec<f32>, lengths: Vec<f32>) -> [f32; 3] {
    let entropy: f32 = calculate_entropy(&probabilities);
    let mut characteristics: [f32; 3] = [0.0, 0.0, 0.0];

    // Calculating average length and Kraft's inequality
    for i in 0..lengths.len() {
        characteristics[0] += lengths[i] * probabilities[i];
        characteristics[1] += 1.0/2_i32.pow(lengths[i] as u32) as f32;
    }

    // Calculating efficiency
    characteristics[2] = entropy / characteristics[0];

    characteristics
}

/// Takes the user input and and calls to characterization().
/// It prints the characterization of the code or the error if
/// there was any.
pub fn input_characterization() {
    let probabilities: Result<Vec<f32>, &str> = parse_user_input_vec("Introduce las probabilidades de los símbolos(separados por comas y sin espacios):");
    let lengths: Result<Vec<f32>, &str> = parse_user_input_vec("Introduce las longitudes de las palabras código(separados por comas y sin espacios):");

    if probabilities.is_ok() && lengths.is_ok() {
        let characteristics = characterization(probabilities.unwrap(), lengths.unwrap());
        println!("\n{}{}", "Longitud media de palabra código(L) = ".green(), characteristics[0].to_string().green());
        println!("{}{}", "Desigualdad de Kraft(K) = ".green(), characteristics[1].to_string().green());
        println!("{}{}", "Eficiencia(n) = ".green(), characteristics[2].to_string().green());
    } else {
        eprintln!("\n{}{}", "ERROR: ".red(), "Couldn't parse the user input to a numeric vector".red());
        return;
    } 
}

#[cfg(test)]
mod tests {
    #[test]
    fn characterization_test() {
        let probs: Vec<f32> = vec![
            0.082, 0.015, 0.028, 0.043, 0.124, 0.022, 0.020, 0.061, 0.070, 0.002,
            0.008, 0.040, 0.024, 0.067, 0.075, 0.019, 0.001, 0.060, 0.063, 0.091,
            0.028, 0.010, 0.024, 0.002, 0.0200, 0.001
        ];
        let lengths: Vec<f32> = vec![
            5.0, 9.0, 11.0, 7.0, 1.0, 9.0, 9.0, 7.0, 3.0, 13.0, 9.0, 9.0, 7.0, 5.0,
            11.0, 11.0, 13.0, 7.0, 5.0, 3.0, 7.0, 9.0, 9.0, 11.0, 13.0, 11.0
        ];
        let mut characts: [f32; 3] = super::characterization(probs, lengths);
        characts[0] = (characts[0] * 1000.0).round() / 1000.0;
        characts[1] = (characts[1] * 1000000.0).round() / 1000000.0;
        characts[2] = (characts[2] * 100000.0).round() / 100000.0;
        assert_eq!(characts[0], 6.104);
        assert_eq!(characts[1], 0.899292);
        assert_eq!(characts[2], 0.68603);
    }
}