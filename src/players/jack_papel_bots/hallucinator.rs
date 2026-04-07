use crate::engine::prelude::*;

/// This bot thinks it's playing Snake. It hallucinates that there is a fruit on the board,
/// and does A* pathfinding to try to eat it. It doesn't care about the other player---only
/// where it can and cannot move to get to the fruit.
/// 
/// When the fruit is fully blocked off or is "eaten" (i.e. there is a head or tail occupying
/// its position), it generates a new location for the next fruit.
pub struct Hallucinator {
    my_player_id: PlayerId,
    fruit_location: GridPosition,
}

impl Bot for Hallucinator {
    fn new(my_player_id: PlayerId) -> Self {
        Hallucinator {
            my_player_id,
            fruit_location: Self::generate_initial_fruit_location(),
        }
    }

    fn next_action(&mut self, game_state: &GameState) -> Direction {
        let mut retries: usize = 0;

        while retries < 50 {
            if 
                let Some(path) = self.find_path_to_fruit(game_state) &&
                let Some(next_pos) = path.into_iter().nth(1)
            {
                return self.direction_to(game_state, next_pos);
            } else {
                // if there is no path to the fruit, try generating a new one again
                retries = retries.checked_add(1).unwrap_or(10);
                self.generate_new_fruit_location(game_state);
                continue;
            }
        }

        // We really couldn't generate a path to the fruit??
        // Damn. We cooked. Better luck next turn.
        println!("hallucinator: could not find path to fruit after {} retries. Giving up.", retries);
        
        self.ideal_directions(game_state).next()
            .or_else(|| self.not_instant_crash_directions(game_state).next())
            .unwrap_or(Direction::NegativeX)
    }
}

use rand::prelude::IndexedRandom;

impl Hallucinator {
    fn generate_new_fruit_location(&mut self, game_state: &GameState) {
        let grid = game_state.current_grid();
        let mut empty_positions = Vec::new();
        for pos in GridPosition::iter_positions() {
            if pos.is_empty(grid) {
                empty_positions.push(pos);
            }
        }

        if let Some(fruit_location) = empty_positions.choose(&mut rand::rng()) {
            self.fruit_location = *fruit_location;
        } else {
            // if there are no empty positions, just put the fruit somewhere invalid.
            // the Game is supposed to be over at this point anyway.
            self.fruit_location = GridPosition::new_from_usize(GRID_SIZE * GRID_SIZE).unwrap();
        }
    }

    fn generate_initial_fruit_location() -> GridPosition {
        // just put the fruit somewhere in the middle to start.
        GridPosition::new(GRID_SIZE / 2 + 5, GRID_SIZE / 2 + 5).unwrap()
    }

    fn find_path_to_fruit(&self, game_state: &GameState) -> Option<Vec<GridPosition>> {
        let grid = game_state.current_grid();
        let start = grid.player_head_position(self.my_player_id);

        a_star_pathfinding(start, self.fruit_location,
            |pos| {
                pos.neighbors()
                    .filter(|neighbor| neighbor.is_empty(grid))
                    .collect()
            }, |pos, goal| {
                // Manhattan distance heuristic
                let (x1, y1): (usize, usize) = pos.into();
                let (x2, y2): (usize, usize) = goal.into();
                (x1 as isize - x2 as isize).unsigned_abs() + (y1 as isize - y2 as isize).unsigned_abs()
            }
        )
    }

    fn direction_to(&self, game_state: &GameState, next_pos: GridPosition) -> Direction {
        let grid = game_state.current_grid();
        let head_pos = grid.player_head_position(self.my_player_id);
        
        let (head_x, head_y): (usize, usize) = head_pos.into();
        let (next_x, next_y): (usize, usize) = next_pos.into();

        match (
            next_x as isize - head_x as isize,
            next_y as isize - head_y as isize
        ) {
            (0, 1) => Direction::PositiveY,
            (0, -1) => Direction::NegativeY,
            (1, 0) => Direction::PositiveX,
            (-1, 0) => Direction::NegativeX,
            _ => {
                println!("Hallucinator: next position is not adjacent to head! This should never happen. Defaulting to moving right.");
                Direction::NegativeY
            },
        }
    }
}

#[derive(Eq)]
struct CellScore(usize, GridPosition);

impl PartialEq for CellScore {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd for CellScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CellScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

fn pathfind_to_fruit(start: GridPosition, fruit_location: GridPosition, grid: &Grid) -> Option<Vec<GridPosition>> {
    a_star_pathfinding(start, fruit_location,
        |pos| {
            pos.neighbors()
                .filter(|neighbor| neighbor.is_empty(grid))
                .collect()
        }, |pos, goal| {
            // Manhattan distance heuristic
            let (x1, y1): (usize, usize) = pos.into();
            let (x2, y2): (usize, usize) = goal.into();
            (x1 as isize - x2 as isize).unsigned_abs() + (y1 as isize - y2 as isize).unsigned_abs()
        }
    )
}

// This entire function was AI generated, but I fixed the errors it made manually.
fn a_star_pathfinding(
    start: GridPosition,
    goal: GridPosition,
    get_neighbors: impl Fn(GridPosition) -> Vec<GridPosition>,
    heuristic: impl Fn(GridPosition, GridPosition) -> usize,
) -> Option<Vec<GridPosition>> {
    use std::collections::{BinaryHeap, HashMap};

    // This is usually a max-heap, but CellScore has reverse ordering, so it's basically a min-heap.
    let mut open_set = BinaryHeap::new();

    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    g_score.insert(start, 0);
    f_score.insert(start, heuristic(start, goal));
    open_set.push(CellScore(f_score[&start], start));

    while let Some(CellScore(_, current)) = open_set.pop() {
        if current == goal {
            let mut path = vec![current];
            while let Some(&prev) = came_from.get(path.last().unwrap()) {
                path.push(prev);
            }
            path.reverse();
            return Some(path);
        }

        for neighbor in get_neighbors(current) {
            let tentative_g_score = g_score[&current] + 1;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + heuristic(neighbor, goal));
                if !open_set.iter().any(|&CellScore(_, pos)| pos == neighbor) {
                    open_set.push(CellScore(f_score[&neighbor], neighbor));
                }
            }
        }
    }

    None
}

// **** THIS CODE COPIED FROM example_bot.rs!!! HOPE THIS ISNT CHEATING ****
impl Hallucinator {
    fn not_instant_crash_directions(
        &self,
        game_state: &GameState,
    ) -> impl Iterator<Item = Direction> {
        let grid = game_state.current_grid();
        let my_pos = grid.player_head_position(self.my_player_id);

        Direction::all().filter(move |d| {
            my_pos
                .after_moved(*d)
                .filter(|p| p.is_empty(grid))
                .is_some()
        })
    }
    fn ideal_directions(&self, game_state: &GameState) -> impl Iterator<Item = Direction> {
        let grid = game_state.current_grid();
        let my_pos = grid.player_head_position(self.my_player_id);

        self.not_instant_crash_directions(game_state)
            .filter(move |d| {
                my_pos.after_moved(*d).is_some_and(|p| {
                    !p.borders_cell(grid, |cell| cell.is_players_head(self.my_player_id.other()))
                })
            })
    }
}
