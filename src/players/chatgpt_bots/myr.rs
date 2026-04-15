use crate::engine::prelude::*;
use std::time::{Duration, Instant};

const MAX_EVAL: i32 = 1_000_000;
const MIN_EVAL: i32 = -1_000_000;
const DRAW_EVAL: i32 = 0;
const CELLS: usize = GRID_SIZE * GRID_SIZE;
const BITBOARD_WORDS: usize = (CELLS + 63) / 64;

// ==========================================
// CACHE-OPTIMIZED DATA STRUCTURES
// ==========================================

#[derive(Clone, Copy)]
struct BitBoard([u64; BITBOARD_WORDS]);

impl BitBoard {
    fn new() -> Self {
        Self([0; BITBOARD_WORDS])
    }
    #[inline(always)]
    fn set(&mut self, i: usize) {
        self.0[i / 64] |= 1 << (i % 64);
    }
    #[inline(always)]
    fn clear(&mut self, i: usize) {
        self.0[i / 64] &= !(1 << (i % 64));
    }
    #[inline(always)]
    fn get(&self, i: usize) -> bool {
        (self.0[i / 64] & (1 << (i % 64))) != 0
    }
}

#[derive(Clone, Copy)]
struct DfsNode {
    pos: GridPosition,
    dir_idx: u8,
    parent: Option<GridPosition>,
}

struct SearchContext {
    q: [GridPosition; CELLS],
    visited_epoch: [u32; CELLS],
    current_epoch: u32,
    dist_m: [u16; CELLS],
    dist_o: [u16; CELLS],
    nodes_evaluated: u32, 
    
    dfs_stack: [DfsNode; CELLS],
    depth: [u16; CELLS],
    low: [u16; CELLS],
    art_board: BitBoard,
}

impl SearchContext {
    fn new() -> Self {
        Self {
            q: [GridPosition::new_from_usize(0).unwrap(); CELLS],
            visited_epoch: [0; CELLS],
            current_epoch: 0,
            dist_m: [u16::MAX; CELLS],
            dist_o: [u16::MAX; CELLS],
            nodes_evaluated: 0,
            
            dfs_stack: [DfsNode { pos: GridPosition::new_from_usize(0).unwrap(), dir_idx: 0, parent: None }; CELLS],
            depth: [0; CELLS],
            low: [0; CELLS],
            art_board: BitBoard::new(),
        }
    }

    #[inline(always)]
    fn next_epoch(&mut self) -> u32 {
        self.current_epoch += 1;
        self.current_epoch
    }
}

// ==========================================
// BOT LOGIC: MYR
// ==========================================

pub struct Myr {
    me: PlayerId,
}

impl Bot for Myr {
    fn new(args: BotArgs) -> Self {
        Self { me: args.my_player() }
    }

