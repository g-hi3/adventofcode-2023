use std::cmp::Ordering;
use std::collections::HashMap;
use crate::camel_cards::Strength::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, Pair, ThreeOfAKind, TwoPairs};

#[derive(Debug)]
pub struct Play {
    hand: Hand,
    bid: u32
}

impl Play {
    pub fn extract(str: &str) -> Vec<Self> {
        str
            .lines()
            .filter_map(Self::new)
            .collect::<Vec<Self>>()
    }

    fn new(str: &str) -> Option<Self> {
        let mut parts = str.split(' ');
        let hand = Hand::new(parts.next()?);
        let bid = parts.next()?.parse::<u32>().ok()?;
        Some(Self { hand, bid })
    }

    pub fn bid(&self) -> u32 {
        self.bid
    }

    pub fn hand(&self) -> String {
        self.hand.cards.iter().map(|card| match card {
            Card::A => 'A',
            Card::K => 'K',
            Card::Q => 'Q',
            Card::J => 'J',
            Card::T => 'T',
            Card::Number(n) => char::from_digit(*n, 10).unwrap(),
        }).collect::<String>()
    }
}

impl PartialEq<Self> for Play {
    fn eq(&self, other: &Self) -> bool {
        self.bid == other.bid
            && self.hand == other.hand
    }
}

impl PartialOrd<Self> for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.hand.cmp(&other.hand))
    }
}

impl Eq for Play {
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand > other.hand {
            Ordering::Greater
        } else if self.hand == other.hand {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }

    fn max(self, other: Self) -> Self where Self: Sized {
        match self.cmp(&other) {
            Ordering::Less => other,
            Ordering::Equal => self,
            Ordering::Greater => self
        }
    }

