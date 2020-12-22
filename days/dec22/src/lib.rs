use std::{collections::VecDeque, error::Error};
use std::fs::read_to_string;

mod test;

extern crate utils;

#[derive(Debug, Clone)]
struct Player {
    name: String,
    deck: VecDeque<i32>
}

fn solve_puzzle(path: &str) -> Result<i32, Box<dyn Error + 'static>> {
    let file_content = read_to_string(path)?;
    let mut players: Vec<Player> = file_content.split("\n\n")
        .map(String::from)
        .map(parse_player)
        .collect();

    dbg!(&players);
    let winner = play(&mut players);

    // Calculate score
    let score: i32 = winner.deck.iter().enumerate()
        .map(|(i, v)| v * (winner.deck.len() - i) as i32)
        .sum();

    dbg!(&score);

    Ok(score)
}

fn play(players: &mut Vec<Player>) -> Player {
    let mut round = 1;
    loop {
        {
            let winningPlayer : Vec<&Player> = players
            .iter().map(|p|(p, p.deck.len()))
            .filter(|(_, ds)| *ds>0)
            .map(|(p, _)|p)
            .collect();


            if winningPlayer.iter().count() == 1 {
                // Winner winner chicken dinner
                return winningPlayer[0].clone();
            }
        }

        // Play a rounds
        println!("-- Round {} --", round);
        for p in players.iter() {
            println!("{}'s deck: {:?}", p.name, p.deck);
        }

        let mut round_cards: Vec<(&mut Player, i32)> = Vec::new();
        for p in players.iter_mut() {
            let front_card = p.deck.pop_front().unwrap();
            let p_name = p.name.clone();
            round_cards.push((p, front_card));
            println!("{} plays {}", p_name, front_card);
        }

        let mut winning_cards: Vec<i32> = round_cards.iter()
            .map(|(p, c)| *c).collect();
        winning_cards.sort();
        winning_cards.reverse();

        let winner = round_cards.iter_mut()
        .max_by(|(p1, c1), (p2, c2)| c1.cmp(c2))
        .unwrap();

        println!("{} wins the round!", winner.0.name);

        for wc in winning_cards {
            winner.0.deck.push_back(wc)
        }
        round += 1;
    }
    
}

fn parse_player(player_text: String) -> Player {
    let split: Vec<String> = player_text.split("\n").map(String::from).collect();
    let player_name = split[0].clone().replace(":","");

    let mut deck = VecDeque::new();

    for i in &split[1..] {
        let card_nr = i.parse::<i32>().unwrap_or(-1);
        deck.push_back(card_nr);
    }

    Player {
        name: player_name,
        deck: deck
    }
}