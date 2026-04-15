use crate::engine::prelude::*;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const MAX_EVAL: i32 = 1_000_000;
const MIN_EVAL: i32 = -1_000_000;
const DRAW_EVAL: i32 = 0; 

pub struct ApexBot {
    me: PlayerId,
}

impl Bot for ApexBot {
    fn new(args: BotArgs) -> Self {
        Self { me: args.my_player() }
    }

    fn next_action(&mut self, game_state: &GameState) -> Direction {
        let start_time = Instant::now();
        let timeout = Duration::from_millis(85); 
        
        let grid = game_state.current_grid();
        let my_pos = self.me.get_head_pos(grid);
        let opp_pos = self.me.other().get_head_pos(grid);

        let mut visited = [false; GRID_SIZE * GRID_SIZE];
        
        // BUG FIX: Now correctly identifies if combat is still possible
        let separated = self.is_separated(grid, my_pos, opp_pos);

        // Safety Default: First open square, defaulting to Right
        let mut best_move = Direction::all()
            .find(|&d| my_pos.after_moved(d).map_or(false, |p| p.is_empty(grid)))
            .unwrap_or(Direction::PositiveX);

        if separated {
            // ==========================================
            // ENDGAME: SURVIVAL MODE 
            // ==========================================
            let mut current_max_depth = 1;
            
            while start_time.elapsed() < timeout && current_max_depth < 400 {
                let mut best_val = -1;
                let mut current_best_move = None;

                for dir in Direction::all() {
                    if let Some(next_pos) = my_pos.after_moved(dir) {
                        if next_pos.is_empty(grid) {
                            visited[next_pos.i()] = true;
                            let val = self.survival_search(grid, next_pos, &mut visited, 1, current_max_depth);
                            visited[next_pos.i()] = false;

                            if val > best_val {
                                best_val = val;
                                current_best_move = Some(dir);
                            }
                        }
                    }
                }

                if let Some(m) = current_best_move { best_move = m; }
                current_max_depth += 1;
            }
        } else {
            // ==========================================
            // MIDGAME: COMBAT MODE 
            // ==========================================
            let mut current_max_depth = 1;
            
            let mut moves: Vec<Direction> = Direction::all().collect();
            let center_x = (GRID_SIZE / 2) as i32;
            let center_y = (GRID_SIZE / 2) as i32;
            
            moves.sort_by_key(|&d| {
                if let Some(next_pos) = my_pos.after_moved(d) {
                    if next_pos.is_empty(grid) {
                        return (center_x - next_pos.x() as i32).abs() + (center_y - next_pos.y() as i32).abs();
                    }
                }
                1000 
            });

            while start_time.elapsed() < timeout && current_max_depth < 20 {
                let mut alpha = MIN_EVAL;
                let beta = MAX_EVAL;
                let mut best_val = MIN_EVAL;
                let mut current_best_move = None;

                for &dir in &moves {
                    if let Some(next_pos) = my_pos.after_moved(dir) {
                        if next_pos.is_empty(grid) || next_pos == opp_pos {
                            visited[next_pos.i()] = true;
                            let val = self.minimax(grid, next_pos, opp_pos, &mut visited, current_max_depth, alpha, beta, false);
                            visited[next_pos.i()] = false;

                            if val > best_val {
                                best_val = val;
                                current_best_move = Some(dir);
                            }
                            alpha = alpha.max(best_val);
                        }
                    }
                }

                if let Some(m) = current_best_move {
                    best_move = m;
                    if best_val >= MAX_EVAL - 100 { break; } 
                }
                
                current_max_depth += 1;
            }
        }

        best_move
    }
}

impl ApexBot {
    fn minimax(
        &self, 
        grid: &Grid, 
        my_pos: GridPosition, 
        opp_pos: GridPosition, 
        visited: &mut [bool; GRID_SIZE * GRID_SIZE], 
        depth: u8, 
        mut alpha: i32, 
        mut beta: i32, 
        is_maximizing: bool
    ) -> i32 {
        if my_pos == opp_pos { return DRAW_EVAL; }

        if depth == 0 {
            return self.evaluate_voronoi(grid, visited, my_pos, opp_pos);
        }

        if is_maximizing {
            let mut max_eval = MIN_EVAL;
            let mut can_move = false;

            for dir in Direction::all() {
                if let Some(next_pos) = my_pos.after_moved(dir) {
                    if next_pos.is_empty(grid) && !visited[next_pos.i()] {
                        can_move = true;
                        visited[next_pos.i()] = true;
                        let eval = self.minimax(grid, next_pos, opp_pos, visited, depth - 1, alpha, beta, false);
                        visited[next_pos.i()] = false;
                        
                        max_eval = max_eval.max(eval);
                        alpha = alpha.max(eval);
                        if beta <= alpha { break; }
                    } else if next_pos == opp_pos {
                        can_move = true;
                        max_eval = max_eval.max(DRAW_EVAL);
                        alpha = alpha.max(DRAW_EVAL);
                        if beta <= alpha { break; }
                    }
                }
            }
            if !can_move { return MIN_EVAL + (20 - depth as i32); } 
            max_eval
        } else {
            let mut min_eval = MAX_EVAL;
            let mut can_move = false;

            for dir in Direction::all() {
                if let Some(next_opp) = opp_pos.after_moved(dir) {
                    if next_opp.is_empty(grid) && !visited[next_opp.i()] {
                        can_move = true;
                        visited[next_opp.i()] = true;
                        let eval = self.minimax(grid, my_pos, next_opp, visited, depth - 1, alpha, beta, true);
                        visited[next_opp.i()] = false;
                        
                        min_eval = min_eval.min(eval);
                        beta = beta.min(eval);
                        if beta <= alpha { break; }
                    } else if next_opp == my_pos {
                        can_move = true;
                        min_eval = min_eval.min(DRAW_EVAL);
                        beta = beta.min(DRAW_EVAL);
                        if beta <= alpha { break; }
                    }
                }
            }
            if !can_move { return MAX_EVAL - (20 - depth as i32); } 
            min_eval
        }
    }

