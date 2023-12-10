use std::cmp::Ordering;
use std::marker::PhantomData;
use std::iter;
use std::str::FromStr;

const HAND_SIZE: usize = 5;
const CARDS: usize = 13;

trait Part: Sized + Eq {
    fn card_num(c: char) -> u8;
    fn sort_counts(counts: &mut [u8; CARDS]);

    fn parse_input(input: String) -> Vec<(Hand<Self>, u32)> {
        input.lines()
            .map(|l| {
                let (hand_s, bid_s) = l.split_once(' ').unwrap();
                (hand_s.parse().unwrap(), bid_s.parse().unwrap())
            })
            .collect()
    }
}

#[derive(PartialEq, Eq)]
struct Part1;

impl Part for Part1 {
    fn card_num(c: char) -> u8 {
        let n = match c {
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card character"),
        };
        n - 2
    }

    fn sort_counts(counts: &mut [u8; CARDS]) {
        // Sort in descending order
        counts.sort_by_key(|n| - (*n as i8));
    }
}

#[derive(PartialEq, Eq)]
struct Part2;

impl Part for Part2 {
    fn card_num(c: char) -> u8 {
        let n = match c {
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 1,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => panic!("Invalid card character"),
        };
        n - 1
    }

    fn sort_counts(counts: &mut [u8; CARDS]) {
        // Add the amount of jokers to the highest count
        let joker_count = std::mem::take(&mut counts[0]);
        Part1::sort_counts(counts);
        counts[0] += joker_count; 
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand<P: Part> {
    cards: [u8; HAND_SIZE],
    // So that the generic type parameter P does not stay unused
    part: PhantomData<P>,
}

impl<P: Part> Hand<P> {
    fn new(cards: [u8; HAND_SIZE]) -> Self {
        Hand {
            cards,
            part: PhantomData,
        }
    }

    fn counts(&self) -> [u8; CARDS] {
        let mut count = [0; CARDS];
        for c in self.cards {
            count[c as usize] += 1;
        }
        count
    }

    fn hand_type(&self) -> HandType {
        let mut counts = self.counts();
        P::sort_counts(&mut counts);
        if counts[0] == 5 {
            return HandType::FiveOfAKind;
        }
        if counts[0] == 4 {
            return HandType::FourOfAKind;
        }
        if counts[0] == 3 {
            if counts[1] == 2 {
                return HandType::FullHouse;
            }
            return HandType::ThreeOfAKind;
        }
        if counts[0] == 2 {
            if counts[1] == 2 {
                return HandType::TwoPair;
            }
            return HandType::OnePair;
        }
        HandType::HighCard
    }
}

impl<P: Part> Ord for Hand<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                for (a, b) in iter::zip(self.cards, other.cards) {
                    if a != b {
                        return a.cmp(&b);
                    }
                }
                Ordering::Equal
            },
            other => other,
        }
    }
}

impl<P: Part> PartialOrd for Hand<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: Part> FromStr for Hand<P> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       let mut cards_it = s.chars()
           .map(P::card_num);
       let mut cards = [0; HAND_SIZE];
       for card in cards.iter_mut() {
           *card = cards_it.next().ok_or("Not enough card characters".to_string())?;
       }
       if cards_it.next().is_some() {
           return Err("Too many card characters".to_string());
       }

       Ok(Hand::new(cards))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn print_winnings<P: Part>(mut game: Vec<(Hand<P>, u32)>) {
    game.sort_by(|g1, g2| g1.0.cmp(&g2.0));
    let winnings = game.iter()
        .enumerate()
        .map(|(i, (_hand, bid))| bid * (i as u32 + 1))
        .sum::<u32>();
    println!("{winnings}");
}

fn part1(input: String) {
    print_winnings(Part1::parse_input(input));
}

fn part2(input: String) {
    print_winnings(Part2::parse_input(input));
}

util::aoc_main!("day7.txt");