    fn min(self, other: Self) -> Self where Self: Sized {
        match self.cmp(&other) {
            Ordering::Less => self,
            Ordering::Equal => self,
            Ordering::Greater => other
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self where Self: Sized, Self: PartialOrd {
        self.min(max).max(min)
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>
}

impl Hand {
    fn new(str: &str) -> Self {
        let cards = str
            .chars()
            .filter_map(Card::new)
            .collect::<Vec<Card>>();
        Self { cards }
    }

    fn strength(self: &Self) -> Option<Strength> {
        if self.cards.len() != 5 {
            return None;
        }

        let mut checked_cards = HashMap::<Card, u32>::new();

        for card in &self.cards {
            let card_count = checked_cards.entry(*card).or_insert(0);
            *card_count += 1;
        }

        let cards = checked_cards
            .iter()
            .map(|(card, count)| (card, *count))
            .collect::<Vec<(&Card, u32)>>();
        let mut cards = cards.iter();

        if cards.len() == 1 {
            return Some(FiveOfAKind);
        } else if cards.len() == 2 {
            let (first_card, first_count) = cards.next()?;
            let (second_card, second_count) = cards.next()?;

            if *first_count == 4 && *second_count == 1 {
                return if **first_card == Card::J || **second_card == Card::J {
                    Some(FiveOfAKind)
                } else {
                    Some(FourOfAKind)
                }
            } else if *first_count == 1 && *second_count == 4 {
                return if **first_card == Card::J || **second_card == Card::J {
                    Some(FiveOfAKind)
                } else {
                    Some(FourOfAKind)
                }
            } else if *first_count == 3 && *second_count == 2 {
                return if **first_card == Card::J || **second_card == Card::J {
                    Some(FiveOfAKind)
                } else {
                    Some(FullHouse)
                }
            } else if *first_count == 2 && *second_count == 3 {
                return if **first_card == Card::J || **second_card == Card::J {
                    Some(FiveOfAKind)
                } else {
                    Some(FullHouse)
                }
            }
        } else if cards.len() == 3 {
            let (first_card, first_count) = cards.next()?;
            let (second_card, second_count) = cards.next()?;
            let (third_card, third_count) = cards.next()?;

            if *first_count == 3 && *second_count == 1 && *third_count == 1 {
                return if **first_card == Card::J || **second_card == Card::J || **third_card == Card::J {
                    Some(FourOfAKind)
                } else {
                    Some(ThreeOfAKind)
                }
            } else if *first_count == 1 && *second_count == 3 && *third_count == 1 {
                return if **first_card == Card::J || **second_card == Card::J || **third_card == Card::J {
                    Some(FourOfAKind)
                } else {
                    Some(ThreeOfAKind)
                }
            } else if *first_count == 1 && *second_count == 1 && *third_count == 3 {
                return if **first_card == Card::J || **second_card == Card::J || **third_card == Card::J {
                    Some(FourOfAKind)
                } else {
                    Some(ThreeOfAKind)
                }
            } else if *first_count == 2 && *second_count == 2 && *third_count == 1 {
                return if **first_card == Card::J || **second_card == Card::J {
                    Some(FourOfAKind)
                } else if **third_card == Card::J {
                    Some(FullHouse)
                } else {
                    Some(TwoPairs)
                }
            } else if *first_count == 2 && *second_count == 1 && *third_count == 2 {
                return if **first_card == Card::J || **third_card == Card::J {
                    Some(FourOfAKind)
                } else if **second_card == Card::J {
                    Some(FullHouse)
                } else {
                    Some(TwoPairs)
                }
            } else if *first_count == 1 && *second_count == 2 && *third_count == 2 {
                return if **third_card == Card::J || **second_card == Card::J {
                    Some(FourOfAKind)
                } else if **first_card == Card::J {
                    Some(FullHouse)
                } else {
                    Some(TwoPairs)
                }
            }
        } else if cards.len() == 4 {
            let (first_card, first_count) = cards.next()?;
            let (second_card, second_count) = cards.next()?;
            let (third_card, third_count) = cards.next()?;
            let (fourth_card, fourth_count) = cards.next()?;

            if *first_count == 2 && *second_count == 1 && *third_count == 1 && *fourth_count == 1 {
                return if **first_card == Card::J || **second_card == Card::J || **third_card == Card::J || **fourth_card == Card::J {
                    Some(ThreeOfAKind)
                } else {
                    Some(Pair)
                }
            } else if *first_count == 1 && *second_count == 2 && *third_count == 1 && *fourth_count == 1 {
                return if **first_card == Card::J || **second_card == Card::J || **third_card == Card::J || **fourth_card == Card::J {
                    Some(ThreeOfAKind)
                } else {
                    Some(Pair)
                }
            } else if *first_count == 1 && *second_count == 1 && *third_count == 2 && *fourth_count == 1 {
                return if **first_card == Card::J || **second_card == Card::J || **third_card == Card::J || **fourth_card == Card::J {
                    Some(ThreeOfAKind)
                } else {
                    Some(Pair)
                }
            } else if *first_count == 1 && *second_count == 1 && *third_count == 1 && *fourth_count == 2 {
                return if **first_card == Card::J || **second_card == Card::J || **third_card == Card::J || **fourth_card == Card::J {
                    Some(ThreeOfAKind)
                } else {
                    Some(Pair)
                }
            }
        } else if cards.len() == 5 {
            let (first_card, _) = cards.next()?;
            let (second_card, _) = cards.next()?;
            let (third_card, _) = cards.next()?;
            let (fourth_card, _) = cards.next()?;
            let (fifth_card, _) = cards.next()?;

            return if **first_card == Card::J
                || **second_card == Card::J
                || **third_card == Card::J
                || **fourth_card == Card::J
                || **fifth_card == Card::J {
                Some(Pair)
            } else {
                Some(HighCard)
            }
        }

        None
    }

    fn get_remaining_two_pairs(first_card: &&Card, second_card: &&Card) -> (Card, Card) {
        if first_card.gt(second_card) {
            (**first_card, **second_card)
        } else {
            (**second_card, **first_card)
        }
    }

    fn get_remaining_two(second_card: &&Card, third_card: &&Card) -> (Card, Card) {
        if second_card.gt(third_card) {
            (**second_card, **third_card)
        } else {
            (**third_card, **second_card)
        }
    }

    fn get_remaining_three(second_card: &&Card, third_card: &&Card, fourth_card: &&Card) -> (Card, Card, Card) {
        if second_card.ge(third_card) && third_card.ge(fourth_card) {
            (**second_card, **third_card, **fourth_card)
        } else if second_card.ge(fourth_card) && fourth_card.ge(third_card) {
            (**second_card, **fourth_card, **third_card)
        } else if third_card.ge(second_card) && second_card.ge(fourth_card) {
            (**third_card, **second_card, **fourth_card)
        } else if third_card.ge(fourth_card) && fourth_card.ge(second_card) {
            (**third_card, **fourth_card, **second_card)
        } else if fourth_card.ge(second_card) && second_card.ge(third_card) {
            (**fourth_card, **second_card, **third_card)
        } else {
            (**fourth_card, **third_card, **second_card)
        }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength() == other.strength()
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.strength() {
            None => None,
            Some(strength) => match other.strength() {
                None => None,
                Some(other_strength) => {
                    match strength.cmp(&other_strength) {
                        Ordering::Less => Some(Ordering::Less),
                        Ordering::Greater => Some(Ordering::Greater),
                        Ordering::Equal => {
                            for i in 0..5 {
                                if self.cards[i] > other.cards[i] {
                                    return Some(Ordering::Greater);
                                } else if self.cards[i] < other.cards[i] {
                                    return Some(Ordering::Less);
                                }
                            }

                            Some(Ordering::Equal)
                        }
                    }
                }
            }
        }
    }
}

impl Eq for Hand {
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[derive(Debug)]
enum Strength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    Pair,
    HighCard
}

impl PartialEq<Self> for Strength {
    fn eq(&self, other: &Self) -> bool {
        match self {
            FiveOfAKind => match other {
                FiveOfAKind => true,
                _ => false
            }
            FourOfAKind => match other {
                FourOfAKind => true,
                _ => false
            }
            FullHouse => match other {
                FullHouse => true,
                _ => false
            }
            ThreeOfAKind => match other {
                ThreeOfAKind => true,
                _ => false
            }
            TwoPairs => match other {
                TwoPairs => true,
                _ => false
            }
            Pair => match other {
                Pair => true,
                _ => false
            }
            HighCard => match other {
                HighCard => true,
                _ => false
            }
        }
    }
}

impl PartialOrd<Self> for Strength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            FiveOfAKind => match other {
                FiveOfAKind => Some(Ordering::Equal),
                _ => Some(Ordering::Greater)
            }
            FourOfAKind => match other {
                FiveOfAKind => Some(Ordering::Less),
                FourOfAKind => Some(Ordering::Equal),
                _ => Some(Ordering::Greater)
            }
            FullHouse => match other {
                FiveOfAKind => Some(Ordering::Less),
                FourOfAKind => Some(Ordering::Less),
                FullHouse => Some(Ordering::Equal),
                _ => Some(Ordering::Greater)
            }
            ThreeOfAKind => match other {
                FiveOfAKind => Some(Ordering::Less),
                FourOfAKind => Some(Ordering::Less),
                FullHouse => Some(Ordering::Less),
                ThreeOfAKind => Some(Ordering::Equal),
                _ => Some(Ordering::Greater)
            }
            TwoPairs => match other {
                TwoPairs => Some(Ordering::Equal),
                Pair => Some(Ordering::Greater),
                HighCard => Some(Ordering::Greater),
                _ => Some(Ordering::Less)
            }
            Pair => match other {
                Pair => Some(Ordering::Equal),
                HighCard => Some(Ordering::Greater),
                _ => Some(Ordering::Less)
            }
            HighCard => match other {
                HighCard => Some(Ordering::Equal),
                _ => Some(Ordering::Less)
            }
        }
    }
}

