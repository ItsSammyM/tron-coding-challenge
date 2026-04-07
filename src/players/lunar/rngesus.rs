use crate::engine::{direction, prelude::*};
use super::pcg::RandPCG;
use super::ALL_DIRECTIONS;

#[allow(dead_code)]
pub struct Rngesus<const BRAIN: u8> {
    id: PlayerId,
    pcg: RandPCG,
}

impl<const BRAIN: u8> Rngesus<BRAIN> {
    fn directions(
        &self,
        game_state: &GameState,
    ) -> Vec<Direction> {
        let mut grid = game_state.current_grid();
        let my_pos = grid.player_head_position(self.id);

        let mut directions = Vec::from(ALL_DIRECTIONS);

        // for direction in directions {
        //     let 
        // }
        

        todo!()
    }
}

impl<const BRAIN: u8> Bot for Rngesus<BRAIN> {
    fn new(my_player_id: PlayerId) -> Self {
        Self {id: my_player_id, pcg: Default::default() }
    }
    fn next_action(&mut self, game_state: &GameState) -> Direction {
        // for direction in self.not_instant_crash_directions(game_state) {
        //     println!("valid direction: {direction:?}")
        // }
        // let possibilities: Vec<Direction> = self.not_instant_crash_directions(game_state);
        // let rand = self.pcg.next_u32() as usize;
        // if possibilities.is_empty() {
        //     eprintln!("Rngesus about to crash: {}", self.id);
        //     ALL_DIRECTIONS[rand % 4]
        // } else {
        //     possibilities[rand % possibilities.len()]
        // }
        todo!()
    }
}