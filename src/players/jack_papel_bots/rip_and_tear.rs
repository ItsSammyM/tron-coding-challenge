use crate::{engine::prelude::*, players::jack_papel_bots::{JackBot, a_star_pathfinding, base_heuristic, direction_to, find_farthest_point, freedom_eater::FreedomEater, get_neighbors, pathfind, shortest_distance}};

// This bot calculates the farthest reachable point from the other bot,
// assumes they are trying to go there, and tries to cut them off by 
// going to the point that is on the path to that point, is closest to us,
// but isn't closer to the other bot than to us.
pub struct RipAndTear {
    my_player_id: PlayerId,
}

impl Bot for RipAndTear {
    fn new(args: BotArgs) -> Self {
        Self { my_player_id: args.my_player() }
    }

    fn next_action(&mut self, game_state: &GameState) -> Direction {
        let grid = game_state.current_grid();
        let my_pos = grid.player_head_position(self.my_player_id);
        let other_pos = grid.player_head_position(self.my_player_id.other());

        let farthest_point = find_farthest_point(other_pos, game_state).1;

        pathfind(other_pos, farthest_point, grid)
            .and_then(|path| {
                path.into_iter()
                    .enumerate()
                    .filter_map(|(other_distance, pos)| {
                        // Ohhh boy this is computationally intensive.
                        let my_distance = shortest_distance(my_pos, pos, grid)?;

                        if my_distance <= other_distance {
                            Some((pos, my_distance))
                        } else {
                            None
                        }
                    })
                    .min_by_key(|&(_, distance)| distance)
                    .map(|(pos, _)| pos)
            })
            .and_then(|next_pos| {
                if next_pos.borders_cell(grid, |cell| cell.is_players_head(self.my_player_id.other())) {
                    // If the next position is right next to the other player, avoid a draw.
                    // Find the point farthest from them---that way we don't walk into a corner or something.
                    let farthest_point = find_farthest_point(other_pos, game_state).1;

                    a_star_pathfinding(
                        my_pos,
                        farthest_point,
                        // Pathfind around their head
                        |pos| get_neighbors(pos, grid)
                            .iter()
                            .filter(|n| 
                                !n.borders_cell(grid, |cell|
                                    cell.is_players_head(self.my_player_id.other())
                                )
                            )
                            .cloned()
                            .collect(),
                        base_heuristic
                    )
                        .and_then(|path| path.into_iter().nth(1))
                        .map(|next_pos| self.direction_to(game_state, next_pos))
                        .or_else(|| {
                            pathfind(my_pos, farthest_point, grid)
                                .and_then(|path| path.into_iter().nth(1))
                                .map(|next_pos| self.direction_to(game_state, next_pos))
                        })
                } else {
                    // Otherwise, try to cut them off.
                    pathfind(my_pos, next_pos, grid)
                        .and_then(|path| path.into_iter().nth(1))
                        .map(|next_pos| self.direction_to(game_state, next_pos))
                }
            })
            .or_else(|| {
                // We can't cut them off.
                // If our spaces are connected, try to get to the farthest point.
                // Otherwise, try to fill the space as efficiently as possible.
                let escape_plan = {
                    let farthest_point = find_farthest_point(my_pos, game_state).1;

                    pathfind(my_pos, farthest_point, grid)
                        .and_then(|path| path.into_iter().nth(1))
                        .map(|next_pos| self.direction_to(game_state, next_pos))
                };

                if pathfind(my_pos, other_pos, grid).is_some() {
                    if game_state.settings.debug_mode {
                        println!("rip_and_tear: escaping");
                    }

                    if 
                        let Some(direction) = escape_plan &&
                        let Some(next_pos) = my_pos.after_moved(direction) &&
                        next_pos.borders_cell(grid, |cell| cell.is_players_head(self.my_player_id.other()))
                    {
                        // If the next position is right next to the other player, avoid a draw.
                        // Find the point farthest from them---that way we don't walk into a corner or something.
                        let farthest_point = find_farthest_point(other_pos, game_state).1;

                        a_star_pathfinding(
                            my_pos,
                            farthest_point,
                            // Pathfind around their head
                            |pos| get_neighbors(pos, grid)
                                .iter()
                                .filter(|n| 
                                    !n.borders_cell(grid, |cell|
                                        cell.is_players_head(self.my_player_id.other())
                                    )
                                )
                                .cloned()
                                .collect(),
                            base_heuristic
                        )
                            .and_then(|path| path.into_iter().nth(1))
                            .map(|next_pos| self.direction_to(game_state, next_pos))
                            .or_else(|| {
                                pathfind(my_pos, farthest_point, grid)
                                    .and_then(|path| path.into_iter().nth(1))
                                    .map(|next_pos| self.direction_to(game_state, next_pos))
                            })
                    } else {
                        // We're not about to crash into them, so just execute the original escape plan.
                        escape_plan
                    }
                } else {
                    // Follow the right wall to fill the space.
                    if game_state.settings.debug_mode {
                        println!("rip_and_tear: filling");
                    }
                    let direction = game_state.current_grid().player_head_direction(self.my_player_id);

                    let available_directions = self.ideal_non_hole_directions(game_state).collect::<Vec<_>>();

                    let can_go_right = available_directions.contains(&direction.right_of());
                    let can_go_forward = available_directions.contains(&direction);
                    let can_go_left = available_directions.contains(&direction.left_of());

                    match (can_go_left, can_go_forward, can_go_right) {
                        (false, false, false) => None,
                        (false, false, true) => Some(direction.right_of()),
                        (false, true, false) => Some(direction),
                        (false, true, true) => {
                            if 
                                my_pos.after_moved(direction)
                                    .and_then(|p| p.after_moved(direction.right_of()))
                                    .is_some_and(|p| p.is_empty(grid))
                            {
                                Some(direction.right_of())
                            } else {
                                escape_plan
                            }
                        },
                        (true, false, false) => Some(direction.left_of()),
                        (true, false, true) => escape_plan,
                        (true, true, false) => {
                            if 
                                my_pos.after_moved(direction)
                                    .and_then(|p| p.after_moved(direction.left_of()))
                                    .is_some_and(|p| p.is_empty(grid))
                            {
                                Some(direction)
                            } else {
                                escape_plan
                            }
                        },
                        (true, true, true) => {
                            let front_left_open = my_pos.after_moved(direction)
                                .and_then(|p| p.after_moved(direction.left_of()))
                                .is_some_and(|p| p.is_empty(grid));
                            let front_right_open = my_pos.after_moved(direction)
                                .and_then(|p| p.after_moved(direction.right_of()))
                                .is_some_and(|p| p.is_empty(grid));
                            
                            match (front_left_open, front_right_open) {
                                (false, false) => escape_plan,
                                (false, true) => {
                                    // Determine whether to turn right or go forward by looking one step further in each direction and seeing if it's open.
                                    // This should automatically exclude the other path since our head is blocking it.
                                    let forward = my_pos.after_moved(direction).unwrap();
                                    let left = my_pos.after_moved(direction.left_of()).unwrap();

                                    if find_farthest_point(forward, game_state).0 > find_farthest_point(left, game_state).0 {
                                        Some(direction)
                                    } else {
                                        Some(direction.left_of())
                                    }
                                },
                                (true, false) => {
                                    // See above---same logic but for the left side.
                                    let forward = my_pos.after_moved(direction).unwrap();
                                    let right = my_pos.after_moved(direction.right_of()).unwrap();

                                    if find_farthest_point(forward, game_state).0 > find_farthest_point(right, game_state).0 {
                                        Some(direction)
                                    } else {
                                        Some(direction.right_of())
                                    }
                                },
                                (true, true) => Some(direction.right_of())
                            }
                        },
                    }
                }
            })
            .unwrap_or(Direction::NegativeX)
    }
}

impl JackBot for RipAndTear {
    fn my_player_id(&self) -> PlayerId {
        self.my_player_id
    }
}