    fn next_action(&mut self, game_state: &GameState) -> Direction {
        let start_time = Instant::now();
        // Lowered to 75ms for a stricter safety margin
        let timeout = Duration::from_millis(75); 
        
        let grid = game_state.current_grid();
        let my_pos = self.me.get_head_pos(grid);
        let opp_pos = self.me.other().get_head_pos(grid);

        let mut board_visited = BitBoard::new();
        let mut ctx = SearchContext::new();

        // 1. Structural Checks at Root
        let separated = self.is_separated(grid, my_pos, opp_pos, &mut ctx);
        
        // 2. ROOT-NODE HEURISTIC: Find choke points ONCE per turn, not per leaf
        if !separated {
            self.find_articulation_points(grid, &board_visited, &mut ctx);
        }

        let mut best_move = Direction::all()
            .find(|&d| my_pos.after_moved(d).map_or(false, |p| p.is_empty(grid)))
            .unwrap_or(Direction::PositiveX);

        let mut abort = false;

        if separated {
            for current_max_depth in 1..400 {
                let mut best_val = -1;
                let mut current_best_move = None;

                for dir in Direction::all() {
                    if let Some(next_pos) = my_pos.after_moved(dir) {
                        if next_pos.is_empty(grid) {
                            board_visited.set(next_pos.i());
                            let val = self.survival_search(grid, next_pos, &mut board_visited, 1, current_max_depth, start_time, timeout, &mut abort, &mut ctx);
                            board_visited.clear(next_pos.i());

                            if abort { break; }
                            if val > best_val {
                                best_val = val;
                                current_best_move = Some(dir);
                            }
                        }
                    }
                }

                if abort { break; } 
                if let Some(m) = current_best_move { best_move = m; }
            }
        } else {
            let mut moves: [Direction; 4] = Direction::all_slice();
            let center = GridPosition::new(GRID_SIZE / 2, GRID_SIZE / 2).unwrap();
            
            moves.sort_by_key(|&d| {
                if let Some(next_pos) = my_pos.after_moved(d) {
                    if next_pos.is_empty(grid) { 
                        let c_dist = next_pos.manhattan_distance(&center) as i32;
                        let open_neighbors = Direction::all().filter(|&nd| next_pos.after_moved(nd).map_or(false, |p| p.is_empty(grid))).count() as i32;
                        return c_dist - (open_neighbors * 5); 
                    }
                }
                255
            });

            for current_max_depth in 1..35 {
                let mut alpha = MIN_EVAL;
                let beta = MAX_EVAL;
                let mut best_val = MIN_EVAL;
                let mut current_best_move = None;

                for &dir in &moves {
                    if let Some(next_pos) = my_pos.after_moved(dir) {
                        if next_pos.is_empty(grid) || next_pos == opp_pos {
                            board_visited.set(next_pos.i());
                            let val = self.minimax(grid, next_pos, opp_pos, &mut board_visited, current_max_depth, alpha, beta, false, start_time, timeout, &mut abort, &mut ctx);
                            board_visited.clear(next_pos.i());

                            if abort { break; }
                            if val > best_val {
                                best_val = val;
                                current_best_move = Some(dir);
                            }
                            alpha = alpha.max(best_val);
                        }
                    }
                }

                if abort { break; } 
                if let Some(m) = current_best_move {
                    best_move = m;
                    if best_val >= MAX_EVAL - 100 { break; } 
                }
            }
        }

        best_move
    }
}

