use std::{fs, u64};

#[derive(Debug)]
struct GameSet(u32, u32, u32);

#[derive(Debug)]
struct Game {
    id: u64,
    sets: Vec<GameSet>,
}

impl Game {
    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        for set in self.sets.iter() {
            if !set.is_possible(red, green, blue) {
                return false;
            }
        }
        return true;
    }

    fn get_min_set_cubes(&self) -> (u32, u32, u32) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        for ele in self.sets.iter() {
            r = r.max(ele.0);
            g = g.max(ele.1);
            b = b.max(ele.2);
        }
        return (r, g, b);
    }
}

impl GameSet {
    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        if red < self.0 || green < self.1 || blue < self.2 {
            return false;
        }
        return true;
    }
}

enum GameError {
    InvalidGameError,
}
impl TryFrom<&str> for Game {
    type Error = GameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vals: Vec<&str> = value.split(":").collect();
        if vals.len() < 2 {
            return Err(GameError::InvalidGameError);
        }
        let sets = vals[1];
        let game: u64 = vals[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .unwrap_or(0);

        let mut new_game = Game {
            id: game,
            sets: Vec::new(),
        };

        let sets: Vec<&str> = sets.split(";").collect();
        for s in sets {
            match s.trim().try_into() {
                Ok(val) => {
                    new_game.sets.push(val);
                }
                Err(_) => {
                    return Err(GameError::InvalidGameError);
                }
            };
        }

        Ok(new_game)
    }
}

enum GameSetError {
    InvalidGameSetError,
}
impl TryFrom<&str> for GameSet {
    type Error = GameSetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let colors = value.split(",");
        let mut set = GameSet(0, 0, 0);
        for color in colors {
            let splitted: Vec<&str> = color.trim().split_whitespace().collect();
            let val = splitted[0].parse::<u32>().unwrap_or(0);
            match splitted[1] {
                "red" => set.0 = val,
                "green" => set.1 = val,
                "blue" => set.2 = val,
                _ => {
                    return Err(GameSetError::InvalidGameSetError);
                }
            };
        }
        Ok(set)
    }
}

pub fn solve_part_one() {
    let path: &str = "./data/two.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    let mut sum = 0;
    for ln in contents.lines() {
        let game: Result<Game, GameError> = ln.try_into();
        match game {
            Ok(val) => {
                if val.is_possible(12, 13, 14) {
                    // println!("{:?}", val);
                    sum += val.id;
                }
            }
            Err(_) => {}
        }
    }
    println!("The sum of game IDs is: {}", sum);
}

pub fn solve_part_two() {
    let path: &str = "./data/two.txt";
    let err_msg = format!("{} not found!", path);
    let contents = fs::read_to_string(&path).expect(&err_msg);

    let mut sum = 0;
    for ln in contents.lines() {
        let game: Result<Game, GameError> = ln.try_into();
        match game {
            Ok(val) => {
                let (r, g, b) = val.get_min_set_cubes();
                // println!("{r} {g} {b}");
                sum += r*g*b;
            }
            Err(_) => {}
        }
    }
    println!("The sum of product of min set of cubes is: {}", sum);
}
