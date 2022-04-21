fn plugboard(input_letters: &String, output_letters: &String, input_char: char) -> char {
    if let Some(_) = input_letters.find(input_char) {
        output_letters
            .chars()
            .nth(input_letters.find(input_char).unwrap())
            .unwrap()
    } else {
        input_char
    }
}

fn reflector(reflector_letters: &String, input_char: char) -> char {
    const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWZYZ";
    let index_in_reflector: usize = ALPHABET.find(input_char).unwrap();
    reflector_letters.chars().nth(index_in_reflector).unwrap()
}

fn main() {
    //           (letters, turnover notch)
    let rotor1: (&str, char) = ("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
    let rotor2: (&str, char) = ("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
    let rotor3: (&str, char) = ("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
    let rotor4: (&str, char) = ("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J');
    let rotor5: (&str, char) = ("VZBRGITYUPSDNHLXAWMJQOFECK", 'Z');

    let reflector_b: String = "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_string();
    let reflector_c: String = "FVPJIAOYEDRZXWGCTKUQSBNMHL".to_string();

    let plugboard_input: String = "ABCDEGHIJLMNOQSTVWXZ".to_string();
    let plugboard_output: String = "HRSLPFOCZYAUDETWJIQM".to_string();

    println!("{}", plugboard(&plugboard_input, &plugboard_output, 'B'));
    println!("{}", reflector(&reflector_b, 'C'));
}
