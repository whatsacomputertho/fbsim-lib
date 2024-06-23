mod game {
    pub mod game;
    pub mod coinflip;
    pub mod log;
    pub mod play;
    pub mod context {
        pub mod clock;
        pub mod context;
        pub mod possession;
        pub mod score;
    }
}
mod team {
    pub mod team;
    pub mod coach;
    pub mod playcall;
    pub mod player;
    pub mod players;
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

use crate::team::coach::{Coach, OffensiveStyle, DefensiveStyle};
use crate::team::team::Team;
use crate::game::game::Game;

fn main() {
    // Initialize two coaches
    let my_coach: Coach = Coach::new(
        "Coach Stone",
        5, // Aggressiveness
        5, // Clock management
        5, // Intelligence
        OffensiveStyle::Run,
        DefensiveStyle::Balanced
    );
    let your_coach: Coach = Coach::new(
        "Coach Cosmo",
        10, // Aggressiveness
        1, // Clock management
        1, // Intelligence
        OffensiveStyle::Pass,
        DefensiveStyle::Blitz
    );

    // Initialize two teams
    let my_team: Team = Team::new(
        "The Ethans",
        "ETH",
        my_coach
    );
    let your_team: Team = Team::new(
        "The Kenzies",
        "KEH",
        your_coach
    );
    
    // Initialize a game
    let mut rng = rand::thread_rng();
    let mut our_game: Game = Game::new(my_team, your_team);
    our_game.simulate_opening_coin_fip(&mut rng);
    for log in our_game.log.log {
        println!("{}", log);
    }
}
