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
use crate::team::player::Player;
use crate::team::team::Team;
use crate::game::game::Game;

fn main() {
    // Initialize two coaches
    let my_coach: Coach = Coach::new(
        "Coach Ethan",
        5, // Aggressiveness
        5, // Clock management
        5, // Intelligence
        OffensiveStyle::Run,
        DefensiveStyle::Balanced
    );
    let your_coach: Coach = Coach::new(
        "Coach Eric",
        10, // Aggressiveness
        1, // Clock management
        1, // Intelligence
        OffensiveStyle::Pass,
        DefensiveStyle::Blitz
    );

    // Initialize two teams
    let mut my_team: Team = Team::new(
        "The Ethans",
        "ETH",
        my_coach
    );
    let mut your_team: Team = Team::new(
        "The Erics",
        "ERI",
        your_coach
    );

    // Initialize two players
    let my_player: Player = Player::new(
        "Ethan Ethan",
        5, // Throwing
        5, // Catching
        5, // Running
        5, // Blocking
        5, // Tackling
        7, // Kicking
    );
    let your_player: Player = Player::new(
        "Eric Eric",
        5, // Throwing
        5, // Catching
        5, // Running
        5, // Blocking
        5, // Tackling
        10, // Kicking
    );

    // Initialize special teams units for both teams
    for i in 0..3 {
        my_team.special_teams.kickers.push(my_player.clone());
        your_team.special_teams.kickers.push(your_player.clone());
    }
    for i in 0..3 {
        my_team.special_teams.kick_returners.push(my_player.clone());
        your_team.special_teams.kick_returners.push(your_player.clone());
    }
    for i in 0..3 {
        my_team.special_teams.punters.push(my_player.clone());
        your_team.special_teams.punters.push(your_player.clone());
    }
    for i in 0..3 {
        my_team.special_teams.punt_returners.push(my_player.clone());
        your_team.special_teams.punt_returners.push(your_player.clone());
    }
    for i in 0..3 {
        my_team.special_teams.linemen.push(my_player.clone());
        your_team.special_teams.linemen.push(your_player.clone());
    }
    for i in 0..3 {
        my_team.special_teams.defenders.push(my_player.clone());
        your_team.special_teams.defenders.push(your_player.clone());
    }
    for i in 0..3 {
        my_team.special_teams.extras.push(my_player.clone());
        your_team.special_teams.extras.push(your_player.clone());
    }
    
    // Initialize a game
    let mut rng = rand::thread_rng();
    let mut our_game: Game = Game::new(my_team, your_team);
    our_game.simulate_opening_coin_fip(&mut rng);
    our_game.simulate_next_play(&mut rng);
    for log in our_game.log.log {
        println!("{}", log);
    }
}