impl Eq for Strength {
}

impl Ord for Strength {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Card {
    A, K, Q, J, T, Number(u32)
}

impl Card {
    fn new(c: char) -> Option<Self> {
        match c {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            '2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => c.to_digit(10).map(|digit| Card::Number(digit)),
            _ => None
        }
    }
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Card::A => match other {
                Card::A => true,
                _ => false
            }
            Card::K => match other {
                Card::K => true,
                _ => false
            },
            Card::Q => match other {
                Card::Q => true,
                _ => false
            },
            Card::J => match other {
                Card::J => true,
                _ => false
            },
            Card::T => match other {
                Card::T => true,
                _ => false
            },
            Card::Number(n) => match other {
                Card::Number(x) => n == x,
                _ => false
            }
        }
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Card::A => match other {
                Card::A => Ordering::Equal,
                _ => Ordering::Greater
            }
            Card::K => match other {
                Card::A => Ordering::Less,
                Card::K => Ordering::Equal,
                _ => Ordering::Greater
            }
            Card::Q => match other {
                Card::A => Ordering::Less,
                Card::K => Ordering::Less,
                Card::Q => Ordering::Equal,
                _ => Ordering::Greater
            }
            Card::J => match other {
                Card::J => Ordering::Equal,
                _ => Ordering::Less
            }
            Card::T => match other {
                Card::T => Ordering::Equal,
                Card::Number(_) => Ordering::Greater,
                Card::J => Ordering::Greater,
                _ => Ordering::Less
            }
            Card::Number(number) => match other {
                Card::Number(other_number) => number.cmp(other_number),
                Card::J => Ordering::Greater,
                _ => Ordering::Less
            }
        })
    }
}

