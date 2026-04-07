use crate::engine::prelude::*;

pub struct GameEngine {
    game_state: GameState,
    o: Box<dyn BotActionGenerator>,
    x: Box<dyn BotActionGenerator>,
}
impl GameEngine {
    pub fn new(o: &Box<dyn BotFactory>, x: &Box<dyn BotFactory>, debug_mode: bool) -> Self {
        Self {
            game_state: GameState::new(),
            o: o.new_bot(BotArgs::new(PlayerId::O, debug_mode)),
            x: x.new_bot(BotArgs::new(PlayerId::X, debug_mode)),
        }
    }
}
impl GameEngine {
    /// returns true if game not over
    pub fn go_to_next_frame(&mut self) -> NextFrameResult {
        let a_action = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(||self.o.generate_next_action(&self.game_state))
        );
        let b_action = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(||self.x.generate_next_action(&self.game_state))
        );

        match (a_action.is_err(), b_action.is_err()){
            (true, true) => NextFrameResult::Draw,
            (true, false) => NextFrameResult::Winner { player_who_won: PlayerId::X },
            (false, true) => NextFrameResult::Winner { player_who_won: PlayerId::X },
            (false, false) => {
                let Ok(a_action) = a_action else {unreachable!()};
                let Ok(b_action) = b_action else {unreachable!()};
                self.game_state.go_to_next_frame(
                    a_action,
                    b_action,
                )
            },
        }
    }

    pub fn print_current_game_state(&self){
        println!("{}", self.game_state)
    }
    pub fn run_game_get_result_print(&mut self) -> GameOver {
        self.print_current_game_state();
        loop{
            if let Some(out) = self.go_to_next_frame().game_over() {
                self.print_current_game_state();
                return out;
            }
            self.print_current_game_state();
        }
    }
    pub fn run_game_get_result(&mut self) -> GameOver {
        loop{
            if let Some(out) = self.go_to_next_frame().game_over() {
                return out;
            }
        }
    }
}

