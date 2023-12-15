#[derive(Debug, PartialEq)]
pub struct History {
    values: Vec<i64>
}

impl History {
    fn new(s: &str) -> Option<Self> {
        let values = s
            .split(" ")
            .filter_map(|value| value.parse::<i64>().ok())
            .collect::<Vec<i64>>();

        Some(History { values })
    }

    pub fn extract(s: &str) -> Vec<Self> {
        s
            .lines()
            .filter_map(Self::new)
            .collect::<Vec<Self>>()
    }

    fn subsequence(&self) -> Self {
        if self.values.len() < 2 {
            return Self { values: vec![] };
        }

        let mut values = Vec::<i64>::new();
        let mut prev_value = self.values.get(0).unwrap();

        for i in 1..self.values.len() {
            let value = self.values.get(i).unwrap();
            values.push(value - prev_value);
            prev_value = value;
        }

        Self { values }
    }

    fn is_final_sequence(&self) -> bool {
        self.values
            .iter()
            .all(|value| *value == 0)
    }

    pub fn predict(&self) -> i64 {
        if self.is_final_sequence() {
            0
        } else {
            let subsequence = self.subsequence();
            let predicted_value = subsequence.predict();
            let last_value = self.values.last().unwrap();
            last_value + predicted_value
        }
    }

    fn extrapolate(&self) -> Self {
        let predicted_value = self.predict();
        let mut values = self.values.clone();
        values.push(predicted_value);

        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_new() {
        let history = History::new("0 3 6 9 12 15");
        assert_eq!(history, Some(History {
            values: vec![0, 3, 6, 9, 12, 15]
        }));

        let history = History::new("1 3 6 10 15 21");
        assert_eq!(history, Some(History {
            values: vec![1, 3, 6, 10, 15, 21]
        }));

        let history = History::new("10 13 16 21 30 45");
        assert_eq!(history, Some(History {
            values: vec![10, 13, 16, 21, 30, 45]
        }));
    }

    #[test]
    fn test_history_extract() {
        let historical_data = History::extract("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");
        assert_eq!(historical_data, vec![
            History::new("0 3 6 9 12 15").unwrap(),
            History::new("1 3 6 10 15 21").unwrap(),
            History::new("10 13 16 21 30 45").unwrap()
        ]);
    }

    #[test]
    fn test_history_subsequence() {
        let history = History::new("0 3 6 9 12 15").unwrap();
        let subsequence = history.subsequence();
        assert_eq!(subsequence, History { values: vec![3, 3, 3, 3, 3] });

        let history = History::new("0 0 0 0 0").unwrap();
        let subsequence = history.subsequence();
        assert_eq!(subsequence, History { values: vec![0, 0, 0, 0] });

        let history = History::new("5").unwrap();
        let subsequence = history.subsequence();
        assert_eq!(subsequence, History { values: vec![] });

        let history = History::new("").unwrap();
        let subsequence = history.subsequence();
        assert_eq!(subsequence, History { values: vec![] });
    }

    #[test]
    fn test_history_is_final_sequence() {
        let history = History::new("0 3 6 9 12 15").unwrap();
        assert!(!history.is_final_sequence());

        let history = History::new("0 0 0 0").unwrap();
        assert!(history.is_final_sequence());
    }

    #[test]
    fn test_history_predict() {
        let history = History::new("0 3 6 9 12 15").unwrap();
        assert_eq!(history.predict(), 18);
        let history = History::new("3 3 3 3 3").unwrap();
        assert_eq!(history.predict(), 3);
        let history = History::new("0 0 0 0").unwrap();
        assert_eq!(history.predict(), 0);
    }

    #[test]
    fn test_history_extrapolate() {
        let history = History::new("0 3 6 9 12 15").unwrap();
        let extrapolated = history.extrapolate();
        assert_eq!(extrapolated, History { values: vec![0, 3, 6, 9, 12, 15, 18] });

        let history = History::new("3 3 3 3 3").unwrap();
        let extrapolated = history.extrapolate();
        assert_eq!(extrapolated, History { values: vec![3, 3, 3, 3, 3, 3] });

        let history = History::new("0 0 0 0").unwrap();
        let extrapolated = history.extrapolate();
        assert_eq!(extrapolated, History { values: vec![0, 0, 0, 0, 0] });
    }
}