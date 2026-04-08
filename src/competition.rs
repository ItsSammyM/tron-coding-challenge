use std::fmt::Display;

use crate::engine::prelude::*;

pub struct Competition;

impl Competition{
    pub fn run_and_print(
        mut players: Vec<CompetitionPlayer>
    ) {
        
        for i in 0..players.len(){
            for j in 0..players.len() {
                let Some([a, b]) = players.get_disjoint_mut([i, j]).ok() else {continue};
                Self::run_competition_round(a, b);
            }
        }

        players.sort_by(|a,b|b.points().total_cmp(&a.points()));
        for player in players {
            println!("{}", player);
        }
    }

    fn run_competition_round(
        a: &mut CompetitionPlayer,
        b: &mut CompetitionPlayer
    ){
        for _ in 0..3 {
            Self::run_one_competition_game_add_points(b, a)
        }
        for _ in 0..3 {
            Self::run_one_competition_game_add_points(a, b)
        }
    }

    fn run_one_competition_game_add_points(
        o: &mut CompetitionPlayer,
        x: &mut CompetitionPlayer
    ) {
        match GameEngine::new(&o.bot_factory, &x.bot_factory, false).run_game_get_result() {
            GameOver::Winner { player_who_won: PlayerId::O } => {
                o.wins += 1;
                x.loses += 1;
            },
            GameOver::Winner { player_who_won: PlayerId::X } => {
                o.loses += 1;
                x.wins += 1;
            },
            GameOver::Draw => {
                o.draws += 1;
                x.draws += 1;
            },
        }
    }
}


pub struct CompetitionPlayer{
    name: String,
    bot_factory: Box<dyn BotFactory>,
    wins: u16,
    loses: u16,
    draws: u16
}
impl CompetitionPlayer{
    pub fn new_player<B: Bot + 'static>() -> Self {
        Self {
            name: std::any::type_name::<B>().split_at("tron_coding_challenge::players::".len()).1.to_string(),
            bot_factory: BuildBot::<B>::new(),
            wins: 0,
            loses: 0,
            draws: 0,
        }
    }
    pub fn points(&self) -> f32 {
        self.wins as f32  - self.loses as f32 - (self.draws as f32 * 0.5f32)
    }
}
impl Display for CompetitionPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: Points: {}, Wins: {}, Loses: {}, Draws: {}", self.name, self.points(), self.wins, self.loses, self.draws)
    }
}