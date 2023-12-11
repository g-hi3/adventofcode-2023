use std::cmp::Ordering;
use std::collections::HashMap;
use crate::camel_cards::Strength::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, Pair, ThreeOfAKind, TwoPairs};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

    fn is_five_of_a_kind(self: &Self) -> bool {
        if self.cards.len() != 5 {
            return false;
        }

        let first_card = self.cards.first();
        match first_card {
            None => false,
            Some(first_card) => {
                for i in 0..5 {
                    if let Some(card) = self.cards.get(i) {
                        if card != first_card {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                return true;
            }
        }
    }

    fn is_four_of_a_kind(self: &Self) -> bool {
        if self.cards.len() != 5 {
            return false;
        }

        let mut cards = self.cards.iter();
        let first_card = match cards.next() {
            None => return false,
            Some(first_card) => first_card
        };
        let mut first_card_count = 1_u32;
        let mut second_card = Option::<Card>::None;
        let mut second_card_count = 0_u32;

        for card in cards {
            if card == first_card {
                first_card_count += 1;
                continue;
            }

            match second_card {
                None => {
                    second_card = Some(*card);
                    second_card_count += 1;
                    continue;
                }
                Some(second_card) => {
                    if *card == second_card {
                        second_card_count += 1;
                    } else {
                        return false;
                    }
                }
            }
        }

        println!("{first_card:?} -> {first_card_count}\n{second_card:?} -> {second_card_count}\n");
        first_card_count == 4 && second_card_count == 1
            || first_card_count == 1 && second_card_count == 4
    }
}

#[derive(Debug, PartialEq)]
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
                Card::T => Ordering::Less,
                Card::Number(_) => Ordering::Less,
                _ => Ordering::Greater
            }
            Card::T => match other {
                Card::T => Ordering::Equal,
                Card::Number(_) => Ordering::Less,
                _ => Ordering::Greater
            }
            Card::Number(number) => match other {
                Card::Number(other) => number.cmp(other),
                _ => Ordering::Greater
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::camel_cards::{Card, Hand, Play, Strength};

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
    fn test_card_is_five_of_a_kind() {
        let hand = Hand::new("KKKKK");
        assert!(hand.is_five_of_a_kind());
        let hand = Hand::new("99999");
        assert!(hand.is_five_of_a_kind());
        let hand = Hand::new("TT");
        assert!(!hand.is_five_of_a_kind());
        let hand = Hand::new("JJJJJJJJ");
        assert!(!hand.is_five_of_a_kind());
    }

    #[test]
    fn test_card_is_four_of_a_kind() {
        let hand = Hand::new("KKKKQ");
        assert!(hand.is_four_of_a_kind());
        let hand = Hand::new("9999K");
        assert!(hand.is_four_of_a_kind());
        let hand = Hand::new("KKQKK");
        assert!(hand.is_four_of_a_kind());
        let hand = Hand::new("T9999");
        assert!(hand.is_four_of_a_kind());
        let hand = Hand::new("TT");
        assert!(!hand.is_four_of_a_kind());
        let hand = Hand::new("JJJJJJJJ");
        assert!(!hand.is_four_of_a_kind());
        let hand = Hand::new("55555");
        assert!(!hand.is_four_of_a_kind());
    }
}