use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() {
    let contents = include_str!("../input");
    let mut parts = contents.split("\n\n");

    let deck_1 = read_deck(parts.next().expect("No player 1 deck"))
        .expect("Error reading player 1 deck");
    
    let deck_2 = read_deck(parts.next().expect("No player 2 deck"))
        .expect("Error reading player 2 deck");

    let score_1 = score(&combat(&deck_1, &deck_2).1);
    println!("Part 1: {}", score_1);

    let score_2 = score(&recursive_combat(&deck_1, &deck_2).1);
    println!("Part 2: {}", score_2);
}

fn read_deck(ls: &str) -> Option<Vec<u64>> {
    ls.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .skip(1)
        .map(|s| s.parse::<u64>().ok())
        .collect()
}

fn score(deck: &[u64]) -> u64 {
    deck.iter().enumerate().map(|(i, x)| x * (deck.len() - i) as u64).sum()
}

fn combat(deck_1: &[u64], deck_2: &[u64]) -> (bool, Vec<u64>) {
    let mut deck_1 = VecDeque::from_iter(deck_1.iter().copied());
    let mut deck_2 = VecDeque::from_iter(deck_2.iter().copied());
    while !deck_1.is_empty() && !deck_2.is_empty() {
        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();
        if card_1 < card_2 {
            deck_2.push_back(card_2);
            deck_2.push_back(card_1);
        } else {
            deck_1.push_back(card_1);
            deck_1.push_back(card_2);
        }
    }
    if deck_1.is_empty() {
        (true, deck_2.iter().copied().collect())
    } else {
        (false, deck_1.iter().copied().collect())
    }
}

fn recursive_combat(deck_1: &[u64], deck_2: &[u64]) -> (bool, Vec<u64>) {
    let mut previous_decks: Vec<(Vec<u64>, Vec<u64>)> = Vec::new();
    let mut deck_1 = VecDeque::from_iter(deck_1.iter().copied());
    let mut deck_2 = VecDeque::from_iter(deck_2.iter().copied());
    while !deck_1.is_empty() && !deck_2.is_empty() {
        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();
        let deck_1_v = deck_1.iter().copied().collect::<Vec<u64>>();
        let deck_2_v = deck_2.iter().copied().collect::<Vec<u64>>();
        if previous_decks.contains(&(deck_1_v.clone(), deck_2_v.clone())) {
            return (false, deck_1_v);
        }
        let p2_wins_round = if card_1 as usize <= deck_1.len() && card_2 as usize <= deck_2.len() {
            recursive_combat(&deck_1_v[..card_1 as usize], &deck_2_v[..card_2 as usize]).0
        } else {
            card_1 < card_2
        };
        if p2_wins_round {
            deck_2.push_back(card_2);
            deck_2.push_back(card_1);
        } else {
            deck_1.push_back(card_1);
            deck_1.push_back(card_2);
        }
        previous_decks.push((deck_1_v, deck_2_v));
    }
    if deck_1.is_empty() {
        (true, deck_2.iter().copied().collect())
    } else {
        (false, deck_1.iter().copied().collect())
    }
}
