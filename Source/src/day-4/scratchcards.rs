use std::str::Split;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Scratchcard {
    pub number: u32,
    winning_numbers: Vec<u32>,
    pulled_numbers: Vec<u32>
}

impl Scratchcard {
    pub fn from_str(str: &str) -> Vec<Scratchcard> {
        str
            .lines()
            .filter_map(Scratchcard::from_line)
            .collect::<Vec<Scratchcard>>()
    }

    fn from_line(line: &str) -> Option<Scratchcard> {
        if line.len() < 48 {
            return None;
        }

        let semicolon_index = line.find(':')?;
        let game_id = line[5..semicolon_index]
            .trim()
            .parse::<u32>()
            .ok()?;
        let mut numbers = line[semicolon_index..]
            .trim()
            .split('|');
        let winning_numbers = extract_numbers(&mut numbers)?;
        let pulled_numbers = extract_numbers(&mut numbers)?;

        Some(Scratchcard {
            number: game_id,
            winning_numbers,
            pulled_numbers
        })
    }

    fn match_count(self: &Self) -> u32 {
        let mut match_count = 0;
        for pulled_number in &self.pulled_numbers {
            for winning_number in &self.winning_numbers {
                if pulled_number == winning_number {
                    match_count += 1;
                }
            }
        }

        match_count
    }

    pub fn points(self: &Self) -> u32 {
        let mut match_count = 0;
        for pulled_number in &self.pulled_numbers {
            for winning_number in &self.winning_numbers {
                if pulled_number == winning_number {
                    match_count += 1;
                }
            }
        }

        if match_count <= 2 {
            match_count
        } else {
            1 << (match_count - 1)
        }
    }
}

fn extract_numbers(numbers: &mut Split<char>) -> Option<Vec<u32>> {
    Some(numbers
        .next()?
        .split(' ')
        .filter_map(|number| number.trim().parse::<u32>().ok())
        .collect::<Vec<u32>>())
}

#[derive(Debug, Eq, PartialEq)]
pub struct CopyData {
    card_number: u32,
    copy_count: u32
}

pub fn get_initial_copies(scratchcards: &Vec<Scratchcard>) -> Vec<CopyData> {
    let mut copy_data = Vec::<CopyData>::new();

    for (index, scratchcard) in scratchcards.iter().enumerate() {
        let match_count = scratchcard.match_count() as usize;

        if match_count > 0 {
            for copy_index in 1..=match_count {
                let copy_index = index + copy_index;
                match scratchcards.get(copy_index) {
                    None => eprintln!("Warning: copy index {copy_index} yielded no scratchcard!"),
                    Some(copy) => copy_data.push(CopyData {
                        card_number: copy.number,
                        copy_count: copy.match_count()
                    })
                }
            }
        }
    }

    copy_data
}