    fn evaluate_voronoi(&self, grid: &Grid, visited: &[bool; GRID_SIZE * GRID_SIZE], my_pos: GridPosition, opp_pos: GridPosition) -> i32 {
        let mut dist_m = [u16::MAX; GRID_SIZE * GRID_SIZE];
        let mut dist_o = [u16::MAX; GRID_SIZE * GRID_SIZE];

        self.fast_bfs(grid, visited, my_pos, &mut dist_m);
        self.fast_bfs(grid, visited, opp_pos, &mut dist_o);

        let mut my_territory = 0;
        let mut opp_territory = 0;

        for i in 0..(GRID_SIZE * GRID_SIZE) {
            if dist_m[i] < dist_o[i] && dist_m[i] != u16::MAX {
                my_territory += 1;
            } else if dist_o[i] < dist_m[i] && dist_o[i] != u16::MAX {
                opp_territory += 1;
            }
        }
        
        let mut score = (my_territory - opp_territory) * 100;

        let center = GridPosition::new(GRID_SIZE / 2, GRID_SIZE / 2).unwrap();
        let dist_to_center = my_pos.manhattan_distance(&center) as i32;
        score -= dist_to_center * 2; 

        let mobility = Direction::all().filter(|&d| {
            my_pos.after_moved(d).map_or(false, |p| p.is_empty(grid) && !visited[p.i()])
        }).count() as i32;
        score += mobility * 5;

        score
    }

    fn survival_search(&self, grid: &Grid, pos: GridPosition, visited: &mut [bool; GRID_SIZE * GRID_SIZE], depth: u16, max_depth: u16) -> i32 {
        if depth == max_depth {
            return depth as i32 * 1000 + self.flood_fill_count(grid, visited, pos) as i32;
        }

        let mut best_val = -1;
        for dir in Direction::all() {
            if let Some(next_pos) = pos.after_moved(dir) {
                if next_pos.is_empty(grid) && !visited[next_pos.i()] {
                    visited[next_pos.i()] = true;
                    let val = self.survival_search(grid, next_pos, visited, depth + 1, max_depth);
                    visited[next_pos.i()] = false;
                    best_val = best_val.max(val);
                }
            }
        }
        
        if best_val == -1 { return depth as i32 * 1000; }
        best_val
    }

    fn fast_bfs(&self, grid: &Grid, visited: &[bool; GRID_SIZE * GRID_SIZE], start: GridPosition, dists: &mut [u16; GRID_SIZE * GRID_SIZE]) {
        let mut q = VecDeque::with_capacity(150);
        dists[start.i()] = 0;
        q.push_back(start);

        while let Some(curr) = q.pop_front() {
            let d = dists[curr.i()] + 1;
            for n in curr.neighbors() {
                if n.is_empty(grid) && !visited[n.i()] && dists[n.i()] == u16::MAX {
                    dists[n.i()] = d;
                    q.push_back(n);
                }
            }
        }
    }

    fn flood_fill_count(&self, grid: &Grid, visited: &[bool; GRID_SIZE * GRID_SIZE], start: GridPosition) -> u16 {
        let mut q = VecDeque::with_capacity(150);
        let mut local_visited = *visited; 
        
        let mut count = 0;
        q.push_back(start);
        local_visited[start.i()] = true;

        while let Some(curr) = q.pop_front() {
            count += 1;
            for n in curr.neighbors() {
                if n.is_empty(grid) && !local_visited[n.i()] {
                    local_visited[n.i()] = true;
                    q.push_back(n);
                }
            }
        }
        count
    }

    // THE FIX:
    fn is_separated(&self, grid: &Grid, my_pos: GridPosition, opp_pos: GridPosition) -> bool {
        let mut q = VecDeque::with_capacity(150);
        let mut visited = [false; GRID_SIZE * GRID_SIZE];
        
        q.push_back(my_pos);
        visited[my_pos.i()] = true;

        while let Some(curr) = q.pop_front() {
            for n in curr.neighbors() {
                // Now properly checks if the neighbor IS the opponent's head
                if n == opp_pos { return false; } 
                if n.is_empty(grid) && !visited[n.i()] {
                    visited[n.i()] = true;
                    q.push_back(n);
                }
            }
        }
        true
    }
}