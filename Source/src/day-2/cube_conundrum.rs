use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Cubes<'a> {
    kind: &'a str,
    count: u32
}

impl Cubes<'_> {
    fn from_str(str: &str) -> Result<Cubes, String> {
        let str = str.trim();
        let space_index = str.find(' ');
        match space_index {
            None => Err(String::from("Invalid string format! Cubes string must be '<count> <kind>'.")),
            Some(space_index) => {
                let count_str = &str[..space_index];
                let kind = &str[space_index+1..];
                match count_str.parse::<u32>() {
                    Ok(count) => Ok(Cubes { kind, count }),
                    Err(parse_error) => Err(String::from(format!("Unable to parse cubes count {count_str}!\n{parse_error}")))
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CubesSet<'a> {
    cubes: Vec<Cubes<'a>>
}

impl CubesSet<'_> {
    fn from_str(str: &str) -> CubesSet {
        let cubes = str
            .trim()
            .split(',')
            .map(Cubes::from_str)
            .filter_map(|result| match result {
                Ok(cubes) => Some(cubes),
                Err(_) => None
            })
            .collect::<Vec<Cubes>>();
        CubesSet { cubes }
    }

    pub fn power(self: &Self) -> u32 {
        self.cubes.iter().fold(1_u32, |prod, cube| prod * cube.count)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Game<'a> {
    pub id: u32,
    sets: Vec<CubesSet<'a>>
}

impl<'a, 'b> Game<'a> where 'a: 'b {
    pub fn from_line(line: &str) -> Result<Game, String> {
        if !line.starts_with("Game ") {
            return Err(String::from("Game line must start with 'Game <id>:'!"));
        }

        let colon_index = line.chars().position(|char| char == ':');
        match colon_index {
            None => Err(String::from("Game line must start with 'Game <id>:'!")),
            Some(colon_index) => {
                let id_str = &line[5..colon_index];
                let id = id_str.parse::<u32>();
                match id {
                    Ok(id) => {
                        let sets = line[colon_index + 1..]
                            .split(';')
                            .map(CubesSet::from_str)
                            .collect::<Vec<CubesSet>>();
                        Ok(Game { id, sets })
                    },
                    Err(error) => Err(format!("Unable to parse game id {id_str}!\n{error}"))
                }
            }
        }
    }

    pub fn is_allowed_by_constraint(self: &Self, constraints: &Vec<MaxBallsConstraint>) -> bool {
        !constraints.iter().any(|constraint| self.sets
            .iter()
            .flat_map(|set| &set.cubes)
            .any(|cubes| cubes.kind == constraint.kind && cubes.count > constraint.max_count))
    }

    pub fn get_minimum_required_set(self: &Self) -> CubesSet<'b> {
        let mut min_required = HashMap::<&str, u32>::new();
        for cubes in self.sets.iter().flat_map(|set| &set.cubes) {
            let entry = min_required.entry(cubes.kind);
            match entry {
                Entry::Occupied(mut occupied) => {
                    if *occupied.get() < cubes.count {
                        occupied.insert(cubes.count);
                    }
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(cubes.count);
                }
            }
        }

        let cubes = min_required
            .iter()
            .map(|(&kind, &count)| {
                let kind = kind.clone();
                let count = count.clone();
                Cubes { kind, count }
            })
            .collect::<Vec<Cubes>>();
        CubesSet { cubes }
    }
}

pub struct MaxBallsConstraint<'a> {
    kind: &'a str,
    max_count: u32
}

impl<'a> MaxBallsConstraint<'a> {
    pub fn new(kind: &'a str, max_count: u32) -> MaxBallsConstraint {
        MaxBallsConstraint {
            kind,
            max_count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubes_from_substr() {
        let cubes = Cubes::from_str("3 blue");
        assert_eq!(cubes, Ok(Cubes {
            kind: "blue",
            count: 3
        }));

        let cubes = Cubes::from_str("2 green");
        assert_eq!(cubes, Ok(Cubes {
            kind: "green",
            count: 2
        }));

        let cubes = Cubes::from_str("\t5109 yellow   ");
        assert_eq!(cubes, Ok(Cubes {
            kind: "yellow",
            count: 5109
        }));

        let cubes = Cubes::from_str("3blue");
        assert_eq!(cubes, Err(String::from("Invalid string format! Cubes string must be '<count> <kind>'.")));

        let cubes = Cubes::from_str("-1 blue");
        assert_eq!(cubes, Err(String::from("Unable to parse cubes count -1!\ninvalid digit found in string")));
    }

    #[test]
    fn test_cubes_set_from_substr() {
        let cubes_set = CubesSet::from_str("8 green, 6 blue, 20 red");
        assert_eq!(cubes_set, CubesSet {
            cubes: vec![
                Cubes { kind: "green", count: 8 },
                Cubes { kind: "blue", count: 6 },
                Cubes { kind: "red", count: 20 }
            ]
        });
    }

    #[test]
    fn test_game_from_line() {
        let game = Game::from_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game, Ok(Game {
            id: 1,
            sets: vec![
                CubesSet {
                    cubes: vec![
                        Cubes {
                            kind: "blue",
                            count: 3
                        },
                        Cubes {
                            kind: "red",
                            count: 4
                        }
                    ]
                },
                CubesSet {
                    cubes: vec![
                        Cubes {
                            kind: "red",
                            count: 1
                        },
                        Cubes {
                            kind: "green",
                            count: 2
                        },
                        Cubes {
                            kind: "blue",
                            count: 6
                        }
                    ]
                },
                CubesSet {
                    cubes: vec![
                        Cubes {
                            kind: "green",
                            count: 2
                        }
                    ]
                }
            ]
        }));
    }

    #[test]
    fn test_is_allowed_by_constraint() {
        let game = Game::from_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
            .expect("Game::from_line() should've created a valid Game!");
        assert!(game.is_allowed_by_constraint(&vec![
            MaxBallsConstraint::new("red", 12),
            MaxBallsConstraint::new("green", 13),
            MaxBallsConstraint::new("blue", 14)]));
        assert!(!game.is_allowed_by_constraint(&vec![
            MaxBallsConstraint::new("red", 2),
            MaxBallsConstraint::new("green", 13),
            MaxBallsConstraint::new("blue", 14)]));
    }

    #[test]
    fn test_get_minimum_required_set() {
        let game = Game::from_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
            .expect("Game::from_line() should've created a valid Game!");
        assert!(game.get_minimum_required_set().cubes.iter().any(|cube| cube.kind == "blue" && cube.count == 6));
        assert!(game.get_minimum_required_set().cubes.iter().any(|cube| cube.kind == "green" && cube.count == 2));
        assert!(game.get_minimum_required_set().cubes.iter().any(|cube| cube.kind == "red" && cube.count == 4));
    }

    #[test]
    fn test_cubes_set_power() {
        let cubes_set = CubesSet::from_str("8 green, 6 blue, 20 red");
        assert_eq!(cubes_set.power(), 8 * 6 * 20);
    }
}