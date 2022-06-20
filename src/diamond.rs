fn alphabet() -> Vec<char> {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>()
}

fn position(alphabet: &[char], input: char) -> usize {
    alphabet.iter().position(|&x| x == input).unwrap() + 1 
}

fn square_length(alphabet: &[char], input: char) -> usize {
    position(alphabet, input) * 2 - 1
}

fn letter_at_pos(alphabet: &[char], index: usize, max_letter: char) -> char {
    let position = position(alphabet, max_letter) - 1;
    let distance = (index as i32 - position as i32).abs() as usize;
    alphabet[position - distance]
}

fn distance(alphabet: &[char], a: char, b: char) -> usize {
    position(alphabet, b) - position(alphabet, a)
}

pub fn build_diamond(input: char) -> String {
    let alphabet = alphabet();
    let length = square_length(&alphabet, input);
    let mut lines = String::from("");

    for i in 0..length {
        let mut line: Vec<char> = (" ".repeat(length) + "\n").chars().collect();
        let letter = letter_at_pos(&alphabet, i, input);
        let distance = distance(&alphabet, letter, input);

        line[distance] = letter;
        line[length - distance - 1] = letter;
        lines.push_str(line.iter().collect::<String>().as_str());
    }

    lines
}

#[cfg(test)]
pub mod tests {
    use super::{letter_at_pos, build_diamond, distance, alphabet};

    fn number_of_lines(input: String) -> usize {
       input.matches('\n').count() 
    }

    fn number_of_columns(input: String) -> usize {
       input.chars().position(|x| x == '\n').unwrap()
    }

    #[test]
    pub fn should_number_of_lines_be_correct() {
        assert_eq!(number_of_lines(build_diamond('A')), 1);
        assert_eq!(number_of_lines(build_diamond('B')), 3);
        assert_eq!(number_of_lines(build_diamond('C')), 5);
    }

    #[test]
    pub fn should_number_of_columns_be_correct() {
        assert_eq!(number_of_columns(build_diamond('A')), 1);
        assert_eq!(number_of_columns(build_diamond('B')), 3);
        assert_eq!(number_of_columns(build_diamond('C')), 5);
    }

    #[test]
    pub fn should_compute_distance() {
        let alphabet = alphabet();
        assert_eq!(distance(&alphabet, 'A', 'C'), 2);
        assert_eq!(distance(&alphabet, 'B', 'C'), 1);
        assert_eq!(distance(&alphabet, 'C', 'C'), 0);
    }

    #[test]
    pub fn should_place_letter_at_position() {
        let alphabet = alphabet();
        assert_eq!(letter_at_pos(&alphabet, 0, 'C'), 'A');
        assert_eq!(letter_at_pos(&alphabet, 1, 'C'), 'B');
        assert_eq!(letter_at_pos(&alphabet, 2, 'C'), 'C');
        assert_eq!(letter_at_pos(&alphabet, 3, 'C'), 'B');
        assert_eq!(letter_at_pos(&alphabet, 4, 'C'), 'A');
    }
}
