use std::cmp::Ordering;
use std::collections::HashMap;
use crate::camel_cards::Strength::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, Pair, ThreeOfAKind, TwoPairs};

#[derive(Debug, PartialEq, Eq, Ord)]
struct Play {
    hand: Hand,
    bid: u32
}

impl Play {
    fn extract(str: &str) -> Vec<Self> {
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
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.hand.cmp(&other.hand))
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
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
            let high_card = *cards.next()?.0;
            return Some(FiveOfAKind(high_card));
        } else if cards.len() == 2 {
            let (first_card, first_count) = cards.next()?;
            let (second_card, second_count) = cards.next()?;

            if *first_count == 4 && *second_count == 1 {
                return Some(FourOfAKind {
                    four: **first_card,
                    one: **second_card
                });
            } else if *first_count == 1 && *second_count == 4 {
                return Some(FourOfAKind {
                    four: **second_card,
                    one: **first_card
                });
            } else if *first_count == 3 && *second_count == 2 {
                return Some(FullHouse {
                    three: **first_card,
                    two: **second_card
                });
            } else if *first_count == 2 && *second_count == 3 {
                return Some(FullHouse {
                    three: **second_card,
                    two: **first_card
                });
            }
        } else if cards.len() == 3 {
            let (first_card, first_count) = cards.next()?;
            let (second_card, second_count) = cards.next()?;
            let (third_card, third_count) = cards.next()?;

            if *first_count == 3 && *second_count == 1 && *third_count == 1 {
                let (fourth, fifth) = Self::get_remaining_two(second_card, third_card);
                return Some(ThreeOfAKind {
                    three: **first_card,
                    fourth,
                    fifth
                });
            } else if *first_count == 1 && *second_count == 3 && *third_count == 1 {
                let (fourth, fifth) = Self::get_remaining_two(first_card, third_card);
                return Some(ThreeOfAKind {
                    three: **second_card,
                    fourth,
                    fifth
                });
            } else if *first_count == 1 && *second_count == 1 && *third_count == 3 {
                let (fourth, fifth) = Self::get_remaining_two(first_card, second_card);
                return Some(ThreeOfAKind {
                    three: **third_card,
                    fourth,
                    fifth
                });
            } else if *first_count == 2 && *second_count == 2 && *third_count == 1 {
                let (pair1, pair2) = Self::get_remaining_two_pairs(first_card, second_card);
                return Some(TwoPairs {
                    pair1,
                    pair2,
                    fifth: **third_card
                });
            } else if *first_count == 2 && *second_count == 1 && *third_count == 2 {
                let (pair1, pair2) = Self::get_remaining_two_pairs(first_card, third_card);
                return Some(TwoPairs {
                    pair1,
                    pair2,
                    fifth: **second_card
                });
            } else if *first_count == 1 && *second_count == 2 && *third_count == 2 {
                let (pair1, pair2) = Self::get_remaining_two_pairs(second_card, third_card);
                return Some(TwoPairs {
                    pair1,
                    pair2,
                    fifth: **first_card
                });
            }
        } else if cards.len() == 4 {
            let (first_card, first_count) = cards.next()?;
            let (second_card, second_count) = cards.next()?;
            let (third_card, third_count) = cards.next()?;
            let (fourth_card, fourth_count) = cards.next()?;

            if *first_count == 2 && *second_count == 1 && *third_count == 1 && *fourth_count == 1 {
                let (third, fourth, fifth) = Self::get_remaining_three(second_card, third_card, fourth_card);
                return Some(Pair {
                    two: **first_card,
                    third,
                    fourth,
                    fifth
                });
            } else if *first_count == 1 && *second_count == 2 && *third_count == 1 && *fourth_count == 1 {
                let (third, fourth, fifth) = Self::get_remaining_three(first_card, third_card, fourth_card);
                return Some(Pair {
                    two: **second_card,
                    third,
                    fourth,
                    fifth
                });
            } else if *first_count == 1 && *second_count == 1 && *third_count == 2 && *fourth_count == 1 {
                let (third, fourth, fifth) = Self::get_remaining_three(first_card, second_card, fourth_card);
                return Some(Pair {
                    two: **third_card,
                    third,
                    fourth,
                    fifth
                });
            } else if *first_count == 1 && *second_count == 1 && *third_count == 1 && *fourth_count == 2 {
                let (third, fourth, fifth) = Self::get_remaining_three(first_card, second_card, third_card);
                return Some(Pair {
                    two: **fourth_card,
                    third,
                    fourth,
                    fifth
                });
            }
        } else if cards.len() == 5 {
            let (first_card, _) = cards.next()?;
            let (second_card, _) = cards.next()?;
            let (third_card, _) = cards.next()?;
            let (fourth_card, _) = cards.next()?;
            let (fifth_card, _) = cards.next()?;
            let mut cards = vec![first_card, second_card, third_card, fourth_card, fifth_card];
            cards.sort();
            cards.reverse();
            let mut cards = cards.iter();

            return Some(HighCard {
                high: ***cards.next()?,
                second: ***cards.next()?,
                third: ***cards.next()?,
                fourth: ***cards.next()?,
                fifth: ***cards.next()?,
            });
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

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.strength() {
            None => None,
            Some(strength) => match other.strength() {
                None => None,
                Some(other_strength) => Some(strength.cmp(&other_strength))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
enum Strength {
    FiveOfAKind(Card),
    FourOfAKind { four: Card, one: Card },
    FullHouse { three: Card, two: Card },
    ThreeOfAKind { three: Card, fourth: Card, fifth: Card },
    TwoPairs { pair1: Card, pair2: Card, fifth: Card },
    Pair { two: Card, third: Card, fourth: Card, fifth: Card },
    HighCard { high: Card, second: Card, third: Card, fourth: Card, fifth: Card }
}

impl Strength {
    fn from(hand: &Hand) -> Option<Strength> {
        let mut card_map = HashMap::<Card, u32>::new();

        for card in &hand.cards {
            let value = card_map.entry(*card).or_insert(0);
            *value += 1;
        }

        let mut cards = card_map.iter().collect::<Vec<(&Card, &u32)>>().clone();
        cards.sort_by(|left, right| match left.1.cmp(right.1) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => left.0.cmp(right.0),
            Ordering::Greater => Ordering::Greater
        });

        for (card, count) in &card_map {
            println!("{card:?} -> {count}");
        }
        println!();

        if card_map.len() == 5 {
            let mut cards = cards.iter();
            Some(HighCard {
                fifth: cards.next().map(|(&card, _)| card.clone())?,
                fourth: cards.next().map(|(&card, _)| card.clone())?,
                third: cards.next().map(|(&card, _)| card.clone())?,
                second: cards.next().map(|(&card, _)| card.clone())?,
                high: cards.next().map(|(&card, _)| card.clone())?
            })
        } else if card_map.len() == 4 {
            let mut cards = cards.iter();
            Some(Pair {
                fifth: cards.next().map(|(&card, _)| card.clone())?,
                fourth: cards.next().map(|(&card, _)| card.clone())?,
                third: cards.next().map(|(&card, _)| card.clone())?,
                two: cards.next().map(|(&card, _)| card.clone())?
            })
        } else if card_map.len() == 3 {
            let mut cards = cards.iter();
            let first = cards.next().map(|(&card, _)| card.clone())?;
            let fourth = cards.next().map(|(&card, _)| card.clone())?;
            let fifth = cards.next().map(|(&card, _)| card.clone())?;
            if let Some((_, count)) = card_map.get_key_value(&fifth) {
                if *count == 3 {
                    Some(ThreeOfAKind {
                        three: fifth,
                        fourth,
                        fifth: first
                    })
                } else {
                    Some(TwoPairs {
                        fifth: first,
                        pair2: fifth,
                        pair1: fourth
                    })
                }
            } else {
                None
            }
        } else if card_map.len() == 2 {
            let mut cards = cards.iter();
            let first = cards.next().map(|(&card, _)| card.clone())?;
            let fifth = cards.next().map(|(&card, _)| card.clone())?;
            if let Some((_, count)) = card_map.get_key_value(&first) {
                if *count == 3 {
                    Some(FullHouse {
                        three: fifth,
                        two: first
                    })
                } else {
                    Some(FourOfAKind {
                        one: first,
                        four: fifth
                    })
                }
            } else {
                None
            }
        } else if card_map.len() == 1 {
            let mut cards = cards.iter();
            Some(FiveOfAKind(cards.next().map(|(&card, _)| card.clone())?))
        } else {
            None
        }
    }
}

impl PartialOrd for Strength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            FiveOfAKind(card) => match other {
                FiveOfAKind(other_card) => Some(card.cmp(other_card)),
                _ => Some(Ordering::Greater)
            }
            FourOfAKind { four, one } => match other {
                FiveOfAKind(_) => Some(Ordering::Less),
                FourOfAKind { four: other_four, one: other_one } => match four.cmp(other_four) {
                    Ordering::Equal => Some(one.cmp(other_one)),
                    ordering => Some(ordering)
                },
                _ => Some(Ordering::Greater)
            }
            FullHouse { three, two } => match other {
                FiveOfAKind(_) => Some(Ordering::Less),
                FourOfAKind { .. } => Some(Ordering::Less),
                FullHouse { three: other_three, two: other_two } => match three.cmp(other_three) {
                    Ordering::Equal => Some(two.cmp(other_two)),
                    ordering => Some(ordering)
                }
                _ => Some(Ordering::Greater)
            }
            ThreeOfAKind { three, fourth, fifth } => match other {
                FiveOfAKind(_) => Some(Ordering::Less),
                FourOfAKind { .. } => Some(Ordering::Less),
                FullHouse { .. } => Some(Ordering::Less),
                ThreeOfAKind { three: other_three, fourth: other_fourth, fifth: other_fifth } => match three.cmp(other_three) {
                    Ordering::Equal => match fourth.cmp(other_fourth) {
                        Ordering::Equal => Some(fifth.cmp(other_fifth)),
                        ordering => Some(ordering)
                    }
                    ordering => Some(ordering)
                }
                _ => Some(Ordering::Greater),
            }
            TwoPairs { pair1, pair2, fifth } => match other {
                TwoPairs { pair1: other_pair1, pair2: other_pair2, fifth: other_fifth } => match pair1.cmp(other_pair1) {
                    Ordering::Equal => match pair2.cmp(other_pair2) {
                        Ordering::Equal => Some(fifth.cmp(other_fifth)),
                        ordering => Some(ordering)
                    }
                    ordering => Some(ordering)
                }
                Pair { .. } => Some(Ordering::Greater),
                HighCard { .. } => Some(Ordering::Greater),
                _ => Some(Ordering::Less),
            }
            Pair { two, third, fourth, fifth } => match other {
                Pair { two: other_two, third: other_third, fourth: other_fourth, fifth: other_fifth } => match two.cmp(other_two) {
                    Ordering::Equal => match third.cmp(other_third) {
                        Ordering::Equal => match fourth.cmp(other_fourth) {
                            Ordering::Equal => Some(fifth.cmp(other_fifth)),
                            ordering => Some(ordering)
                        }
                        ordering => Some(ordering)
                    }
                    ordering => Some(ordering)
                }
                HighCard { .. } => Some(Ordering::Greater),
                _ => Some(Ordering::Less)
            }
            HighCard { high, second, third, fourth, fifth } => match other {
                HighCard { high: other_high, second: other_second, third: other_third, fourth: other_fourth, fifth: other_fifth } => match high.cmp(other_high) {
                    Ordering::Equal => match second.cmp(other_second) {
                        Ordering::Equal => match third.cmp(other_third) {
                            Ordering::Equal => match fourth.cmp(other_fourth) {
                                Ordering::Equal => Some(fifth.cmp(other_fifth)),
                                ordering => Some(ordering)
                            }
                            ordering => Some(ordering)
                        }
                        ordering => Some(ordering)
                    }
                    ordering => Some(ordering)
                },
                _ => Some(Ordering::Less)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash)]
enum Card {
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

impl PartialOrd for Card {
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
                Card::T => Ordering::Greater,
                Card::Number(_) => Ordering::Greater,
                _ => Ordering::Less
            }
            Card::T => match other {
                Card::T => Ordering::Equal,
                Card::Number(_) => Ordering::Greater,
                _ => Ordering::Less
            }
            Card::Number(number) => match other {
                Card::Number(other) => number.cmp(other),
                _ => Ordering::Less
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::camel_cards::{Card, Hand, Play, Strength};
    use crate::camel_cards::Strength::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, Pair, TwoPairs};

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
    fn test_strength_from_hand() {
        let hand = Hand::new("KKKKK");
        assert_eq!(Strength::from(&hand), Some(Strength::FiveOfAKind(Card::K)));
        let hand = Hand::new("KKQKK");
        assert_eq!(Strength::from(&hand), Some(Strength::FourOfAKind { four: Card::K, one: Card::Q }));
        let hand = Hand::new("QKQQQ");
        assert_eq!(Strength::from(&hand), Some(Strength::FourOfAKind { four: Card::Q, one: Card::K }));
        let hand = Hand::new("QQKJK");
        assert_eq!(Strength::from(&hand), Some(Strength::TwoPairs { pair1: Card::K, pair2: Card::Q, fifth: Card::J }));
        let hand = Hand::new("93993");
        assert_eq!(Strength::from(&hand), Some(Strength::FullHouse { three: Card::Number(9), two: Card::Number(3) }));
        let hand = Hand::new("93393");
        assert_eq!(Strength::from(&hand), Some(Strength::FullHouse { three: Card::Number(3), two: Card::Number(9) }));
    }

    #[test]
    fn test_strength() {
        let hand = Hand::new("KKKKK");
        assert_eq!(hand.strength(), Some(FiveOfAKind(Card::K)));
        let hand = Hand::new("99999");
        assert_eq!(hand.strength(), Some(FiveOfAKind(Card::Number(9))));
        let hand = Hand::new("99599");
        assert_eq!(hand.strength(), Some(FourOfAKind{
            four: Card::Number(9),
            one: Card::Number(5)
        }));
        let hand = Hand::new("KQQQQ");
        assert_eq!(hand.strength(), Some(FourOfAKind{
            four: Card::Q,
            one: Card::K
        }));
        let hand = Hand::new("333KK");
        assert_eq!(hand.strength(), Some(FullHouse {
            three: Card::Number(3),
            two: Card::K
        }));
        let hand = Hand::new("Q3Q3Q");
        assert_eq!(hand.strength(), Some(FullHouse {
            three: Card::Q,
            two: Card::Number(3)
        }));
        let hand = Hand::new("4J46J");
        assert_eq!(hand.strength(), Some(TwoPairs {
            pair1: Card::J,
            pair2: Card::Number(4),
            fifth: Card::Number(6)
        }));
        let hand = Hand::new("37583");
        assert_eq!(hand.strength(), Some(Pair {
            two: Card::Number(3),
            third: Card::Number(8),
            fourth: Card::Number(7),
            fifth: Card::Number(5)
        }));
        let hand = Hand::new("TJJQK");
        assert_eq!(hand.strength(), Some(Pair {
            two: Card::J,
            third: Card::K,
            fourth: Card::Q,
            fifth: Card::T
        }));
        let hand = Hand::new("4ATJ3");
        assert_eq!(hand.strength(), Some(HighCard {
            high: Card::A,
            second: Card::J,
            third: Card::T,
            fourth: Card::Number(4),
            fifth: Card::Number(3)
        }));
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
            &Hand::new("KTJJT"),
            &Hand::new("KK677"),
            &Hand::new("T55J5"),
            &Hand::new("QQQJA"),
        ]);
    }

    #[test]
    fn test_day7() {
        let strength1 = Strength::ThreeOfAKind {
            three: Card::Q,
            fourth: Card::A,
            fifth: Card::J
        };
        let strength2 = Strength::Pair {
            two: Card::Number(3),
            third: Card::K,
            fourth: Card::T,
            fifth: Card::Number(2)
        };
        assert_eq!(strength1.cmp(&strength2), Ordering::Less);

        let plays = Play::extract("QQQJA 483");
        let play1 = plays.get(0).unwrap();
        let plays = Play::extract("32T3K 765");
        let play2 = plays.get(0).unwrap();
        let strength1 = play1.hand.strength().unwrap();
        let strength2 = play2.hand.strength().unwrap();
        assert_eq!(strength1.cmp(&strength2), Ordering::Less);

        let mut plays = Play::extract("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        plays.sort();
        assert_eq!(plays, vec![
            Play { bid: 483, hand: Hand::new("QQQJA") },
            Play { bid: 684, hand: Hand::new("T55J5") },
            Play { bid: 28, hand: Hand::new("KK677") },
            Play { bid: 220, hand: Hand::new("KTJJT") },
            Play { bid: 765, hand: Hand::new("32T3K") }
        ]);

        let ranks = plays
            .iter()
            .enumerate()
            .map(|(index, play)| play.bid as u64 * (index as u64 + 1))
            .sum::<u64>();
        assert_eq!(ranks, 6440);
    }
}