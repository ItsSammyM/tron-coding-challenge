use crate::engine::prelude::*;
pub struct GameEngine<A: Bot, B: Bot> {
    game_state: GameState,
    player_a_bot: A,
    player_b_bot: B,
}
impl<A: Bot, B: Bot> GameEngine<A, B> {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
            player_a_bot: A::new(PlayerId::new_o()),
            player_b_bot: B::new(PlayerId::new_x()),
        }
    }
    /// returns true if game not over
    fn go_to_next_frame(&mut self) -> bool {
        self.game_state.go_to_next_frame(
            self.player_a_bot.next_action(&self.game_state),
            self.player_b_bot.next_action(&self.game_state),
        )
    }
    fn print_current_game_state(&self) {
        println!("{}", self.game_state)
    }
    pub fn run_game(&mut self) {
        self.print_current_game_state();
        while self.go_to_next_frame() {
            self.print_current_game_state();
        }
        self.print_current_game_state();
    }
}
