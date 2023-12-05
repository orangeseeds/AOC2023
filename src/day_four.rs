use std::{collections::HashMap, fs, u16, usize};

#[derive(Debug)]
struct Card {
    id: usize,
    winning_hand: Vec<u32>,
    my_hand: Vec<u32>,
    copies: u32,
}

impl Card {
    fn from_line(content: &str) -> Result<Card, &str> {
        let (card, hands) = match content.split_once(":") {
            Some((c, h)) => (c, h),
            None => {
                return Err("failed to parse the line!");
            }
        };

        let (winning, my) = match hands.split_once("|") {
            Some((w, m)) => (w, m),
            None => {
                return Err("failed to parse the line (hands)!");
            }
        };

        let winning: Vec<u32> = winning
            .split_whitespace()
            .map(|val| {
                return val.parse().unwrap_or_default();
            })
            .collect();
        let my: Vec<u32> = my
            .split_whitespace()
            .map(|val| {
                return val.parse().unwrap_or_default();
            })
            .collect();

        Ok(Card {
            id: card.split_whitespace().collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap_or_default(),
            winning_hand: winning,
            my_hand: my,
            copies: 1,
        })
    }

    fn common_nums(&self) -> u16 {
        let mut count: u16 = 0;
        for w in &self.winning_hand {
            for m in &self.my_hand {
                if w == m {
                    count += 1;
                }
            }
        }
        return count;
    }
}

pub fn solve_part_one() {
    let path: &str = "./data/four.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    let mut sum = 0;
    for ln in contents.lines() {
        match Card::from_line(ln) {
            Ok(c) => {
                let val = c.common_nums();
                // println!("{val}");
                if val > 0 {
                    sum += 2_u32.pow(u32::from(val) - 1);
                }
            }
            Err(_) => {}
        };
    }
    println!("Day four sum is: {}", sum);
}

pub fn solve_part_two() {
    let path: &str = "./data/four.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);
    let card_count = contents.lines().count();
    let mut cards = HashMap::new();

    for i in 1..=214 {
        cards.insert(i, 1);
    }

    for ln in contents.lines() {
        match Card::from_line(ln) {
            Ok(c) => {
                let i = c.id;
                let val = c.common_nums();
                for j in (i + 1)..(i + 1 + usize::from(val)) {
                    if j > card_count {
                        continue;
                    }
                    if let Some(copies) = cards.get(&j) {
                        cards.insert(j, copies + cards[&i]);
                    }
                }
            }
            Err(_) => {}
        };
    }
    // println!("{:?}", cards);
    let mut sum = 0;

    for (_, v) in cards {
        sum += v;
    }

    println!("Day four part_two sum is: {}", sum);
}
