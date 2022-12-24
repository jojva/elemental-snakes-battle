mod color;
mod grid;
mod player;

use color::Color;
use grid::{Grid, GRID_HEIGHT, GRID_WIDTH};
use player::Player;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut grid = Grid::default();
    let mut player = Player::Bottom;

    let mut err = String::new();

    loop {
        print!("{esc}c", esc = 27 as char);
        println!("🐍 Elemental Snakes Battle 🐍\n\n");
        println!("{}\n", grid);
        if !err.is_empty() {
            println!("\x1b[31;1m{}\x1b[39;0m", err);
        }
        err.clear();
        println!("It is {:?}'s turn.", player);
        print!("Place a color: ");
        io::stdout().flush().unwrap();

        let (color, x, y) = match parse_input() {
            Ok((color, x, y)) => (color, x, y),
            Err(e) => {
                err = e;
                continue;
            }
        };

        let win = match grid.is_valid_move(&player, &color, x, y) {
            Ok(win) => win,
            Err(e) => {
                err = e;
                continue;
            }
        };
        grid.play_move(&player, &color, x, y);

        if win {
            break;
        }

        player = player.next();
    }

    println!("\n{:?} wins the game! 🏆", player);

    Ok(())
}

fn parse_input() -> Result<(Color, usize, usize), String> {
    let stdin = io::stdin();
    let mut user_input = String::new();

    user_input.clear();
    stdin.read_line(&mut user_input).unwrap();
    let commands = user_input.trim().chars().collect::<Vec<_>>();
    if commands.len() != 3 {
        return Err(format!(
            "Expected 3 parameters, got {}: {:?}",
            commands.len(),
            commands
        ));
    }
    let color = Color::try_from(commands[0]);
    let color = match color {
        Ok(color) => color,
        Err(_) => {
            return Err(format!("Couldn't parse color '{}'", commands[0]));
        }
    };
    let x = (commands[1] as usize).wrapping_sub('a' as usize);
    if x >= GRID_WIDTH {
        return Err("Column must be 'a', 'b', 'c', 'd', or 'e'".to_string());
    }
    let y = char::to_digit(commands[2], 10);
    let y = match y {
        Some(y) if y == 0 || y as usize > GRID_HEIGHT => {
            return Err(format!("Row must be between 1 and {}", GRID_HEIGHT));
        }
        Some(y) => (y - 1) as usize,
        None => {
            return Err("Couldn't parse row".to_string());
        }
    };
    Ok((color, x, y))
}
