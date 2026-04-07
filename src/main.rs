use crate::engine::prelude::*;
use crate::players::*;
use competition::Competition;
use competition::CompetitionPlayer;

mod engine;
mod players;
mod competition;

fn main() {
    run_test_game_print::<example_bot::ExampleBot, human_controlled_bot::HumanControlledBot>();

    Competition::run_and_print(vec![
        CompetitionPlayer::new_player::<example_bot::ExampleBot>(),
        CompetitionPlayer::new_player::<bot_template::BotTemplate>(),
        CompetitionPlayer::new_player::<stardustz_bots::StardustzBot>(),
        CompetitionPlayer::new_player::<jack_papel_bots::hallucinator::Hallucinator>(),
        CompetitionPlayer::new_player::<jack_papel_bots::rip_and_tear::RipAndTear>(),
        CompetitionPlayer::new_player::<jack_papel_bots::freedom_eater::FreedomEater>(),
    ]);
}

fn run_test_game_print<O: Bot + 'static, X: Bot + 'static>(){
    GameEngine::new(
        &BuildBot::<O>::new(),
        &BuildBot::<X>::new(),
        true
    ).run_game_get_result_print();
}