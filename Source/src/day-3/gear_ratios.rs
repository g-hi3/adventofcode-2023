#[derive(Debug, Eq, PartialEq)]
pub struct Coords (u32, u32);

impl Coords {
    fn left(self: &Self) -> u32 {
        if self.0 > 0 {
            self.0 - 1
        } else {
            0
        }
    }

    fn right(self: &Self, order_of_magnitude: u32) -> u32 {
        self.0 + 1 + order_of_magnitude
    }

    fn up(self: &Self) -> u32 {
        if self.1 > 0 {
            self.1 - 1
        } else {
            0
        }
    }

    fn down(self: &Self) -> u32 {
        self.1 + 1
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SchematicPart {
    Number {
        value: u32,
        position: Coords
    },
    Symbol {
        kind: char,
        position: Coords
    }
}

impl SchematicPart {
    pub fn extract(schematic: &str) -> Vec<SchematicPart> {
        let mut schematic_parts = Vec::<SchematicPart>::new();
        for (y, line) in schematic.lines().enumerate() {
            let mut number_value = 0;
            let mut number_x = 0;

            for (x, char) in line.chars().enumerate() {
                if let Some(digit) = char.to_digit(10) {
                    if number_value == 0 {
                        number_x = x;
                    }

                    number_value = number_value * 10 + digit;
                    continue;
                }

                if number_value > 0 {
                    schematic_parts.push(SchematicPart::Number {
                        value: number_value,
                        position: Coords(number_x as u32, y as u32)
                    });
                    number_value = 0;
                    number_x = 0;
                }

                if char != '.' {
                    schematic_parts.push(SchematicPart::Symbol {
                        kind: char,
                        position: Coords (x as u32, y as u32)
                    });
                }
            }

            if number_value > 0 {
                schematic_parts.push(SchematicPart::Number {
                    value: number_value,
                    position: Coords(number_x as u32, y as u32)
                });
            }
        }

        schematic_parts
    }
}

pub trait GetPartNumbers {
    fn get_part_numbers(self: &Self) -> Vec<u32>;
    fn get_gear_values(self: &Self) -> Vec<u32>;
}

impl GetPartNumbers for Vec<SchematicPart> {
    fn get_part_numbers(self: &Self) -> Vec<u32> {
        let mut part_numbers = Vec::<u32>::new();

        'outer: for part in self {
            match part {
                SchematicPart::Number { value: number_value, position: number_position } => {
                    for other_part in self {
                        match other_part {
                            SchematicPart::Number { .. } => continue,
                            SchematicPart::Symbol { position: symbol_position, .. } => {
                                if is_adjacent(number_position, symbol_position, get_order_of_magnitude(*number_value)) {
                                    part_numbers.push(*number_value);
                                    continue 'outer;
                                }
                            }
                        }
                    }
                }
                SchematicPart::Symbol { .. } => continue
            }
        }

        part_numbers
    }

    fn get_gear_values(self: &Self) -> Vec<u32> {
        let mut gear_values = Vec::<u32>::new();

        'outer: for part in self {
            match part {
                SchematicPart::Symbol { kind, position: symbol_position } => {
                    if *kind != '*' {
                        continue;
                    }

                    let mut adjacent_numbers = Vec::<u32>::new();
                    for other_part in self {
                        match other_part {
                            SchematicPart::Symbol { .. } => continue,
                            SchematicPart::Number { value: number_value, position: number_position} => {
                                if is_adjacent(number_position, symbol_position, get_order_of_magnitude(*number_value)) {
                                    adjacent_numbers.push(*number_value);
                                }

                                if adjacent_numbers.len() > 2 {
                                    continue 'outer;
                                }
                            }
                        }
                    }

                    if adjacent_numbers.len() == 2 {
                        let number_one = adjacent_numbers.get(0).unwrap_or(&0);
                        let number_two = adjacent_numbers.get(1).unwrap_or(&0);
                        gear_values.push(number_one * number_two);
                    }
                }
                SchematicPart::Number { .. } => continue
            }
        }

        gear_values
    }
}