impl Eq for Card {
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::camel_cards::{Card, Hand, Play, Strength};
    use crate::camel_cards::Strength::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, Pair, ThreeOfAKind, TwoPairs};

    #[test]
    fn test_new_card() {
        let card = Card::new('A');
        assert_eq!(card, Some(Card::A));
        let card = Card::new('K');
        assert_eq!(card, Some(Card::K));
        let card = Card::new('Q');
        assert_eq!(card, Some(Card::Q));
        let card = Card::new('J');
        assert_eq!(card, Some(Card::J));
        let card = Card::new('T');
        assert_eq!(card, Some(Card::T));
        let card = Card::new('9');
        assert_eq!(card, Some(Card::Number(9)));
        let card = Card::new('8');
        assert_eq!(card, Some(Card::Number(8)));
        let card = Card::new('7');
        assert_eq!(card, Some(Card::Number(7)));
        let card = Card::new('6');
        assert_eq!(card, Some(Card::Number(6)));
        let card = Card::new('5');
        assert_eq!(card, Some(Card::Number(5)));
        let card = Card::new('4');
        assert_eq!(card, Some(Card::Number(4)));
        let card = Card::new('3');
        assert_eq!(card, Some(Card::Number(3)));
        let card = Card::new('2');
        assert_eq!(card, Some(Card::Number(2)));
        let card = Card::new('1');
        assert_eq!(card, None);
        let card = Card::new('0');
        assert_eq!(card, None);
        let card = Card::new(' ');
        assert_eq!(card, None);
    }

    #[test]
    fn test_card_equality() {
        let card = Card::Number(4);
        let other_card = Card::Number(4);
        assert_eq!(card, other_card);
    }

    #[test]
    fn test_new_hand() {
        let hand = Hand::new("32T3K");
        assert_eq!(hand, Hand {
            cards: vec![
                Card::Number(3),
                Card::Number(2),
                Card::T,
                Card::Number(3),
                Card::K
            ]
        });

        let hand = Hand::new("T55J5");
        assert_eq!(hand, Hand {
            cards: vec![
                Card::T,
                Card::Number(5),
                Card::Number(5),
                Card::J,
                Card::Number(5)
            ]
        });

        let hand = Hand::new("KK677");
        assert_eq!(hand, Hand {
            cards: vec![
                Card::K,
                Card::K,
                Card::Number(6),
                Card::Number(7),
                Card::Number(7)
            ]
        });

        let hand = Hand::new("KTJJT");
        assert_eq!(hand, Hand {
            cards: vec![
                Card::K,
                Card::T,
                Card::J,
                Card::J,
                Card::T
            ]
        });

        let hand = Hand::new("QQQJA");
        assert_eq!(hand, Hand {
            cards: vec![
                Card::Q,
                Card::Q,
                Card::Q,
                Card::J,
                Card::A
            ]
        });
    }

    #[test]
    fn test_extract_play() {
        let plays = Play::extract("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        assert_eq!(plays, vec![
            Play::new("32T3K 765").unwrap(),
            Play::new("T55J5 684").unwrap(),
            Play::new("KK677 28").unwrap(),
            Play::new("KTJJT 220").unwrap(),
            Play::new("QQQJA 483").unwrap()
        ]);
    }

    #[test]
    fn test_new_play() {
        let play = Play::new("32T3K 765");
        assert_eq!(play, Some(Play {
            hand: Hand::new("32T3K"),
            bid: 765
        }));
    }

    #[test]
    fn test_strength() {
        let hand = Hand::new("KKKKK");
        assert_eq!(hand.strength(), Some(FiveOfAKind));
        let hand = Hand::new("99999");
        assert_eq!(hand.strength(), Some(FiveOfAKind));
        let hand = Hand::new("99599");
        assert_eq!(hand.strength(), Some(FourOfAKind));
        let hand = Hand::new("KQQQQ");
        assert_eq!(hand.strength(), Some(FourOfAKind));
        let hand = Hand::new("333KK");
        assert_eq!(hand.strength(), Some(FullHouse));
        let hand = Hand::new("Q3Q3Q");
        assert_eq!(hand.strength(), Some(FullHouse));
        let hand = Hand::new("4J46J");
        assert_eq!(hand.strength(), Some(FourOfAKind));
        let hand = Hand::new("37583");
        assert_eq!(hand.strength(), Some(Pair));
        let hand = Hand::new("TJJQK");
        assert_eq!(hand.strength(), Some(ThreeOfAKind));
        let hand = Hand::new("4ATJ3");
        assert_eq!(hand.strength(), Some(Pair));
    }

    #[test]
    fn test_ranks() {
        let plays = Play::extract("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        let mut plays = plays.iter()
            .map(|play| &play.hand)
            .collect::<Vec<&Hand>>();
        plays.sort();
        assert_eq!(plays, vec![
            &Hand::new("32T3K"),
            &Hand::new("KK677"),
            &Hand::new("T55J5"),
            &Hand::new("QQQJA"),
            &Hand::new("KTJJT")
        ]);
    }

    #[test]
    fn test_day7() {
        let strength1 = Strength::ThreeOfAKind;
        let strength2 = Strength::Pair;
        assert_eq!(strength1.cmp(&strength2), Ordering::Greater);

        let plays = Play::extract("KK677 28");
        let play1 = plays.get(0).unwrap();
        let plays = Play::extract("QQQJA 483");
        let play2 = plays.get(0).unwrap();
        let strength1 = play1.hand.strength().unwrap();
        let strength2 = play2.hand.strength().unwrap();
        println!("{:?} < {:?}", strength1, strength2);
        assert_eq!(strength1.cmp(&strength2), Ordering::Less);

        let mut plays = Play::extract("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        plays.sort();
        assert_eq!(plays, vec![
            Play { bid: 765, hand: Hand::new("32T3K") },
            Play { bid: 28, hand: Hand::new("KK677") },
            Play { bid: 684, hand: Hand::new("T55J5") },
            Play { bid: 483, hand: Hand::new("QQQJA") },
            Play { bid: 220, hand: Hand::new("KTJJT") }
        ]);

        let ranks = plays
            .iter()
            .enumerate()
            .map(|(index, play)| {
                println!("{} * ({index} + 1) = {}", play.bid, play.bid as u64 * (index as u64 + 1));
                play.bid as u64 * (index as u64 + 1)
            })
            .sum::<u64>();
        assert_eq!(ranks, 5905);
    }
}