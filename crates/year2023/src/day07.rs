use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Card {
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Tapir,
    Joker,
    Queen,
    King,
    Ace,
}

#[test]
fn card_score() {
    let cards = "AKQJT98765432"
        .chars()
        .map(|elem| Card::try_from(elem).unwrap())
        .collect::<Vec<_>>();
    assert!(cards.windows(2).all(|window| window[0] > window[1]));
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Tapir,
            '9' => Self::Num9,
            '8' => Self::Num8,
            '7' => Self::Num7,
            '6' => Self::Num6,
            '5' => Self::Num5,
            '4' => Self::Num4,
            '3' => Self::Num3,
            '2' => Self::Num2,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    Kind3,
    FullHouse,
    Kind4,
    Kind5,
}

#[test]
fn hand_category_order() {
    assert!(HandCategory::Kind5 > HandCategory::Kind4);
    assert!(HandCategory::Kind4 > HandCategory::FullHouse);
    assert!(HandCategory::FullHouse > HandCategory::Kind3);
    assert!(HandCategory::Kind3 > HandCategory::TwoPair);
    assert!(HandCategory::TwoPair > HandCategory::OnePair);
    assert!(HandCategory::OnePair > HandCategory::HighCard);
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand([Card; 5]);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn category(&self) -> HandCategory {
        let mut cards = self.0.clone();
        cards.sort();

        let card_counts: BTreeMap<_, _> = cards
            .iter()
            .map(|elem| {
                (
                    elem.clone(),
                    cards.iter().filter(|&elem2| elem == elem2).count(),
                )
            })
            .collect();

        match card_counts.iter().map(|elem| elem.1).max() {
            Some(1) => HandCategory::HighCard,
            Some(4) => HandCategory::Kind4,
            Some(5) => HandCategory::Kind5,
            Some(2) => {
                if card_counts.len() == 3 {
                    HandCategory::TwoPair
                } else {
                    HandCategory::OnePair
                }
            }
            Some(3) => {
                if card_counts.len() == 2 {
                    HandCategory::FullHouse
                } else {
                    HandCategory::Kind3
                }
            }
            _ => unreachable!("There are only five cards in a hand"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let category = self.category().cmp(&other.category());

        if category.is_ne() {
            category
        } else {
            self.0.cmp(&other.0)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AlternativeHand([Card; 5]);
impl AlternativeHand {
    fn category(&self) -> HandCategory {
        let jokers = self.0.iter().filter(|&card| card == &Card::Joker).count();
        let standard = Hand(self.0.clone()).category();

        match jokers {
            0 | 5 => standard,
            1 => match standard {
                HandCategory::HighCard => HandCategory::OnePair,
                HandCategory::OnePair => HandCategory::Kind3,
                HandCategory::TwoPair => HandCategory::FullHouse,
                HandCategory::Kind3 => HandCategory::Kind4,
                HandCategory::FullHouse => {
                    unreachable!("Only possible with 2 or 3 Joker, but we have 1")
                }
                HandCategory::Kind4 => HandCategory::Kind5,
                HandCategory::Kind5 => unreachable!("Only possible with 5 Joker, but we have 1"),
            },
            2 => match standard {
                HandCategory::HighCard => {
                    unreachable!("Only possible with 0 or 1 Joker, but we have 2")
                }
                HandCategory::OnePair => HandCategory::Kind3,
                HandCategory::TwoPair => HandCategory::Kind4,
                HandCategory::Kind3 => {
                    unreachable!("Only possible with 1 or 3 Joker, but we have 2")
                }
                HandCategory::FullHouse => HandCategory::Kind5,
                HandCategory::Kind4 => {
                    unreachable!("Only possible with 1 or 4 Joker, but we have 2")
                }
                HandCategory::Kind5 => {
                    unreachable!("Only possible with 0 or 5 Joker, but we have 2")
                }
            },
            3 => match standard {
                HandCategory::HighCard => {
                    unreachable!("Only possible with 0 or 1 Joker, but we have 3")
                }
                HandCategory::OnePair => {
                    unreachable!("Only possible with 0-2 Joker, but we have 3")
                }
                HandCategory::TwoPair => {
                    unreachable!("Only possible with 0-2 Joker, but we have 3")
                }
                HandCategory::Kind3 => HandCategory::Kind4,
                HandCategory::FullHouse => HandCategory::Kind5,
                HandCategory::Kind4 => {
                    unreachable!("Only possible with 1 or 4 Joker, but we have 3")
                }
                HandCategory::Kind5 => {
                    unreachable!("Only possible with 0 or 5 Joker, but we have 3")
                }
            },
            4 => HandCategory::Kind5,
            _ => unreachable!("A hand has 5 cards"),
        }
    }
}

impl PartialOrd for AlternativeHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AlternativeHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let category = self.category().cmp(&other.category());

        if category.is_ne() {
            category
        } else {
            fn re_map_cards(old: Card) -> Card {
                match old {
                    Card::Num2 => Card::Num3,
                    Card::Num3 => Card::Num4,
                    Card::Num4 => Card::Num5,
                    Card::Num5 => Card::Num6,
                    Card::Num6 => Card::Num7,
                    Card::Num7 => Card::Num8,
                    Card::Num8 => Card::Num9,
                    Card::Num9 => Card::Tapir,
                    Card::Tapir => Card::Joker,
                    Card::Joker => Card::Num2,
                    Card::Queen => Card::Queen,
                    Card::King => Card::King,
                    Card::Ace => Card::Ace,
                }
            }
            self.0
                .clone()
                .map(re_map_cards)
                .cmp(&other.0.clone().map(re_map_cards))
        }
    }
}

#[test]
fn hand_comparison() {
    let example_a = Hand([Card::King, Card::King, Card::Num6, Card::Num7, Card::Num7]);
    let example_b = Hand([
        Card::King,
        Card::Tapir,
        Card::Joker,
        Card::Joker,
        Card::Tapir,
    ]);
    assert!(example_a.cmp(&example_b).is_ge());

    let example_a = Hand([
        Card::Queen,
        Card::Queen,
        Card::Queen,
        Card::Joker,
        Card::Ace,
    ]);
    let example_b = Hand([Card::Tapir, Card::Num5, Card::Num5, Card::Joker, Card::Num5]);
    assert!(example_a.cmp(&example_b).is_ge());
}

struct Bet {
    hand: Hand,
    amount: u16,
}

impl FromStr for Bet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bet) = s.split_once(' ').ok_or(())?;
        let hand = cards
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| ())?;
        Ok(Self {
            hand: Hand(hand),
            amount: bet.parse().map_err(|_| ())?,
        })
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Bet> + '_ {
    input.lines().flat_map(Bet::from_str)
}

pub fn part1(input: &str) -> usize {
    let mut bets = parse_input(input).collect::<Vec<_>>();
    bets.sort_by_key(|bet| bet.hand.clone());

    bets.iter()
        .enumerate()
        .map(|(idx, bet)| (idx + 1) * bet.amount as usize)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut bets = parse_input(input).collect::<Vec<_>>();
    bets.sort_by_key(|bet| AlternativeHand(bet.hand.0.clone()));

    bets.iter()
        .enumerate()
        .map(|(idx, bet)| (idx + 1) * bet.amount as usize)
        .sum()
}

#[test]
#[allow(clippy::identity_op)]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day07.example.txt"
    ));
    assert_eq!(part1(input), 765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day07.txt"
    ));
    assert_eq!(part1(input), 248396258);
}

#[test]
fn part2_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day07.example.txt"
    ));
    assert_eq!(part2(input), 5905);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day07.txt"
    ));
    assert_eq!(part2(input), 246436046);
}
