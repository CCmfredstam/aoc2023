use std::fs::read_to_string;

use regex::Regex;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

struct GameInfo {
    id: u32,
    rounds: Vec<String>,
}

fn main() {
    // Read todays input
    let data = read_to_string("input/day2.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    // Part 1
    let game_id_pattern = r"Game (?<id>\d+)";
    let re_game_id = Regex::new(game_id_pattern).expect("re_game_id: Invalid regex pattern...");

    let mut games: Vec<GameInfo> = vec![];

    // For each line (game), parse the game ID and all rounds (cubes drawn)
    for line in &lines {

        let parts: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
        let Some(game_id) = re_game_id.captures(&parts[0]) else { todo!() };
        let game_id = game_id["id"].parse::<u32>().unwrap();
        let rounds: Vec<String> = parts[1].split(';').map(|s| s.trim().to_string()).collect();
        games.push(GameInfo { id: game_id, rounds });
    }

    // For each round in each game, check if any cube color is more then allowed max.
    let mut valid_game_ids: Vec<u32> = vec![];
    
    let cube_pattern = r"(?<cube_num>\d+) (?<cube_col>red|green|blue)";
    let re_cube = Regex::new(cube_pattern).expect("re_cube: Invalid regex pattern...");

    for game in games {
        let mut valid_game = true;
        for round in game.rounds {
            for cube in round.split(',').map(|s| s.trim().to_string()) {
                let Some(c) = re_cube.captures(&cube) else { todo!() };
                let cube_num = c["cube_num"].parse::<u32>().unwrap();
                let cube_color = &c["cube_col"];
                match cube_color {
                    "red" => {
                        if cube_num > MAX_RED_CUBES {
                            valid_game = false;
                        }
                    },
                    "green" => {
                        if cube_num > MAX_GREEN_CUBES {
                            valid_game = false;
                        }
                    },
                    "blue" => {
                        if cube_num > MAX_BLUE_CUBES {
                            valid_game = false;
                        }
                    },
                    _ => todo!()
                }
            }
        }

        if valid_game {
            valid_game_ids.push(game.id);
        }
    }

    // Sum all IDs and output
    let game_id_sum: u32 = valid_game_ids.iter().sum();
    println!("Part 1 answer: {}", game_id_sum)

}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_placeholder() {
    }
}