pub fn get_copy_data(scratchcards: &Vec<Scratchcard>, copy_data: &Vec<CopyData>) -> Vec<CopyData> {
    let mut new_copy_data = Vec::<CopyData>::new();

    for copy_data in copy_data {
        if copy_data.copy_count > 0 {
            for copy_index in 1..=copy_data.copy_count {
                let copy_index = (copy_data.card_number - 1 + copy_index) as usize;
                match scratchcards.get(copy_index) {
                    None => {}
                    Some(copy) => new_copy_data.push(CopyData {
                        card_number: copy.number,
                        copy_count: copy.match_count()
                    })
                }
            }
        }
    }

    new_copy_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_line() {
        let scratchcard = Scratchcard::from_line("Card 1: 34 67  3 99 23 | 34 85 33 31 22 53 33 24");
        assert_eq!(scratchcard, Some(Scratchcard {
            number: 1,
            winning_numbers: vec![34, 67, 3, 99, 23],
            pulled_numbers: vec![34, 85, 33, 31, 22, 53, 33, 24]
        }))
    }

    #[test]
    fn test_from_str() {
        let scratchcards = Scratchcard::from_str(
            "Card 1: 34 67  3 99 23 | 34 85 33 31 22 53 33 24
Card 2: 45 73 23 14 74 | 33 25 34 31 22 52 11  3
Card 3:  8 41 25 14 99 | 45 23 52 74 35 24 51 63
");
        let scratchcard1 = Scratchcard {
            number: 1,
            winning_numbers: vec![34, 67, 3, 99, 23],
            pulled_numbers: vec![34, 85, 33, 31, 22, 53, 33, 24]
        };
        let scratchcard2 = Scratchcard {
            number: 2,
            winning_numbers: vec![45, 73, 23, 14, 74],
            pulled_numbers: vec![33, 25, 34, 31, 22, 52, 11,  3]
        };
        let scratchcard3 = Scratchcard {
            number: 3,
            winning_numbers: vec![8, 41, 25, 14, 99],
            pulled_numbers: vec![45, 23, 52, 74, 35, 24, 51, 63]
        };
        assert_eq!(scratchcards, vec![scratchcard1, scratchcard2, scratchcard3]);
    }

    #[test]
    fn test_from_line_with_card_spaces() {
        let scratchcards = Scratchcard::from_line(
            "Card   1: 34 67  3 99 23 | 34 85 33 31 22 53 33 24");
        let scratchcard1 = Scratchcard {
            number: 1,
            winning_numbers: vec![34, 67, 3, 99, 23],
            pulled_numbers: vec![34, 85, 33, 31, 22, 53, 33, 24]
        };
        assert_eq!(scratchcards, Some(scratchcard1));
    }

    #[test]
    fn test_points() {
        let points = Scratchcard::from_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
            .unwrap()
            .points();
        assert_eq!(points, 8);

        let points = Scratchcard::from_line("Card 2: 41 13 59 45 22 | 83 86  6 31 17  9 48 53")
            .unwrap()
            .points();
        assert_eq!(points, 0);

        let points = Scratchcard::from_line("Card 3: 41 13 59  1 22 | 83 86  1 31 17  9 48 53")
            .unwrap()
            .points();
        assert_eq!(points, 1);

        let points = Scratchcard::from_line("Card 4: 41 13 59  1 74 | 83 86 22 31 41  9 48 53")
            .unwrap()
            .points();
        assert_eq!(points, 1);
    }

    #[test]
    fn test_example_part1() {
        let points = Scratchcard::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
            .iter()
            .map(|scratchcard| scratchcard.points())
            .sum::<u32>();
        assert_eq!(points, 13);
    }

    #[test]
    fn test_get_copy_data() {
        let scratchcards = Scratchcard::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        let copy_data = get_initial_copies(&scratchcards);
        assert_eq!(copy_data, vec![
            CopyData { card_number: 2, copy_count: 2 },
            CopyData { card_number: 3, copy_count: 2 },
            CopyData { card_number: 4, copy_count: 1 },
            CopyData { card_number: 5, copy_count: 0 },
            CopyData { card_number: 3, copy_count: 2 },
            CopyData { card_number: 4, copy_count: 1 },
            CopyData { card_number: 4, copy_count: 1 },
            CopyData { card_number: 5, copy_count: 0 },
            CopyData { card_number: 5, copy_count: 0 }
        ]);
        let copy_data = get_copy_data(&scratchcards, &copy_data);
        assert_eq!(copy_data, vec![
            CopyData { card_number: 3, copy_count: 2 },
            CopyData { card_number: 4, copy_count: 1 },
            CopyData { card_number: 4, copy_count: 1 },
            CopyData { card_number: 5, copy_count: 0 },
            CopyData { card_number: 5, copy_count: 0 },
            CopyData { card_number: 4, copy_count: 1 },
            CopyData { card_number: 5, copy_count: 0 },
            CopyData { card_number: 5, copy_count: 0 },
            CopyData { card_number: 5, copy_count: 0 }
        ]);
    }
}