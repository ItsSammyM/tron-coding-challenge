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

        players.sort_by(|a,b|b.points.total_cmp(&a.points));
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
        match GameEngine::new(&o.bot_factory, &x.bot_factory, false).run_game() {
            GameOver::Winner { player_who_won: PlayerId::O } => {
                o.points += 1.0;
                x.points -= 1.0;
            },
            GameOver::Winner { player_who_won: PlayerId::X } => {
                o.points -= 1.0;
                x.points += 1.0;
            },
            GameOver::Draw => {
                o.points -= 0.5;
                x.points -= 0.5;
            },
        }
    }
}


pub struct CompetitionPlayer{
    name: String,
    points: f32,
    bot_factory: Box<dyn BotFactory>
}
impl CompetitionPlayer{
    pub fn new_player<B: Bot + 'static>() -> Self {
        Self {
            name: std::any::type_name::<B>().split_at("tron_coding_challenge::players::".len()).1.to_string(),
            bot_factory: BuildBot::<B>::new(),
            points: 0.0
        }
    }
}
impl Display for CompetitionPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.points)
    }
}