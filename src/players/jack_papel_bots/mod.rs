use crate::engine::{grid::Grid, prelude::{Direction, GameState, GridPosition, PlayerId}};

pub mod hallucinator;
pub mod freedom_eater;

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

fn base_heuristic(pos: GridPosition, goal: GridPosition) -> usize {
    // Manhattan distance heuristic
    let (x1, y1): (usize, usize) = pos.into();
    let (x2, y2): (usize, usize) = goal.into();
    (x1 as isize - x2 as isize).unsigned_abs() + (y1 as isize - y2 as isize).unsigned_abs()
}

fn get_neighbors(pos: GridPosition, grid: &Grid) -> Vec<GridPosition> {
    pos.neighbors()
        .filter(|neighbor| neighbor.is_empty(grid))
        .collect()
}

fn pathfind(start: GridPosition, fruit_location: GridPosition, grid: &Grid) -> Option<Vec<GridPosition>> {
    a_star_pathfinding(
        start,
        fruit_location,
        |pos| get_neighbors(pos, grid),
        base_heuristic
    )
}

// **** SOME OF THIS CODE COPIED FROM example_bot.rs!!! HOPE THIS ISNT CHEATING ****
trait JackBot {
    fn my_player_id(&self) -> PlayerId;

    fn not_instant_crash_directions(
        &self,
        game_state: &GameState,
    ) -> impl Iterator<Item = Direction> {
        let grid = game_state.current_grid();
        let my_pos = grid.player_head_position(self.my_player_id());

        Direction::all().filter(move |d| {
            my_pos
                .after_moved(*d)
                .filter(|p| p.is_empty(grid))
                .is_some()
        })
    }

    fn ideal_directions(&self, game_state: &GameState) -> impl Iterator<Item = Direction> {
        let grid = game_state.current_grid();
        let my_pos = grid.player_head_position(self.my_player_id());

        self.not_instant_crash_directions(game_state)
            .filter(move |d| {
                my_pos.after_moved(*d).is_some_and(|p| {
                    !p.borders_cell(grid, |cell| cell.is_players_head(self.my_player_id().other()))
                })
            })
    }

    fn direction_to(&self, game_state: &GameState, next_pos: GridPosition) -> Direction {
        let grid = game_state.current_grid();
        let head_pos = grid.player_head_position(self.my_player_id());
        
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