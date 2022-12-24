use crate::color::Color;
use crate::player::Player;

use anyhow::{bail, Result};
use std::fmt::Display;

pub const GRID_WIDTH: usize = 5;
pub const GRID_HEIGHT: usize = 7;

#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Player(Player),
    Uncolored,
    Colored(Player, Color),
}

impl Grid {
    pub fn is_valid_move(
        &self,
        player: &Player,
        color: &Color,
        x: usize,
        y: usize,
    ) -> Result<bool> {
        match &self.cells[y][x] {
            Cell::Player(targeted_player) => {
                if player == targeted_player {
                    bail!("You can't place a color on your own invocator");
                }
                match self.find_snake_head(player, color) {
                    Some((snake_head_x, snake_head_y)) => {
                        if !Grid::grows_head(player, snake_head_x, snake_head_y, x, y) {
                            bail!("The cell you picked does not grow your existing snake");
                        }
                        Ok(true)
                    }
                    None => {
                        if (*player == Player::Top && y != 0)
                            || (*player == Player::Bottom && y != GRID_HEIGHT - 1)
                        {
                            bail!("You must invoke your snake on your invocation line");
                        }
                        Ok(false)
                    }
                }
            }
            Cell::Colored(_, targeted_color) => match self.find_snake_head(player, color) {
                Some((snake_head_x, snake_head_y)) => {
                    if !Grid::grows_head(player, snake_head_x, snake_head_y, x, y) {
                        bail!("The cell you picked does not grow your existing snake");
                    }
                    if !color.beats(targeted_color) {
                        bail!("This snake is not strong enough to beat the targeted snake");
                    }
                    Ok(false)
                }
                None => {
                    if (*player == Player::Top && y != 0)
                        || (*player == Player::Bottom && y != GRID_HEIGHT - 1)
                    {
                        bail!("You must invoke your snake on your invocation line");
                    }
                    Ok(false)
                }
            },
            Cell::Uncolored => match self.find_snake_head(player, color) {
                Some((snake_head_x, snake_head_y)) => {
                    if !Grid::grows_head(player, snake_head_x, snake_head_y, x, y) {
                        bail!("The cell you picked does not grow your existing snake");
                    }
                    Ok(false)
                }
                None => {
                    if (*player == Player::Top && y != 0)
                        || (*player == Player::Bottom && y != GRID_HEIGHT - 1)
                    {
                        bail!("You must invoke your snake on your invocation line");
                    }
                    Ok(false)
                }
            },
        }
    }

    pub fn play_move(&mut self, player: &Player, color: &Color, x: usize, y: usize) {
        if let Cell::Colored(targeted_player, targeted_color) = self.cells[y][x] {
            let dir = match targeted_player {
                Player::Bottom => -1,
                Player::Top => 1,
            };
            self.eat_snake_rec(&targeted_player, &targeted_color, x, y, dir);
        }
        self.cells[y][x] = Cell::Colored(*player, *color);
    }

    fn eat_snake_rec(&mut self, player: &Player, color: &Color, x: usize, y: usize, dir: i32) {
        self.cells[y][x] = Cell::Uncolored;

        if let Some((next_x, next_y)) = self.next_snake_part(player, color, x, y, dir) {
            self.eat_snake_rec(player, color, next_x, next_y, dir);
        }
    }

    fn find_snake_head(&self, player: &Player, color: &Color) -> Option<(usize, usize)> {
        let (y, dir) = match player {
            Player::Bottom => (GRID_HEIGHT - 1, -1),
            Player::Top => (0, 1),
        };
        if self.cells[y][0] == Cell::Colored(*player, *color) {
            return Some(self.find_snake_head_rec(player, color, 0, y, dir));
        }
        if self.cells[y][1] == Cell::Colored(*player, *color) {
            return Some(self.find_snake_head_rec(player, color, 1, y, dir));
        }
        if self.cells[y][3] == Cell::Colored(*player, *color) {
            return Some(self.find_snake_head_rec(player, color, 3, y, dir));
        }
        if self.cells[y][4] == Cell::Colored(*player, *color) {
            return Some(self.find_snake_head_rec(player, color, 4, y, dir));
        }
        None
    }

    fn find_snake_head_rec(
        &self,
        player: &Player,
        color: &Color,
        x: usize,
        y: usize,
        dir: i32,
    ) -> (usize, usize) {
        match self.next_snake_part(player, color, x, y, dir) {
            Some((next_x, next_y)) => self.find_snake_head_rec(player, color, next_x, next_y, dir),
            None => (x, y),
        }
    }

    fn next_snake_part(
        &self,
        player: &Player,
        color: &Color,
        x: usize,
        y: usize,
        dir: i32,
    ) -> Option<(usize, usize)> {
        let next_y = y as i32 + dir;
        // Bounds check
        if next_y < 0 || next_y >= GRID_HEIGHT as i32 {
            return None;
        }
        let next_y = next_y as usize;
        // Try to find next part of the body
        if x > 0 && self.cells[next_y][x - 1] == Cell::Colored(*player, *color) {
            return Some((x - 1, next_y));
        }
        if self.cells[next_y][x] == Cell::Colored(*player, *color) {
            return Some((x, next_y));
        }
        if x < GRID_WIDTH - 1 && self.cells[next_y][x + 1] == Cell::Colored(*player, *color) {
            return Some((x + 1, next_y));
        }
        None
    }

    fn grows_head(
        player: &Player,
        snake_head_x: usize,
        snake_head_y: usize,
        next_x: usize,
        next_y: usize,
    ) -> bool {
        let dir = match player {
            Player::Bottom => -1,
            Player::Top => 1,
        };
        let grow_y = next_y as i32 - snake_head_y as i32;
        let grow_x = next_x as i32 - snake_head_x as i32;
        grow_y == dir && grow_x.abs() <= 1
    }
}

impl Default for Grid {
    fn default() -> Self {
        let mut grid = Self {
            cells: vec![vec![Cell::Uncolored; GRID_WIDTH]; GRID_HEIGHT],
        };
        grid.cells[0][2] = Cell::Player(Player::Top);
        grid.cells[6][2] = Cell::Player(Player::Bottom);
        grid
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (y, row) in self.cells.iter().enumerate() {
            write!(f, "        {}", y + 1)?;
            for cell in row {
                match cell {
                    Cell::Uncolored => {
                        write!(f, "⬜")?;
                    }
                    Cell::Colored(player, color) => match (player, color) {
                        (Player::Top, Color::Red) => {
                            write!(f, "🟥")?;
                        }
                        (Player::Top, Color::Green) => {
                            write!(f, "🟩")?;
                        }
                        (Player::Top, Color::Yellow) => {
                            write!(f, "🟨")?;
                        }
                        (Player::Top, Color::Blue) => {
                            write!(f, "🟦")?;
                        }
                        (Player::Bottom, Color::Red) => {
                            write!(f, "🔴")?;
                        }
                        (Player::Bottom, Color::Green) => {
                            write!(f, "🟢")?;
                        }
                        (Player::Bottom, Color::Yellow) => {
                            write!(f, "🟡")?;
                        }
                        (Player::Bottom, Color::Blue) => {
                            write!(f, "🔵")?;
                        }
                    },
                    Cell::Player(player) => match player {
                        Player::Top => {
                            write!(f, "🧙")?;
                        }
                        Player::Bottom => {
                            write!(f, "🥷")?;
                        }
                    },
                }
            }
            writeln!(f)?;
        }
        write!(f, "          a b c d e")?;
        Ok(())
    }
}