fn is_adjacent(number_position: &Coords, symbol_position: &Coords, order_of_magnitude: u32) -> bool {
    symbol_position.0 >= number_position.left()
        && symbol_position.0 <= number_position.right(order_of_magnitude)
        && symbol_position.1 >= number_position.up()
        && symbol_position.1 <= number_position.down()
}

fn get_order_of_magnitude(number: u32) -> u32 {
    let mut remainder = number;
    let mut order_of_magnitude = 0;

    while remainder >= 10 {
        remainder /= 10;
        order_of_magnitude += 1;
    }

    order_of_magnitude
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_schematic_parts() {
        let schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let parts = SchematicPart::extract(schematic);
        assert_eq!(parts, vec![
            SchematicPart::Number { value: 467, position: Coords(0, 0) },
            SchematicPart::Number { value: 114, position: Coords(5, 0) },
            SchematicPart::Symbol { kind: '*', position: Coords(3, 1) },
            SchematicPart::Number { value: 35, position: Coords(2, 2) },
            SchematicPart::Number { value: 633, position: Coords(6, 2) },
            SchematicPart::Symbol { kind: '#', position: Coords(6, 3) },
            SchematicPart::Number { value: 617, position: Coords(0, 4) },
            SchematicPart::Symbol { kind: '*', position: Coords(3, 4) },
            SchematicPart::Symbol { kind: '+', position: Coords(5, 5) },
            SchematicPart::Number { value: 58, position: Coords(7, 5) },
            SchematicPart::Number { value: 592, position: Coords(2, 6) },
            SchematicPart::Number { value: 755, position: Coords(6, 7) },
            SchematicPart::Symbol { kind: '$', position: Coords(3, 8) },
            SchematicPart::Symbol { kind: '*', position: Coords(5, 8) },
            SchematicPart::Number { value: 664, position: Coords(1, 9) },
            SchematicPart::Number { value: 598, position: Coords(5, 9) }]);
    }

    #[test]
    fn test_filter_invalid() {
        let parts = vec![
            SchematicPart::Number { value: 45, position: Coords(4, 5) },
            SchematicPart::Symbol { kind: '$', position: Coords(6, 5) },
            SchematicPart::Number { value: 711, position: Coords(5, 6) },
            SchematicPart::Number { value: 6, position: Coords(8, 5) }
        ];
        let part_numbers = parts.get_part_numbers();
        assert_eq!(part_numbers, vec![45, 711]);
    }

    #[test]
    fn test_sum_of_part_numbers() {
        let schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 4361);

        let schematic = "467..114..
+..*......";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 467);

        let schematic = "467..114..
....*.....";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 114);

        let schematic = "*....
.475.
.....";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = ".*...
.475.
.....";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = "..*..
.475.
.....";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = "...*.
.475.
.....";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = "....*
.475.
.....";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = ".....
*475.
.....";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = ".....
.475*
.....";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = ".....
.475.
*....";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = ".....
.475.
.*...";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = ".....
.475.
..*..";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = ".....
.475.
...*.";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = ".....
.475.
....*";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 475);

        let schematic = ".....
...1.
..../";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 1);

        let schematic = ".....
...42
..../";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 42);

        let schematic = ".....
42...
/....";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 42);

        let schematic = ".....
/....
42...";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 42);

        let schematic = ".....
../..
10...";
        let sum_of_part_numbers = SchematicPart::extract(schematic)
            .get_part_numbers()
            .iter()
            .sum::<u32>();
        assert_eq!(sum_of_part_numbers, 10);
    }

    #[test]
    fn test_part_two() {
        let schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let gear_values = SchematicPart::extract(schematic).get_gear_values().iter().sum::<u32>();
        assert_eq!(gear_values, 467_835);
    }
}