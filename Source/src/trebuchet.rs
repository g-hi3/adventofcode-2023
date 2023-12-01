pub fn extract_calibration_value(line: &str) -> u32 {
    let first_digit = get_first_digit(line);
    let last_digit = get_last_digit(line);

    last_digit + first_digit * 10
}

fn get_first_digit(line: &str) -> u32 {
    DIGITS
        .iter()
        .map(|digit| DigitPos::in_str(line, digit))
        .filter_map(|digit_pos| digit_pos)
        .min_by(DigitPos::compare_by_position)
        .map(|digit_pos| digit_pos.digit.value)
        .unwrap_or(0)
}

fn get_last_digit(line: &str) -> u32 {
    DIGITS
        .iter()
        .map(|digit| DigitPos::r_in_str(line, digit))
        .filter_map(|digit_pos| digit_pos)
        .max_by(DigitPos::compare_by_position)
        .map(|digit_pos| digit_pos.digit.value)
        .unwrap_or(0)
}

const DIGITS: [Digit; 18] = [
    Digit { name: "1", value: 1 },
    Digit { name: "one", value: 1 },
    Digit { name: "2", value: 2 },
    Digit { name: "two", value: 2 },
    Digit { name: "3", value: 3 },
    Digit { name: "three", value: 3 },
    Digit { name: "4", value: 4 },
    Digit { name: "four", value: 4 },
    Digit { name: "5", value: 5 },
    Digit { name: "five", value: 5 },
    Digit { name: "6", value: 6 },
    Digit { name: "six", value: 6 },
    Digit { name: "7", value: 7 },
    Digit { name: "seven", value: 7 },
    Digit { name: "8", value: 8 },
    Digit { name: "eight", value: 8 },
    Digit { name: "9", value: 9 },
    Digit { name: "nine", value: 9 }
];

struct DigitPos<'a> {
    digit: &'a Digit,
    position: usize
}

impl<'a, 'b> DigitPos<'a> {
    fn in_str(str: &str, digit: &'a Digit) -> Option<DigitPos<'a>> {
        str
            .find(digit.name)
            .map(|position| DigitPos { digit, position })
    }

    fn r_in_str(str: &str, digit: &'a Digit) -> Option<DigitPos<'a>> {
        str
            .rfind(digit.name)
            .map(|position| DigitPos { digit, position })
    }

    fn compare_by_position(left: &DigitPos<'a>, right: &DigitPos<'b>) -> std::cmp::Ordering {
        left.position.cmp(&right.position)
    }
}

struct Digit {
    name: &'static str,
    value: u32
}
