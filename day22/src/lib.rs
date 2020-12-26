use std::collections::{HashSet, VecDeque};

pub type Cards = VecDeque<usize>;

fn combat(player1: &mut Cards, player2: &mut Cards) {
    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }
}

fn score(cards: &Cards) -> usize {
    cards
        .iter()
        .rev()
        .zip(1..)
        .map(|(card, weight)| card * weight)
        .sum()
}

pub fn part1(cards1: &Cards, cards2: &Cards) -> usize {
    let mut player1: Cards = cards1.clone();
    let mut player2: Cards = cards2.clone();
    combat(&mut player1, &mut player2);
    let winner = if player1.is_empty() { player2 } else { player1 };
    score(&winner)
}

enum Winner {
    Player1,
    Player2,
}

fn recursive_combat(player1: &mut Cards, player2: &mut Cards) -> Winner {
    let mut hands1 = HashSet::new();
    let mut hands2 = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        if !hands1.insert(player1.clone()) && !hands2.insert(player2.clone()) {
            return Winner::Player1;
        }
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 <= player1.len() && card2 <= player2.len() {
            let mut subcards1 = player1.iter().take(card1).copied().collect();
            let mut subcards2 = player2.iter().take(card2).copied().collect();
            match recursive_combat(&mut subcards1, &mut subcards2) {
                Winner::Player1 => {
                    player1.push_back(card1);
                    player1.push_back(card2);
                }
                Winner::Player2 => {
                    player2.push_back(card2);
                    player2.push_back(card1);
                }
            }
        } else if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }
    if player1.is_empty() {
        Winner::Player2
    } else {
        Winner::Player1
    }
}

pub fn part2(cards1: &Cards, cards2: &Cards) -> usize {
    let mut player1: Cards = cards1.clone();
    let mut player2: Cards = cards2.clone();
    match recursive_combat(&mut player1, &mut player2) {
        Winner::Player1 => score(&player1),
        Winner::Player2 => score(&player2),
    }
}