impl Myr {
    #[allow(clippy::too_many_arguments)]
    fn minimax(
        &self, 
        grid: &Grid, 
        my_pos: GridPosition, 
        opp_pos: GridPosition, 
        board_visited: &mut BitBoard, 
        depth: u8, 
        mut alpha: i32, 
        mut beta: i32, 
        is_maximizing: bool,
        start_time: Instant,
        timeout: Duration,
        abort: &mut bool,
        ctx: &mut SearchContext
    ) -> i32 {
        ctx.nodes_evaluated += 1;
        // Strict clock check: Now happens every 256 nodes instead of 1024
        if ctx.nodes_evaluated & 255 == 0 && start_time.elapsed() >= timeout {
            *abort = true;
            return 0;
        }

        if my_pos == opp_pos { return DRAW_EVAL; }
        // Evaluation happens at depth 0, without expensive Tarjan calls
        if depth == 0 { return self.evaluate_voronoi(grid, board_visited, my_pos, opp_pos, ctx); }

        if is_maximizing {
            let mut max_eval = MIN_EVAL;
            let mut can_move = false;

            for dir in Direction::all() {
                if let Some(next_pos) = my_pos.after_moved(dir) {
                    if next_pos.is_empty(grid) && !board_visited.get(next_pos.i()) {
                        can_move = true;
                        board_visited.set(next_pos.i());
                        let eval = self.minimax(grid, next_pos, opp_pos, board_visited, depth - 1, alpha, beta, false, start_time, timeout, abort, ctx);
                        board_visited.clear(next_pos.i());
                        
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
            if !can_move { return MIN_EVAL + (40 - depth as i32); } 
            max_eval
        } else {
            let mut min_eval = MAX_EVAL;
            let mut can_move = false;

            for dir in Direction::all() {
                if let Some(next_opp) = opp_pos.after_moved(dir) {
                    if next_opp.is_empty(grid) && !board_visited.get(next_opp.i()) {
                        can_move = true;
                        board_visited.set(next_opp.i());
                        let eval = self.minimax(grid, my_pos, next_opp, board_visited, depth - 1, alpha, beta, true, start_time, timeout, abort, ctx);
                        board_visited.clear(next_opp.i());
                        
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
            if !can_move { return MAX_EVAL - (40 - depth as i32); } 
            min_eval
        }
    }

    fn evaluate_voronoi(&self, grid: &Grid, board_visited: &BitBoard, my_pos: GridPosition, opp_pos: GridPosition, ctx: &mut SearchContext) -> i32 {
        self.fast_bfs(grid, board_visited, my_pos, true, ctx);
        self.fast_bfs(grid, board_visited, opp_pos, false, ctx);

        let mut score = 0;
        for i in 0..CELLS {
            let d_m = ctx.dist_m[i];
            let d_o = ctx.dist_o[i];
            
            let mut cell_value = 100;
            
            // Instantly reads the choke points found at the start of the turn
            if ctx.art_board.get(i) {
                cell_value += 400; 
            }
            
            if d_m < d_o && d_m != u16::MAX {
                score += cell_value;
            } else if d_o < d_m && d_o != u16::MAX {
                score -= cell_value;
            }
        }

        let center = GridPosition::new(GRID_SIZE / 2, GRID_SIZE / 2).unwrap();
        score -= my_pos.manhattan_distance(&center) as i32; 

        score
    }

    fn find_articulation_points(&self, grid: &Grid, board_visited: &BitBoard, ctx: &mut SearchContext) {
        ctx.art_board = BitBoard::new();
        let epoch = ctx.next_epoch();
        let mut timer = 1;

        for i in 0..CELLS {
            let pos = GridPosition::new_from_usize(i).unwrap();
            if pos.is_empty(grid) && !board_visited.get(i) && ctx.visited_epoch[i] != epoch {
                ctx.visited_epoch[i] = epoch;
                ctx.depth[i] = timer;
                ctx.low[i] = timer;
                timer += 1;

                ctx.dfs_stack[0] = DfsNode { pos, dir_idx: 0, parent: None };
                let mut sp = 1;
                let mut root_children = 0;

                while sp > 0 {
                    let top_idx = sp - 1;
                    let u = ctx.dfs_stack[top_idx].pos;
                    let p = ctx.dfs_stack[top_idx].parent;
                    let dir_idx = ctx.dfs_stack[top_idx].dir_idx;

                    if dir_idx < 4 {
                        ctx.dfs_stack[top_idx].dir_idx += 1;
                        let dir = Direction::all_slice()[dir_idx as usize];
                        if let Some(v) = u.after_moved(dir) {
                            let vi = v.i();
                            if v.is_empty(grid) && !board_visited.get(vi) {
                                if ctx.visited_epoch[vi] != epoch {
                                    if p.is_none() { root_children += 1; }
                                    ctx.visited_epoch[vi] = epoch;
                                    ctx.depth[vi] = timer;
                                    ctx.low[vi] = timer;
                                    timer += 1;

                                    ctx.dfs_stack[sp] = DfsNode { pos: v, dir_idx: 0, parent: Some(u) };
                                    sp += 1;
                                } else if Some(v) != p {
                                    ctx.low[u.i()] = ctx.low[u.i()].min(ctx.depth[vi]);
                                }
                            }
                        }
                    } else {
                        sp -= 1;
                        if let Some(parent) = p {
                            ctx.low[parent.i()] = ctx.low[parent.i()].min(ctx.low[u.i()]);
                            if ctx.low[u.i()] >= ctx.depth[parent.i()] && p != Some(pos) {
                                ctx.art_board.set(parent.i());
                            }
                        }
                    }
                }

                if root_children > 1 {
                    ctx.art_board.set(i);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn survival_search(&self, grid: &Grid, pos: GridPosition, board_visited: &mut BitBoard, depth: u16, max_depth: u16, start_time: Instant, timeout: Duration, abort: &mut bool, ctx: &mut SearchContext) -> i32 {
        ctx.nodes_evaluated += 1;
        if ctx.nodes_evaluated & 255 == 0 && start_time.elapsed() >= timeout {
            *abort = true;
            return 0;
        }

        if depth == max_depth {
            return depth as i32 * 1000 + self.flood_fill_count(grid, board_visited, pos, ctx) as i32;
        }

        let mut best_val = -1;
        for dir in Direction::all() {
            if let Some(next_pos) = pos.after_moved(dir) {
                if next_pos.is_empty(grid) && !board_visited.get(next_pos.i()) {
                    board_visited.set(next_pos.i());
                    let val = self.survival_search(grid, next_pos, board_visited, depth + 1, max_depth, start_time, timeout, abort, ctx);
                    board_visited.clear(next_pos.i());
                    best_val = best_val.max(val);
                }
            }
        }
        
        if best_val == -1 { return depth as i32 * 1000; }
        best_val
    }

    fn fast_bfs(&self, grid: &Grid, board_visited: &BitBoard, start: GridPosition, is_me: bool, ctx: &mut SearchContext) {
        let epoch = ctx.next_epoch();
        let dists = if is_me { &mut ctx.dist_m } else { &mut ctx.dist_o };
        
        dists.fill(u16::MAX); 
        
        ctx.q[0] = start;
        let mut head = 0;
        let mut tail = 1;
        dists[start.i()] = 0;
        ctx.visited_epoch[start.i()] = epoch;

        while head < tail {
            let curr = ctx.q[head];
            head += 1;
            let d = dists[curr.i()] + 1;
            
            for n in curr.neighbors() {
                let ni = n.i();
                if n.is_empty(grid) && !board_visited.get(ni) && ctx.visited_epoch[ni] != epoch {
                    ctx.visited_epoch[ni] = epoch;
                    dists[ni] = d;
                    ctx.q[tail] = n;
                    tail += 1;
                }
            }
        }
    }

    fn flood_fill_count(&self, grid: &Grid, board_visited: &BitBoard, start: GridPosition, ctx: &mut SearchContext) -> u16 {
        let epoch = ctx.next_epoch();
        
        ctx.q[0] = start;
        let mut head = 0;
        let mut tail = 1;
        ctx.visited_epoch[start.i()] = epoch;
        
        let mut count = 0;

        while head < tail {
            let curr = ctx.q[head];
            head += 1;
            count += 1;
            
            for n in curr.neighbors() {
                let ni = n.i();
                if n.is_empty(grid) && !board_visited.get(ni) && ctx.visited_epoch[ni] != epoch {
                    ctx.visited_epoch[ni] = epoch;
                    ctx.q[tail] = n;
                    tail += 1;
                }
            }
        }
        count
    }

    fn is_separated(&self, grid: &Grid, my_pos: GridPosition, opp_pos: GridPosition, ctx: &mut SearchContext) -> bool {
        let epoch = ctx.next_epoch();
        
        ctx.q[0] = my_pos;
        let mut head = 0;
        let mut tail = 1;
        ctx.visited_epoch[my_pos.i()] = epoch;

        while head < tail {
            let curr = ctx.q[head];
            head += 1;
            
            for n in curr.neighbors() {
                if n == opp_pos { return false; } 
                let ni = n.i();
                if n.is_empty(grid) && ctx.visited_epoch[ni] != epoch {
                    ctx.visited_epoch[ni] = epoch;
                    ctx.q[tail] = n;
                    tail += 1;
                }
            }
        }
        true
    }
}