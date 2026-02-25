use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

pub fn read_games(
    filepath: &str,
    games_to_read: usize,
) -> Result<Vec<(Vec<String>, String)>, Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut games = Vec::new();
    let mut curr_game = String::new();

    let mut dead_space_cnt: u8 = 0;
    let mut dead_space = false;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if dead_space {
                continue;
            } else {
                dead_space = true;
                dead_space_cnt += 1;
            }
        } else {
            dead_space = false;
        }
        if dead_space_cnt == 2 {
            games.push(get_move_list(curr_game.clone().trim()));
            curr_game.clear();

            dead_space_cnt = 0;
            if games_to_read != 0 && games.len() == games_to_read {
                break;
            }
            continue;
        }
        if dead_space_cnt == 1 {
            curr_game.push_str(&line.trim());
            curr_game.push(' ');
        }
    }

    if games_to_read == 0 || games.len() < games_to_read {
        if !curr_game.trim().is_empty() {
            games.push(get_move_list(curr_game.trim()))
        }
    }

    Ok(games)
}

pub fn get_move_list(game_str: &str) -> (Vec<String>, String) {
    let mut moves = Vec::new();

    let segments: Vec<&str> = game_str.split('.').collect();
    let mut result = String::new();

    for (i, segment) in segments.iter().skip(1).enumerate() {
        let splt: Vec<&str> = segment.split_whitespace().collect();
        for mv in &splt[..splt.len() - 1] {
            moves.push(mv.to_string());
            if i == segments.len() - 2 {
                result = splt[splt.len() - 1].to_owned();
            }
        }
    }

    return (moves, result);
}
