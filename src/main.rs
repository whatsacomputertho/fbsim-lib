mod game {
    pub mod game;
    pub mod coinflip;
    pub mod clock;
    pub mod log;
    pub mod possession;
    pub mod score;
    pub mod play;
}
mod team {
    pub mod team;
    pub mod player;
    pub mod units {
        pub mod defense {
            pub mod defense;
        }
        pub mod offense {
            pub mod offense;
        }
        pub mod specialteams {
            pub mod specialteams;
        }
    }
}

use crate::team::player::Player;
use crate::team::team::Team;
use crate::game::game::Game;

fn main() {
    // Initialize two teams
    let my_team: Team = Team::new(
        "The Ethans",
        "ETH"
    );
    let your_team: Team = Team::new(
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
