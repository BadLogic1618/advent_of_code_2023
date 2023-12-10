use std::{cmp::Ordering, collections::HashMap};

pub const INPUT: &str = include_str!("../../advent_of_code_2023_input/day_07/input.txt");


struct Hand<'a>{
    pub cards: &'a str,
    pub bid: usize,
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(order_hands(self.cards, other.cards, reg_hand_rank, false))
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        order_hands(self.cards, other.cards, reg_hand_rank, false)
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

impl Eq for Hand<'_> {}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAkind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

fn card_to_val(card: char, joker: bool) -> usize {
    match card {
        card if card.is_numeric() => card.to_digit(10).unwrap() as usize,
        'T' => 10,
        'J' => if joker {1} else {11},
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!()
    }
}

fn order_hands<F>(hand1: &str, hand2: &str, mut get_hand_rank: F, joker: bool) -> std::cmp::Ordering
where
    F: FnMut(&str) -> HandRank, 
{
    let ordering = get_hand_rank(hand1).cmp(&get_hand_rank(hand2));
    if ordering != Ordering::Equal { return ordering; }

    for (hand1, hand2) in hand1.chars().zip(hand2.chars()) {
        if hand1 == hand2 { continue; }
        return card_to_val(hand1, joker).cmp(&card_to_val(hand2, joker));
    }

    Ordering::Equal
}

fn reg_hand_rank(hand: &str) -> HandRank {
    let mut counts = HashMap::<char, usize>::new();

    for char in hand.chars() {
        counts.entry(char).and_modify(|v| *v += 1 ).or_insert(1);
    }

    let mut vals: Vec<&usize> = counts.values().collect();

    vals.sort_unstable();
    vals.reverse();

    return match vals[0] {
        5 => HandRank::FiveOfAKind,
        4 => HandRank::FourOfAKind,
        3 if *vals[1] == 2 => HandRank::FullHouse,
        3 => HandRank::ThreeOfAkind,
        2 if *vals[1] == 2 => HandRank::TwoPair,
        2 => HandRank::OnePair,
        1 => HandRank::HighCard,
        
        _ => unreachable!()
    }
}

pub fn part_one(input: &str) -> usize {
    let mut hands = input.lines()
        .map(|v| v.split_once(" ").unwrap())
        .map(|v| {
            Hand{cards: v.0, bid: v.1.parse().unwrap()}
        })
        .collect::<Vec<Hand>>();

    hands.sort_unstable();
    hands.iter().enumerate()
        .map(|(v, hand)| (1+v)*hand.bid)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut hands = input.lines()
    .map(|v| v.split_once(" ").unwrap())
    .map(|v| {
        Hand{cards: v.0, bid: v.1.parse().unwrap()}
    })
    .collect::<Vec<Hand>>();

    hands.sort_unstable_by(joker_sort);
    hands.iter().enumerate()
        .map(|(v, hand)| (1+v)*hand.bid)
        .sum()
}

fn joker_sort(lhs: &Hand, rhs: &Hand) -> Ordering {
    order_hands(lhs.cards, rhs.cards, joker_hands, true)
}

fn joker_hands(hand: &str) -> HandRank {
    let mut counts = HashMap::<char, usize>::new();

    let mut jcount = 0;

    for char in hand.chars() {
        if char =='J' {
            jcount += 1;
            continue;
        }
        counts.entry(char).and_modify(|v| *v += 1 ).or_insert(1);
    }

    if jcount == 5 { 
        return HandRank::FiveOfAKind;
    }

    let mut vals: Vec<&usize> = counts.values().collect();

    vals.sort_unstable();
    vals.reverse();

    return match vals[0]+jcount {
        5 => HandRank::FiveOfAKind,
        4 => HandRank::FourOfAKind,
        3 if *vals[1] == 2 => HandRank::FullHouse,
        3 => HandRank::ThreeOfAkind,
        2 if *vals[1] == 2 => HandRank::TwoPair,
        2 => HandRank::OnePair,
        1 => HandRank::HighCard,
        
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = include_str!("../../advent_of_code_2023_input/day_07/test.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 6440);
    }
    
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 5905);
    }
}