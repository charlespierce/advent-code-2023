use aoc_runner_derive::aoc;
use std::cmp::Ordering;
use std::collections::HashMap;

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> usize {
    let mut hands: Vec<_> = input.lines().map(Hand::for_part_1).collect();
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum()
}

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> usize {
    let mut hands: Vec<_> = input.lines().map(Hand::for_part_2).collect();
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum()
}

struct Hand {
    category: Category,
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn for_part_1(value: &str) -> Self {
        let (cards_str, bid_str) = value.split_once(' ').unwrap();
        let cards = cards_str.chars().map(Card::for_part_1).collect();

        Self::new(cards, bid_str.parse().unwrap())
    }

    fn for_part_2(value: &str) -> Self {
        let (cards_str, bid_str) = value.split_once(' ').unwrap();
        let cards = cards_str.chars().map(Card::for_part_2).collect();

        Self::new(cards, bid_str.parse().unwrap())
    }

    fn new(cards: Vec<Card>, bid: usize) -> Self {
        let category = Category::for_cards(&cards);

        Self {
            category,
            cards,
            bid,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.category.cmp(&other.category) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    Trips,
    Boat,
    Quads,
    Quints,
}

impl Category {
    fn for_cards(cards: &[Card]) -> Category {
        let mut card_map = HashMap::new();
        for card in cards {
            if card != &Card::Joker {
                *card_map.entry(card).or_default() += 1;
            }
        }

        let mut counts: Vec<usize> = card_map.values().copied().collect();
        counts.sort();

        match counts.as_slice() {
            // All non-joker cards are the same, so this is 5 of a kind
            [5] | [4] | [3] | [2] | [1] | [] => Category::Quints,
            // A single non-joker card different from the rest, so this is 4 of a kind
            [1, 4] | [1, 3] | [1, 2] | [1, 1] => Category::Quads,
            // A complete full house or two pairs with a joker
            [2, 3] | [2, 2] => Category::Boat,
            // Two non-joker cards different from the rest, so this is 3 of a kind
            [1, 1, 3] | [1, 1, 2] | [1, 1, 1] => Category::Trips,
            // Two pair can only exist without a joker, as a joker would make it into trips
            [1, 2, 2] => Category::TwoPair,
            // One pair or four different cards plus a joker
            [1, 1, 1, 2] | [1, 1, 1, 1] => Category::OnePair,
            // High card can only exist without a joker
            [1, 1, 1, 1, 1] => Category::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn for_part_1(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }

    fn for_part_2(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }
}
