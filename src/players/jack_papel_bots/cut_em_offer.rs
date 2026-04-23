use std::rc::Rc;

use crate::{engine::prelude::*, players::jack_papel_bots::{JackBot, RelevantInformation, SkillEstimate, a_star_diagnostic, a_star_pathfinding, base_heuristic, find_farthest_point, find_farthest_point_in_general, get_neighbors, next_direction_from_path, pathfind, shortest_distance}};

/// This bot calculates the farthest reachable point from the other bot,
/// assumes they are trying to go there, and tries to cut them off by 
/// going to the point that is on the path to that point, is closest to us,
/// but isn't closer to the other bot than to us.
pub struct CutEmOffer {
    my_player_id: PlayerId
}

impl Bot for CutEmOffer {
    fn new(args: BotArgs) -> Self {
        Self {
            my_player_id: args.my_player()
        }
    }

    fn next_action(&mut self, game_state: &GameState) -> Direction {
        let grid = game_state.current_grid();
        let my_pos = grid.player_head_position(self.my_player_id);
        let other_pos = grid.player_head_position(self.my_player_id.other());

        let my_a_star = a_star_diagnostic(
            my_pos,
            grid.player_head_direction(self.my_player_id),
            other_pos,
            game_state.current_grid()
        );
        let other_a_star = a_star_diagnostic(
            other_pos,
            grid.player_head_direction(self.my_player_id.other()),
            my_pos,
            game_state.current_grid()
        );
        let relevant_info = RelevantInformation {
            game_state,
            other_bot_skill: &mut SkillEstimate::new(),
            my_a_star: &my_a_star,
            other_a_star: &other_a_star
        };

        // Try to cut THEM off
        other_a_star.to_farthest_point
            .iter()
            .enumerate()
            .filter_map(|(other_distance, pos)| {
                let my_distance = my_a_star.distances.get(pos).unwrap_or(&usize::MAX);

                if *my_distance < other_distance {
                    Some((pos, other_distance, my_distance))
                } else {
                    None
                }
            })
            .min_by_key(|&(_, other_distance, my_distance)| (my_distance, other_distance))
            .map(|(pos, _, _)| pos)
            .and_then(|next_pos| {
                if next_pos.borders_cell(grid, |cell| cell.is_players_head(self.my_player_id.other())) {
                    // If the next position is right next to the other player, avoid a draw.
                    // Find the point farthest from them---that way we don't walk into a corner or something.
                    if game_state.settings.debug_mode {
                        println!("cut_em_offer: Oh darn we messed up!");
                    }
                    self.move_to_most_open_space(&relevant_info)
                    // self.get_the_hell_out_of_dodge(game_state)
                } else {
                    // Otherwise, try to cut them off.
                    if game_state.settings.debug_mode {
                        println!("cut_em_offer: Cutting them off!");
                    }
                    next_direction_from_path(*next_pos, &my_a_star, game_state)
                }
            })
            .or_else(|| {
                // We can't cut them off.
                // If our spaces are connected, try to get to the farthest point.
                // Otherwise, try to fill the space as efficiently as possible.

                if my_a_star.to_goal.is_some() {
                    if game_state.settings.debug_mode {
                        println!("cut_em_offer: escaping");
                    }

                    self.move_to_most_open_space(&relevant_info)
                } else {
                    // Follow the right wall to fill the space.
                    if game_state.settings.debug_mode {
                        println!("cut_em_offer: filling");
                    }

                    self.fill_space(&relevant_info)
                }
            })
            .unwrap_or(Direction::NegativeX)
    }
}

impl JackBot for CutEmOffer {
    fn my_player_id(&self) -> PlayerId {
        self.my_player_id
    }
}