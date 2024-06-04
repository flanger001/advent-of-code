// 12 red cubes, 13 green cubes, and 14 blue cubes
use std::cmp;

#[derive(Debug, PartialEq)]
pub struct GameStat {
    id: usize,
    min_red: usize,
    min_green: usize,
    min_blue: usize,
}

impl Default for GameStat {
    fn default() -> Self {
        GameStat {
            id: 0,
            min_red: 0,
            min_green: 0,
            min_blue: 0,
        }
    }
}

pub fn parse_game(game: &str) -> GameStat {
    let mut game_stat = GameStat::default();
    let (game_id, results) = game.split_at(game.find(":").expect("not a valid game line"));
    let (_, id) = game_id.split_at(5);
    game_stat.id = id.parse().unwrap();
    let results = results.strip_prefix(": ").unwrap();

    for round in results.split(";") {
        for die in round.split(", ") {
            let parsed_die: Vec<&str> = die.trim().split_whitespace().collect();
            let new_die = parsed_die[0].parse::<usize>().unwrap();
            match parsed_die[1] {
                "red" => game_stat.min_red = cmp::max(new_die, game_stat.min_red),
                "green" => game_stat.min_green = cmp::max(new_die, game_stat.min_green),
                "blue" => game_stat.min_blue = cmp::max(new_die, game_stat.min_blue),
                _ => (),
            }
        }
    }
    game_stat
}

pub fn build_stats(input: &str) -> Vec<GameStat> {
    let mut stats: Vec<GameStat> = vec![];
    for line in input.split("\n") {
        if !line.trim().is_empty() {
            stats.push(parse_game(line.trim()))
        }
    }
    stats
}

pub fn constrain_games(
    stats: &Vec<GameStat>,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
) -> Vec<&GameStat> {
    stats
        .iter()
        .filter(|game| {
            game.min_red <= max_red && game.min_green <= max_green && game.min_blue <= max_blue
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_game() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game_stat = GameStat {
            id: 1,
            min_red: 4,
            min_green: 2,
            min_blue: 6,
        };
        assert_eq!(parse_game(game), game_stat);
        let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game_stat = GameStat {
            id: 2,
            min_red: 1,
            min_green: 3,
            min_blue: 4,
        };

        assert_eq!(parse_game(game), game_stat);
        let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game_stat = GameStat {
            id: 3,
            min_red: 20,
            min_green: 13,
            min_blue: 6,
        };

        assert_eq!(parse_game(game), game_stat);
        let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let game_stat = GameStat {
            id: 4,
            min_red: 14,
            min_green: 3,
            min_blue: 15,
        };

        assert_eq!(parse_game(game), game_stat);
        let game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let game_stat = GameStat {
            id: 5,
            min_red: 6,
            min_green: 3,
            min_blue: 2,
        };

        assert_eq!(parse_game(game), game_stat);
    }

    #[test]
    fn test_build_stats() {
        let input = r"Game 1: 3 green, 1 blue, 3 red; 3 blue, 1 green, 3 red; 2 red, 12 green, 7 blue; 1 red, 4 blue, 5 green; 7 green, 2 blue, 2 red\n\
            Game 2: 1 green, 19 blue, 1 red; 8 blue, 4 red; 3 red, 6 blue; 1 green, 1 red, 12 blue\n\
            Game 3: 3 green, 1 blue, 9 red; 1 blue, 2 green, 8 red; 1 blue, 2 red\n\
            Game 4: 6 green, 2 red; 2 red, 16 green; 3 red, 1 blue";
        let stats = build_stats(input);
        let expected = vec![
            GameStat {
                id: 1,
                min_red: 3,
                min_green: 12,
                min_blue: 7,
            },
            GameStat {
                id: 2,
                min_red: 4,
                min_green: 1,
                min_blue: 19,
            },
            GameStat {
                id: 3,
                min_red: 9,
                min_green: 3,
                min_blue: 1,
            },
            GameStat {
                id: 4,
                min_red: 3,
                min_green: 16,
                min_blue: 1,
            },
        ];
        assert_eq!(stats, expected)
    }

    #[test]
    fn test_constrain_games() {
        let input = r"Game 1: 3 green, 1 blue, 3 red; 3 blue, 1 green, 3 red; 2 red, 12 green, 7 blue; 1 red, 4 blue, 5 green; 7 green, 2 blue, 2 red\n\
            Game 2: 1 green, 19 blue, 1 red; 8 blue, 4 red; 3 red, 6 blue; 1 green, 1 red, 12 blue";
        let stats = build_stats(input);
        let constrained = constrain_games(&stats, 3, 12, 19);
        assert_eq!(
            constrained,
            vec![&GameStat {
                id: 1,
                min_red: 3,
                min_green: 12,
                min_blue: 7,
            }],
        )
    }

    #[test]
    fn test_aoc02_01() {
        // 12 red cubes, 13 green cubes, and 14 blue cubes
        let input = fs::read_to_string("./src/input.txt").unwrap();
        let stats = build_stats(&input);
        let constrained = constrain_games(&stats, 12, 13, 14);
        let sum: usize = constrained.iter().fold(0, |acc, gs| acc + gs.id);
        assert_eq!(sum, 1853);
    }

    #[test]
    fn test_aoc02_02() {
        // 12 red cubes, 13 green cubes, and 14 blue cubes
        let input = fs::read_to_string("./src/input.txt").unwrap();
        let stats = build_stats(&input);
        let powers: usize = stats
            .iter()
            .fold(0, |acc, gs| acc + (gs.min_red * gs.min_green * gs.min_blue));
        assert_eq!(powers, 72706);
    }
}
