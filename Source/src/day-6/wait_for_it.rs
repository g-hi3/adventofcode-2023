#[derive(Debug, PartialEq)]
pub struct RaceRecord {
    time: u64,
    distance: u64
}

impl RaceRecord {
    pub fn extract(str: &str) -> Vec<Self> {
        let mut lines = str.lines();
        let times = match lines.next() {
            Some(line) => get_values(line),
            None => return vec![]
        };
        let distances = match lines.next() {
            Some(line) => get_values(line),
            None => return vec![]
        };
        let mut races = Vec::<Self>::new();

        for (&time, distance) in times.iter().zip(distances) {
            races.push(RaceRecord { time, distance })
        }

        races
    }

    pub fn transform(sub_values: Vec<Self>) -> Self {
        let mut total_time = 0;
        let mut total_distance = 0;

        for sub_value in sub_values {
            total_time = add_sub_value(total_time, sub_value.time);
            total_distance = add_sub_value(total_distance, sub_value.distance);
        }

        Self {
            time: total_time,
            distance: total_distance
        }
    }
}

fn get_values(line: &str) -> Vec<u64> {
    let colon_index = line.find(':');
    match colon_index {
        None => vec![],
        Some(colon_index) => line[colon_index+1..]
            .trim()
            .split(' ')
            .filter_map(|part| part.parse::<u64>().ok())
            .collect::<Vec<u64>>()
    }
}

fn add_sub_value(v: u64, w: u64) -> u64 {
    let mut v = v;
    let mut remainder = w;

    while remainder >= 10 {
        remainder /= 10;
        v *= 10;
    }

    v * 10 + w
}

#[derive(Debug, PartialEq)]
pub struct ToyBoat {
    starting_speed: u64,
    speed_rate: u64
}

impl ToyBoat {
    pub fn get_my() -> ToyBoat {
        ToyBoat {
            starting_speed: 0,
            speed_rate: 1
        }
    }

    fn distance(self: &Self, hold_time: u64, total_time: u64) -> u64 {
        self.speed_rate * hold_time * (total_time - hold_time)
    }

    pub fn count_winning_hold_times(self: &Self, race_record: &RaceRecord) -> u32 {
        (1..race_record.time)
            .filter(|&hold_time| self.distance(hold_time, race_record.time) > race_record.distance)
            .count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract() {
        let race_records = RaceRecord::extract("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(race_records, vec![
            RaceRecord { time: 7, distance: 9 },
            RaceRecord { time: 15, distance: 40 },
            RaceRecord { time: 30, distance: 200 }
        ])
    }

    #[test]
    fn test_distance() {
        let my_toy_boat = ToyBoat::get_my();
        assert_eq!(my_toy_boat.distance(0, 7), 0);
        assert_eq!(my_toy_boat.distance(1, 7), 6);
        assert_eq!(my_toy_boat.distance(2, 7), 10);
        assert_eq!(my_toy_boat.distance(3, 7), 12);
        assert_eq!(my_toy_boat.distance(4, 7), 12);
        assert_eq!(my_toy_boat.distance(5, 7), 10);
        assert_eq!(my_toy_boat.distance(6, 7), 6);
        assert_eq!(my_toy_boat.distance(7, 7), 0);
    }

    #[test]
    fn test_count_winning_hold_times() {
        let my_toy_boat = ToyBoat::get_my();
        let race_records = RaceRecord::extract("Time:      7  15   30
Distance:  9  40  200");
        let winning_hold_times_count = my_toy_boat.count_winning_hold_times(race_records.get(0).unwrap());
        assert_eq!(winning_hold_times_count, 4);
        let winning_hold_times_count = my_toy_boat.count_winning_hold_times(race_records.get(1).unwrap());
        assert_eq!(winning_hold_times_count, 8);
        let winning_hold_times_count = my_toy_boat.count_winning_hold_times(race_records.get(2).unwrap());
        assert_eq!(winning_hold_times_count, 9);
    }

    #[test]
    fn test_add_sub_value() {
        let v = add_sub_value(7, 15);
        assert_eq!(v, 715);
        let v = add_sub_value(940, 200);
        assert_eq!(v, 940200);
    }

    #[test]
    fn test_transform() {
        let race_records = RaceRecord::extract("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(RaceRecord::transform(race_records), RaceRecord {
            time: 71530,
            distance: 940200
        });
    }
}