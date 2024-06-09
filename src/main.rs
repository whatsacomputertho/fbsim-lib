mod player;
mod team;
mod game;
mod coinflip;
mod clock;
mod log;
mod possession;
mod score;

use crate::player::Player;
use crate::team::Team;
use crate::game::Game;

fn main() {
    // Initialize two players
    let my_player: Player = Player::new(
        "Ethan Balcik",
        2, // Throwing
        3, // Catching
        4, // Running
        5, // Blocking
        6, // Tackling
        7  // Kicking
    );
    let your_player: Player = Player::new(
        "Eric Balcik",
        8, // Throwing
        7, // Catching
        6, // Running
        5, // Blocking
        4, // Tackling
        3, // Kicking
    );

    // Initialize two vecs of players
    let mut my_players: Vec<Player> = Vec::new();
    for _ in 0..10 {
        my_players.push(my_player.clone());
    }
    let mut your_players: Vec<Player> = Vec::new();
    for _ in 0..10 {
        your_players.push(your_player.clone());
    }

    // Initialize two teams
    let my_team: Team = Team::new(
        my_players,
        "The Ethans",
        "ETH"
    );
    let your_team: Team = Team::new(
        your_players,
        "The Kenzies",
        "KEH"
    );
    
    // Initialize a game
    let mut rng = rand::thread_rng();
    let mut our_game: Game = Game::new(my_team, your_team);
    our_game.simulate_opening_coin_fip(&mut rng);
    for log in our_game.log.log {
        println!("{}", log);
    }
}
